//! System-wide audio routing through a virtual "FXSound" output device.
//!
//! ## Why a virtual device
//!
//! Up to 1.1.4 FXSound recorded the default sink's monitor and played the
//! processed copy back into that same sink. Applications kept playing to the
//! speakers directly, so the processed copy arrived on top of the original —
//! and FXSound then recorded its own output again, every pass louder: a
//! feedback loop (#511).
//!
//! Now FXSound creates a null sink called "FXSound" and makes it the system
//! default. Applications play into it; FXSound records its monitor, processes
//! the audio and plays the result on the real output device:
//!
//! ```text
//!   apps ──▶ FXSound sink ──monitor──▶ AudioEngine ──▶ speakers / headphones
//! ```
//!
//! Applications never reach the hardware directly and FXSound never hears its
//! own output. Both streams are opened with `DONT_MOVE`, so the server can
//! never re-route FXSound's output into its own sink (which would rebuild the
//! loop) — if the output device disappears, FXSound picks another itself.
//!
//! ## Threading
//!
//! One thread owns everything PulseAudio: the main loop, the context and both
//! streams. Every wait is bounded (`Mainloop::prepare` with a timeout), so a
//! slow or wedged audio server cannot hang anything. The UI never talks to
//! the server; it reads a cached [`RoutingStatus`] and sends commands. The old
//! device query ran on the GTK main thread and waited on a condition that
//! nothing ever signalled, freezing the window at launch (#510).

use crate::audio::{AudioEngine, CHANNELS, SAMPLE_RATE};
use libpulse_binding as pulse;
use pulse::callbacks::ListResult;
use pulse::context::subscribe::{Facility, InterestMaskSet};
use pulse::context::{Context, FlagSet as ContextFlags, State as ContextState};
use pulse::def::BufferAttr;
use pulse::mainloop::standard::Mainloop;
use pulse::proplist::Proplist;
use pulse::sample::{Format, Spec};
use pulse::stream::{FlagSet as StreamFlags, PeekResult, SeekMode, State as StreamState, Stream};
use pulse::time::MicroSeconds;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Node name of the virtual sink.
pub const VIRTUAL_SINK: &str = "fxsound_sink";
/// What desktop sound settings show for it.
const VIRTUAL_SINK_DESCRIPTION: &str = "FXSound";
/// Application name on the PulseAudio connection and both streams.
const APP_NAME: &str = "FXSound";

/// How long to wait for the server to accept the connection.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
/// How long to wait for any single request (sink list, module load, ...).
const REQUEST_TIMEOUT: Duration = Duration::from_secs(3);
/// Longest a single main-loop iteration may block.
const POLL_INTERVAL: Duration = Duration::from_millis(20);

/// Capture fragment and playback buffer target. Together with the server's
/// own buffering this keeps the added latency around 50 ms.
const CAPTURE_FRAGMENT_MS: u32 = 10;
const PLAYBACK_TARGET_MS: u32 = 40;
/// If FXSound's own playback queue grows past this (after a stall, or as two
/// devices' clocks drift apart), skip ahead to the target instead of lagging
/// further.
const QUEUE_LIMIT_MS: u64 = 150;
/// How often to check the queue, and how long to leave a freshly opened
/// output alone. A device that is just waking up can swallow a large first
/// block and leave a backlog of half a second or more behind it, which then
/// drains on its own within a few seconds.
const QUEUE_CHECK_INTERVAL: Duration = Duration::from_millis(500);
const QUEUE_SETTLE_TIME: Duration = Duration::from_secs(5);
/// Consecutive over-limit, non-draining measurements before skipping ahead.
const QUEUE_STRIKES: u8 = 3;

/// At most this many times per `REASSERT_WINDOW` will FXSound put its device
/// back as the default after something else changed it. Prevents an endless
/// tug-of-war with another tool that also insists on being the default.
const MAX_REASSERTS: usize = 3;
const REASSERT_WINDOW: Duration = Duration::from_secs(30);

/// An output device FXSound can play to.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct AudioSink {
    /// Stable server-side name; what the playback stream is opened against.
    pub name: String,
    /// Human-readable name shown in the UI.
    pub description: String,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceState {
    /// Connecting to the audio server and setting up the FXSound device.
    Starting,
    /// Audio is flowing through FXSound.
    Running,
    /// Not processing; `message` says why. Retries automatically.
    Error,
}

/// Snapshot of the routing, as shown in the UI.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct RoutingStatus {
    pub state: ServiceState,
    pub message: Option<String>,
    /// Devices FXSound can play to. Never includes FXSound's own sink.
    pub devices: Vec<AudioSink>,
    /// The device FXSound is currently playing to.
    pub output: Option<String>,
    /// The device the user asked for, if any; remembered across restarts.
    pub preferred: Option<String>,
}

impl RoutingStatus {
    fn starting(preferred: Option<String>) -> Self {
        Self {
            state: ServiceState::Starting,
            message: None,
            devices: Vec::new(),
            output: None,
            preferred,
        }
    }
}

enum Command {
    /// Play to this device, or let FXSound choose when `None`.
    SetOutput(Option<String>),
    /// Give the system its normal routing back, then stop.
    Shutdown(Sender<()>),
}

/// Handle to the audio routing thread.
pub struct AudioService {
    commands: Sender<Command>,
    status: Arc<Mutex<RoutingStatus>>,
    thread: Mutex<Option<std::thread::JoinHandle<()>>>,
}

impl AudioService {
    /// Start routing system audio through `engine`.
    ///
    /// `on_change` runs on the audio thread whenever the status changes.
    pub fn start(
        engine: Arc<Mutex<AudioEngine>>,
        preferred_output: Option<String>,
        on_change: impl Fn(&RoutingStatus) + Send + 'static,
    ) -> Self {
        let (commands, rx) = mpsc::channel();
        let status = Arc::new(Mutex::new(RoutingStatus::starting(
            preferred_output.clone(),
        )));
        let publisher = Publisher {
            status: Arc::clone(&status),
            on_change: Box::new(on_change),
        };
        let thread = std::thread::Builder::new()
            .name("fxsound-audio".into())
            .spawn(move || service_thread(rx, engine, publisher, preferred_output))
            .expect("failed to spawn the audio thread");
        Self {
            commands,
            status,
            thread: Mutex::new(Some(thread)),
        }
    }

    pub fn status(&self) -> RoutingStatus {
        self.status
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }

    pub fn set_output(&self, sink: Option<String>) {
        let _ = self.commands.send(Command::SetOutput(sink));
    }

    /// Restore the system's own routing and stop. Waits at most `timeout`;
    /// safe to call more than once.
    pub fn shutdown(&self, timeout: Duration) {
        let Some(thread) = self.thread.lock().unwrap_or_else(|e| e.into_inner()).take() else {
            return;
        };
        let (ack_tx, ack_rx) = mpsc::channel();
        if self.commands.send(Command::Shutdown(ack_tx)).is_ok()
            && ack_rx.recv_timeout(timeout).is_ok()
        {
            let _ = thread.join();
        } else {
            log::warn!("Audio thread did not stop in time; leaving it behind");
        }
    }
}

/// Publishes status changes to the UI.
struct Publisher {
    status: Arc<Mutex<RoutingStatus>>,
    on_change: Box<dyn Fn(&RoutingStatus) + Send>,
}

impl Publisher {
    fn update(&self, change: impl FnOnce(&mut RoutingStatus)) {
        let snapshot = {
            let mut status = self.status.lock().unwrap_or_else(|e| e.into_inner());
            let mut next = status.clone();
            change(&mut next);
            if next == *status {
                return;
            }
            *status = next.clone();
            next
        };
        (self.on_change)(&snapshot);
    }

    fn error(&self, reason: &str) {
        log::warn!("Audio routing unavailable: {reason}");
        let message = user_message(reason).to_string();
        self.update(|s| {
            s.state = ServiceState::Error;
            s.message = Some(message);
        });
    }
}

/// Why a running session ended.
enum End {
    Shutdown(Option<Sender<()>>),
    Lost(String),
}

fn service_thread(
    rx: Receiver<Command>,
    engine: Arc<Mutex<AudioEngine>>,
    publisher: Publisher,
    mut preferred: Option<String>,
) {
    let mut retry = Duration::from_millis(500);
    loop {
        // A panic anywhere in a session (the binding panics on requests made
        // in a bad state) must not take audio down for good: treat it like a
        // lost connection and start over.
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut session = match Session::open(Arc::clone(&engine), preferred.clone()) {
                Ok(session) => session,
                Err(e) => return Err(e),
            };
            publisher.update(|s| session.describe(s));
            log::info!("Routing system audio through FXSound to {}", session.output);
            let end = session.run(&rx, &publisher);
            preferred = session.preferred.clone();
            session.close();
            Ok(end)
        }));

        match outcome {
            Ok(Ok(End::Shutdown(ack))) => {
                if let Some(ack) = ack {
                    let _ = ack.send(());
                }
                return;
            }
            Ok(Ok(End::Lost(reason))) => {
                publisher.error(&reason);
                retry = Duration::from_millis(500);
            }
            Ok(Err(reason)) => publisher.error(&reason),
            Err(_) => publisher.error("the audio thread hit an internal error"),
        }

        // Wait before retrying, still answering commands.
        match rx.recv_timeout(retry) {
            Ok(Command::Shutdown(ack)) => {
                let _ = ack.send(());
                return;
            }
            Ok(Command::SetOutput(sink)) => preferred = sink,
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => return,
        }
        retry = (retry * 2).min(Duration::from_secs(10));
    }
}

/// What the subscription callback saw since it was last checked.
#[derive(Default)]
struct Pending {
    sinks: bool,
    server: bool,
}

#[derive(Clone, Debug)]
struct SinkEntry {
    name: String,
    description: String,
    owner_module: Option<u32>,
    monitor_source: u32,
    monitor_name: Option<String>,
    /// PipeWire's `priority.session`; 0 when the server doesn't say.
    priority: i64,
}

struct ServerSnapshot {
    name: String,
    default_sink: Option<String>,
}

/// One connection to the audio server with FXSound's device set up on it.
struct Session {
    // Field order is drop order, and here it matters: streams before their
    // context, the context before its main loop. Freeing the main loop first
    // leaves a context that is still connecting holding dead timer events,
    // and libpulse aborts the whole process when the context is then freed
    // ("Assertion '!e->dead' failed") — which is what happened when the audio
    // server stopped answering mid-connect.
    capture: Option<Stream>,
    playback: Option<Stream>,
    context: Context,
    mainloop: Mainloop,

    engine: Arc<Mutex<AudioEngine>>,
    pending: Rc<RefCell<Pending>>,
    is_pipewire: bool,

    /// Index of the module providing the FXSound sink, once loaded.
    module: Option<u32>,

    /// Hardware device being played to.
    output: String,
    /// Device the user chose, in FXSound or in the system sound settings.
    preferred: Option<String>,
    /// The system default before FXSound took over.
    original_default: Option<String>,
    devices: Vec<AudioSink>,
    reasserts: Vec<Instant>,
    /// Earliest time to retry an output device that just failed to open.
    output_retry_at: Option<Instant>,

    /// Bytes of upcoming output to skip to bring the queue back down.
    skip_bytes: usize,
    /// When to next measure the playback queue, whether a measurement is in
    /// flight, whether its answer has arrived, and how many measurements in a
    /// row found the queue too long.
    next_queue_check: Instant,
    queue_requested: bool,
    queue_answered: Rc<Cell<bool>>,
    queue_strikes: u8,
    queue_last_ms: u64,

    samples_in: Vec<f32>,
    samples_out: Vec<f32>,
    bytes_out: Vec<u8>,
}

fn bytes_for_ms(ms: u32) -> u32 {
    SAMPLE_RATE / 1000 * ms * CHANNELS as u32 * 4
}

fn sample_spec() -> Spec {
    Spec {
        format: Format::F32le,
        channels: CHANNELS,
        rate: SAMPLE_RATE,
    }
}

impl Session {
    /// Connect, set up the FXSound device and start the streams.
    fn open(engine: Arc<Mutex<AudioEngine>>, preferred: Option<String>) -> Result<Self, String> {
        let mainloop = Mainloop::new().ok_or("could not create a PulseAudio main loop")?;
        let mut props = Proplist::new().ok_or("could not create a property list")?;
        let _ = props.set_str("application.name", APP_NAME);
        let _ = props.set_str("application.id", "com.fxsound.linux");
        let _ = props.set_str("application.icon_name", "com.fxsound.linux");
        let context = Context::new_with_proplist(&mainloop, APP_NAME, &props)
            .ok_or("could not create a PulseAudio context")?;

        let mut session = Session {
            mainloop,
            context,
            engine,
            pending: Rc::new(RefCell::new(Pending::default())),
            is_pipewire: false,
            module: None,
            capture: None,
            playback: None,
            output: String::new(),
            preferred,
            original_default: None,
            devices: Vec::new(),
            reasserts: Vec::new(),
            output_retry_at: None,
            skip_bytes: 0,
            next_queue_check: Instant::now() + QUEUE_SETTLE_TIME,
            queue_requested: false,
            queue_answered: Rc::new(Cell::new(false)),
            queue_strikes: 0,
            queue_last_ms: 0,
            samples_in: Vec::new(),
            samples_out: Vec::new(),
            bytes_out: Vec::new(),
        };

        session
            .context
            .connect(None, ContextFlags::NOAUTOSPAWN, None)
            .map_err(|e| format!("could not reach the audio server ({e})"))?;
        session.wait_until("the audio server", CONNECT_TIMEOUT, |s| {
            match s.context.get_state() {
                ContextState::Ready => Some(Ok(())),
                ContextState::Failed | ContextState::Terminated => Some(Err(format!(
                    "no PulseAudio or PipeWire server is running ({})",
                    s.context.errno()
                ))),
                _ => None,
            }
        })?;

        if let Err(e) = session.set_up() {
            session.close();
            return Err(e);
        }
        Ok(session)
    }

    fn set_up(&mut self) -> Result<(), String> {
        let server = self.server_info()?;
        self.is_pipewire = server.name.to_ascii_lowercase().contains("pipewire");
        log::info!("Connected to {}", server.name);

        self.remove_leftovers()?;

        let server = self.server_info()?;
        let sinks = self.sinks()?;
        self.original_default = server
            .default_sink
            .filter(|name| is_output_candidate(name) && sinks.iter().any(|s| &s.name == name));
        self.output = choose_output(
            &sinks,
            self.preferred.as_deref(),
            None,
            self.original_default.as_deref(),
        )
        .ok_or("no audio output device was found")?;

        self.create_virtual_sink()?;
        let monitor = self
            .sinks()?
            .into_iter()
            .find(|s| s.name == VIRTUAL_SINK)
            .and_then(|s| s.monitor_name)
            .unwrap_or_else(|| format!("{VIRTUAL_SINK}.monitor"));

        let output = self.output.clone();
        self.playback = Some(self.connect_playback(&output)?);
        self.capture = Some(self.connect_capture(&monitor)?);

        // Only now, with audio flowing into a working pipeline, take over as
        // the default so applications move across without a gap.
        self.set_default_sink(VIRTUAL_SINK)?;
        self.subscribe();
        self.refresh_devices()?;
        Ok(())
    }

    // ── Main loop plumbing ──

    /// Run one main-loop iteration (blocking at most `timeout`) and move any
    /// captured audio through the engine.
    fn iterate(&mut self, timeout: Duration) -> Result<(), String> {
        let micros = MicroSeconds(timeout.as_micros() as u64);
        self.mainloop
            .prepare(Some(micros))
            .and_then(|_| self.mainloop.poll())
            .and_then(|_| self.mainloop.dispatch())
            .map_err(|e| format!("audio server main loop failed ({e})"))?;
        self.pump();
        self.ensure_connected()
    }

    fn ensure_connected(&self) -> Result<(), String> {
        match self.context.get_state() {
            ContextState::Failed | ContextState::Terminated => {
                Err("lost the connection to the audio server".into())
            }
            _ => Ok(()),
        }
    }

    /// Iterate until `check` returns a result, failing after `timeout`.
    fn wait_until<T>(
        &mut self,
        what: &str,
        timeout: Duration,
        mut check: impl FnMut(&mut Self) -> Option<Result<T, String>>,
    ) -> Result<T, String> {
        let deadline = Instant::now() + timeout;
        loop {
            if let Some(result) = check(self) {
                return result;
            }
            if Instant::now() >= deadline {
                return Err(format!("timed out waiting for {what}"));
            }
            self.iterate(Duration::from_millis(10))?;
        }
    }

    /// Wait for a request's result to land in `slot`.
    fn wait_for<T>(&mut self, what: &str, slot: &Rc<RefCell<Option<T>>>) -> Result<T, String> {
        self.wait_until(what, REQUEST_TIMEOUT, |_| slot.borrow_mut().take().map(Ok))
    }

    // ── Requests ──

    fn server_info(&mut self) -> Result<ServerSnapshot, String> {
        self.ensure_ready()?;
        let slot = Rc::new(RefCell::new(None));
        let fill = Rc::clone(&slot);
        let _op = self.context.introspect().get_server_info(move |info| {
            *fill.borrow_mut() = Some(ServerSnapshot {
                name: info
                    .server_name
                    .as_deref()
                    .unwrap_or("unknown server")
                    .to_string(),
                default_sink: info.default_sink_name.as_ref().map(|s| s.to_string()),
            });
        });
        self.wait_for("server information", &slot)
    }

    fn sinks(&mut self) -> Result<Vec<SinkEntry>, String> {
        self.ensure_ready()?;
        let list = Rc::new(RefCell::new(Vec::new()));
        let slot = Rc::new(RefCell::new(None));
        let (items, done) = (Rc::clone(&list), Rc::clone(&slot));
        let _op = self
            .context
            .introspect()
            .get_sink_info_list(move |result| match result {
                ListResult::Item(info) => {
                    let Some(name) = info.name.as_ref().map(|n| n.to_string()) else {
                        return; // can't open a stream against a nameless sink
                    };
                    let description = info
                        .description
                        .as_ref()
                        .map(|d| d.to_string())
                        .unwrap_or_else(|| name.clone());
                    let priority = info
                        .proplist
                        .get_str("priority.session")
                        .and_then(|p| p.parse().ok())
                        .unwrap_or(0);
                    items.borrow_mut().push(SinkEntry {
                        name,
                        description,
                        owner_module: info.owner_module,
                        monitor_source: info.monitor_source,
                        monitor_name: info.monitor_source_name.as_ref().map(|m| m.to_string()),
                        priority,
                    });
                }
                ListResult::End => *done.borrow_mut() = Some(Ok(())),
                ListResult::Error => *done.borrow_mut() = Some(Err(())),
            });
        self.wait_for("the list of output devices", &slot)?
            .map_err(|_| "the audio server refused to list output devices".to_string())?;
        let sinks = list.borrow().clone();
        Ok(sinks)
    }

    fn set_default_sink(&mut self, name: &str) -> Result<(), String> {
        self.ensure_ready()?;
        let slot = Rc::new(RefCell::new(None));
        let fill = Rc::clone(&slot);
        let _op = self
            .context
            .set_default_sink(name, move |ok| *fill.borrow_mut() = Some(ok));
        if self.wait_for("the default output to change", &slot)? {
            Ok(())
        } else {
            Err(format!(
                "the audio server would not make {name} the default output"
            ))
        }
    }

    fn unload_module(&mut self, index: u32) -> Result<(), String> {
        self.ensure_ready()?;
        let slot = Rc::new(RefCell::new(None));
        let fill = Rc::clone(&slot);
        let _op = self
            .context
            .introspect()
            .unload_module(index, move |ok| *fill.borrow_mut() = Some(ok));
        match self.wait_for("a module to unload", &slot)? {
            true => Ok(()),
            false => Err(format!("could not unload module #{index}")),
        }
    }

    fn ensure_ready(&self) -> Result<(), String> {
        // The binding panics if a request is made on a context that is not
        // ready, so check first.
        if self.context.get_state() == ContextState::Ready {
            Ok(())
        } else {
            Err("not connected to the audio server".into())
        }
    }

    // ── Setting up and tearing down the FXSound device ──

    fn create_virtual_sink(&mut self) -> Result<(), String> {
        self.ensure_ready()?;
        // float32 so a hot mix of several applications can exceed 0 dBFS
        // inside the sink without clipping before the limiter sees it.
        let args = format!(
            "sink_name={VIRTUAL_SINK} \
             sink_properties=device.description={VIRTUAL_SINK_DESCRIPTION} \
             format=float32le rate={SAMPLE_RATE} channels={CHANNELS} \
             channel_map=front-left,front-right"
        );
        let slot = Rc::new(RefCell::new(None));
        let fill = Rc::clone(&slot);
        let _op = self
            .context
            .introspect()
            .load_module("module-null-sink", &args, move |index| {
                *fill.borrow_mut() = Some(index)
            });
        let index = self.wait_for("the FXSound device to be created", &slot)?;
        if index == u32::MAX {
            return Err(
                "the audio server refused to create the FXSound device (module-null-sink)".into(),
            );
        }
        self.module = Some(index);
        Ok(())
    }

    /// Remove an FXSound device left behind by a previous run that did not
    /// shut down cleanly — but not one that a running FXSound is using.
    fn remove_leftovers(&mut self) -> Result<(), String> {
        let sinks = self.sinks()?;
        let Some(old) = sinks.into_iter().find(|s| s.name == VIRTUAL_SINK) else {
            return Ok(());
        };

        if self.someone_records(old.monitor_source)? {
            return Err(
                "FXSound is already running (another copy is using the FXSound device)".into(),
            );
        }

        let mut stale: Vec<u32> = old.owner_module.into_iter().collect();
        // Early development builds wired a loopback into a sink of the same
        // name with scripts/setup-audio.sh; clear any such loopback too, or it
        // would end up looping a monitor straight back into the speakers.
        for (index, name, argument) in self.modules()? {
            if name.contains("loopback") && argument.contains(VIRTUAL_SINK) {
                stale.push(index);
            }
        }
        for index in stale {
            log::info!("Removing leftover FXSound module #{index}");
            self.unload_module(index)?;
        }
        Ok(())
    }

    /// Whether a live FXSound process (other than this one) records `source`.
    fn someone_records(&mut self, source: u32) -> Result<bool, String> {
        self.ensure_ready()?;
        let found = Rc::new(RefCell::new(false));
        let slot = Rc::new(RefCell::new(None));
        let (hit, done) = (Rc::clone(&found), Rc::clone(&slot));
        let own_pid = std::process::id().to_string();
        let _op =
            self.context
                .introspect()
                .get_source_output_info_list(move |result| match result {
                    ListResult::Item(info) if info.source == source => {
                        let app = info
                            .proplist
                            .get_str("application.name")
                            .unwrap_or_default();
                        let pid = info.proplist.get_str("application.process.id");
                        if app == APP_NAME && pid.as_deref() != Some(own_pid.as_str()) {
                            let alive = pid
                                .and_then(|p| p.parse::<i32>().ok())
                                .is_none_or(process_alive);
                            *hit.borrow_mut() |= alive;
                        }
                    }
                    ListResult::Item(_) => {}
                    ListResult::End | ListResult::Error => *done.borrow_mut() = Some(()),
                });
        self.wait_for("the list of recording streams", &slot)?;
        let found = *found.borrow();
        Ok(found)
    }

    fn modules(&mut self) -> Result<Vec<(u32, String, String)>, String> {
        self.ensure_ready()?;
        let list = Rc::new(RefCell::new(Vec::new()));
        let slot = Rc::new(RefCell::new(None));
        let (items, done) = (Rc::clone(&list), Rc::clone(&slot));
        let _op = self
            .context
            .introspect()
            .get_module_info_list(move |result| match result {
                ListResult::Item(info) => items.borrow_mut().push((
                    info.index,
                    info.name.as_deref().unwrap_or_default().to_string(),
                    info.argument.as_deref().unwrap_or_default().to_string(),
                )),
                ListResult::End | ListResult::Error => *done.borrow_mut() = Some(()),
            });
        self.wait_for("the list of modules", &slot)?;
        let modules = list.borrow().clone();
        Ok(modules)
    }

    /// Put the system back the way it was: the hardware device as default,
    /// FXSound's device gone. Best effort — the server may already be gone.
    fn close(&mut self) {
        // Stop the streams first. A short silence while applications move is
        // better than a moment of them playing on top of FXSound's buffered
        // output.
        for stream in [self.capture.take(), self.playback.take()]
            .into_iter()
            .flatten()
        {
            let mut stream = stream;
            let _ = stream.disconnect();
        }

        if self.context.get_state() == ContextState::Ready {
            let target = Some(self.output.clone())
                .filter(|o| !o.is_empty())
                .or_else(|| self.original_default.clone());
            if let Some(target) = target {
                let is_default = self
                    .server_info()
                    .map(|s| s.default_sink.as_deref() == Some(VIRTUAL_SINK))
                    .unwrap_or(true);
                if is_default {
                    if let Err(e) = self.set_default_sink(&target) {
                        log::warn!("Could not restore the default output: {e}");
                    }
                }
            }
            if let Some(module) = self.module.take() {
                if let Err(e) = self.unload_module(module) {
                    log::warn!("Could not remove the FXSound device: {e}");
                }
            }
            // Let the server process the requests before hanging up.
            let _ = self.iterate(Duration::from_millis(10));
        }
        self.context.disconnect();
    }

    // ── Streams ──

    fn new_stream(&mut self, name: &str) -> Result<Stream, String> {
        let mut props = Proplist::new().ok_or("could not create a property list")?;
        let _ = props.set_str("media.name", name);
        let _ = props.set_str("application.name", APP_NAME);
        if self.is_pipewire {
            // Schedule capture and playback in one PipeWire graph, driven by
            // the output device's clock. Otherwise the FXSound sink runs on
            // the system timer and the two clocks slowly drift apart.
            let _ = props.set_str("node.group", "fxsound-linux");
        }
        Stream::new_with_proplist(&mut self.context, name, &sample_spec(), None, &mut props)
            .ok_or_else(|| format!("could not create the {name} stream"))
    }

    fn connect_playback(&mut self, sink: &str) -> Result<Stream, String> {
        // Never let FXSound play into its own device: that is the loop.
        if sink == VIRTUAL_SINK {
            return Err("refusing to play into the FXSound device itself".into());
        }
        self.ensure_ready()?;
        let mut stream = self.new_stream("FXSound Output")?;
        let attr = BufferAttr {
            maxlength: u32::MAX,
            tlength: bytes_for_ms(PLAYBACK_TARGET_MS),
            prebuf: u32::MAX,
            minreq: u32::MAX,
            fragsize: u32::MAX,
        };
        let flags = StreamFlags::ADJUST_LATENCY
            | StreamFlags::DONT_MOVE
            | StreamFlags::INTERPOLATE_TIMING
            | StreamFlags::AUTO_TIMING_UPDATE;
        stream
            .connect_playback(Some(sink), Some(&attr), flags, None, None)
            .map_err(|e| format!("could not play to {sink} ({e})"))?;
        self.wait_ready(stream, sink)
    }

    fn connect_capture(&mut self, monitor: &str) -> Result<Stream, String> {
        self.ensure_ready()?;
        let mut stream = self.new_stream("FXSound Capture")?;
        let attr = BufferAttr {
            maxlength: u32::MAX,
            tlength: u32::MAX,
            prebuf: u32::MAX,
            minreq: u32::MAX,
            fragsize: bytes_for_ms(CAPTURE_FRAGMENT_MS),
        };
        stream
            .connect_record(
                Some(monitor),
                Some(&attr),
                StreamFlags::ADJUST_LATENCY | StreamFlags::DONT_MOVE,
            )
            .map_err(|e| format!("could not record {monitor} ({e})"))?;
        self.wait_ready(stream, monitor)
    }

    fn wait_ready(&mut self, stream: Stream, device: &str) -> Result<Stream, String> {
        self.wait_until(
            &format!("the stream on {device}"),
            REQUEST_TIMEOUT,
            |_| match stream.get_state() {
                StreamState::Ready => Some(Ok(())),
                StreamState::Failed | StreamState::Terminated => Some(Err(format!(
                    "the audio server closed the stream on {device}"
                ))),
                _ => None,
            },
        )?;
        Ok(stream)
    }

    /// Move everything captured so far through the engine to the output.
    fn pump(&mut self) {
        let Session {
            capture,
            playback,
            engine,
            samples_in,
            samples_out,
            bytes_out,
            skip_bytes,
            next_queue_check,
            queue_requested,
            queue_answered,
            queue_strikes,
            queue_last_ms,
            ..
        } = self;
        let Some(capture) = capture
            .as_mut()
            .filter(|c| c.get_state() == StreamState::Ready)
        else {
            return;
        };
        let Some(playback) = playback
            .as_mut()
            .filter(|p| p.get_state() == StreamState::Ready)
        else {
            // No working output right now: drop what arrives instead of
            // letting it pile up and play late once a device is back.
            while let Ok(PeekResult::Data(_) | PeekResult::Hole(_)) = capture.peek() {
                let _ = capture.discard();
            }
            return;
        };

        loop {
            match capture.peek() {
                Ok(PeekResult::Data(data)) => {
                    samples_in.clear();
                    samples_in.extend(
                        data.chunks_exact(4)
                            .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]])),
                    );
                }
                Ok(PeekResult::Hole(_)) => {
                    let _ = capture.discard();
                    continue;
                }
                Ok(PeekResult::Empty) => break,
                Err(e) => {
                    log::warn!("Reading captured audio failed: {e}");
                    break;
                }
            }
            let _ = capture.discard();

            samples_out.resize(samples_in.len(), 0.0);
            engine
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .process_audio(samples_in, samples_out);

            bytes_out.clear();
            bytes_out.extend(samples_out.iter().flat_map(|s| s.to_le_bytes()));

            // Catch up after a stall instead of carrying the delay forever.
            let mut chunk = &bytes_out[..];
            if *skip_bytes > 0 {
                let skip = (*skip_bytes).min(chunk.len()) / 8 * 8;
                *skip_bytes -= skip.min(*skip_bytes);
                chunk = &chunk[skip..];
            }
            if !chunk.is_empty() {
                if let Err(e) = playback.write_copy(chunk, 0, SeekMode::Relative) {
                    log::warn!("Writing processed audio failed: {e}");
                    break;
                }
            }
        }

        // Measure FXSound's own playback queue with a fresh timing update. The
        // total latency is no use here: it includes the device's latency,
        // which other programs can raise and which skipping input can never
        // shrink. Interpolated figures are no better — between automatic
        // updates (up to 1.5 s apart) the read position goes stale and the
        // queue looks far longer than it is.
        if queue_answered.replace(false) {
            *queue_requested = false;
            if let Some(t) = playback.get_timing_info() {
                if t.write_index_corrupt == 0 && t.read_index_corrupt == 0 {
                    let queued = (t.write_index - t.read_index).max(0) as u64;
                    let queued_ms = queued * 1000 / u64::from(bytes_for_ms(1000));
                    // A real backlog holds steady or grows; one that is
                    // shrinking is a device catching up, and skipping would
                    // throw away audio it was about to play.
                    let draining = queued_ms + 20 < *queue_last_ms;
                    *queue_last_ms = queued_ms;
                    if queued_ms > QUEUE_LIMIT_MS && !draining {
                        *queue_strikes += 1;
                    } else {
                        *queue_strikes = 0;
                    }
                    if *queue_strikes >= QUEUE_STRIKES {
                        *queue_strikes = 0;
                        let excess_ms = queued_ms - u64::from(PLAYBACK_TARGET_MS);
                        *skip_bytes = bytes_for_ms(excess_ms as u32) as usize;
                        log::info!(
                            "Playback queue at {queued_ms} ms; skipping ahead {excess_ms} ms"
                        );
                    }
                }
            }
        }
        if !*queue_requested && Instant::now() >= *next_queue_check {
            *next_queue_check = Instant::now() + QUEUE_CHECK_INTERVAL;
            *queue_requested = true;
            let answered = Rc::clone(queue_answered);
            let _op = playback.update_timing_info(Some(Box::new(move |_| answered.set(true))));
        }
    }

    // ── Reacting to changes ──

    fn subscribe(&mut self) {
        let pending = Rc::clone(&self.pending);
        self.context
            .set_subscribe_callback(Some(Box::new(move |facility, _, _| {
                let mut pending = pending.borrow_mut();
                match facility {
                    Some(Facility::Sink) | Some(Facility::Module) => pending.sinks = true,
                    Some(Facility::Server) => pending.server = true,
                    _ => {}
                }
            })));
        let _op = self.context.subscribe(
            InterestMaskSet::SINK | InterestMaskSet::SERVER | InterestMaskSet::MODULE,
            |_| {},
        );
    }

    fn run(&mut self, rx: &Receiver<Command>, publisher: &Publisher) -> End {
        loop {
            if let Err(e) = self.iterate(POLL_INTERVAL) {
                return End::Lost(e);
            }

            loop {
                match rx.try_recv() {
                    Ok(Command::Shutdown(ack)) => return End::Shutdown(Some(ack)),
                    Ok(Command::SetOutput(sink)) => {
                        log::info!(
                            "Output device requested: {}",
                            sink.as_deref().unwrap_or("automatic")
                        );
                        self.preferred = sink;
                        if let Err(e) = self.reconcile() {
                            return End::Lost(e);
                        }
                        publisher.update(|s| self.describe(s));
                    }
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => return End::Shutdown(None),
                }
            }

            let capture_alive = self
                .capture
                .as_ref()
                .is_some_and(|s| s.get_state() == StreamState::Ready);
            if !capture_alive {
                return End::Lost("the FXSound device was removed".into());
            }

            let playback_alive = self
                .playback
                .as_ref()
                .is_some_and(|s| s.get_state() == StreamState::Ready);
            let changed = {
                let mut pending = self.pending.borrow_mut();
                let changed = pending.sinks || pending.server;
                *pending = Pending::default();
                changed
            };
            let retry_due = self.output_retry_at.is_none_or(|at| Instant::now() >= at);
            if changed || (!playback_alive && retry_due) {
                if let Err(e) = self.reconcile() {
                    return End::Lost(e);
                }
                publisher.update(|s| self.describe(s));
            }
        }
    }

    /// Bring the routing in line with the current devices and default.
    fn reconcile(&mut self) -> Result<(), String> {
        let server = self.server_info()?;
        let sinks = self.sinks()?;
        if !sinks.iter().any(|s| s.name == VIRTUAL_SINK) {
            return Err("the FXSound device was removed".into());
        }

        match server.default_sink.as_deref() {
            Some(VIRTUAL_SINK) => {}
            Some(other) if sinks.iter().any(|s| s.name == other) => {
                // Someone chose a real device as the default — most likely the
                // user in the system sound settings, or a headset that just
                // connected. Follow them: play to that device, and put FXSound
                // back in front of it so it stays enhanced.
                log::info!("System default changed to {other}; following it");
                self.preferred = Some(other.to_string());
                self.reassert_default();
            }
            _ => self.reassert_default(),
        }

        let current_ok = self
            .playback
            .as_ref()
            .is_some_and(|s| s.get_state() == StreamState::Ready);
        let want = choose_output(
            &sinks,
            self.preferred.as_deref(),
            current_ok.then_some(self.output.as_str()),
            self.original_default.as_deref(),
        );
        match want {
            Some(want) if want != self.output || !current_ok => self.switch_output(&want),
            Some(_) => {}
            None => log::warn!("No output device left to play to"),
        }

        self.devices = hardware_sinks(&sinks);
        Ok(())
    }

    fn reassert_default(&mut self) {
        let now = Instant::now();
        self.reasserts
            .retain(|t| now.duration_since(*t) < REASSERT_WINDOW);
        if self.reasserts.len() >= MAX_REASSERTS {
            log::warn!("Another program keeps changing the default output; leaving it alone");
            return;
        }
        self.reasserts.push(now);
        if let Err(e) = self.set_default_sink(VIRTUAL_SINK) {
            log::warn!("Could not make FXSound the default output again: {e}");
        }
    }

    fn switch_output(&mut self, sink: &str) {
        if self.output_retry_at.is_some_and(|at| Instant::now() < at) {
            return;
        }
        match self.connect_playback(sink) {
            Ok(stream) => {
                if let Some(mut old) = self.playback.replace(stream) {
                    let _ = old.disconnect();
                }
                log::info!("Playing to {sink}");
                self.output = sink.to_string();
                self.skip_bytes = 0;
                self.queue_strikes = 0;
                self.queue_last_ms = 0;
                self.queue_requested = false;
                self.queue_answered.set(false);
                self.next_queue_check = Instant::now() + QUEUE_SETTLE_TIME;
                self.output_retry_at = None;
            }
            // Keep whatever is working rather than going silent, and don't
            // hammer the server with a device that keeps failing.
            Err(e) => {
                log::error!("Keeping the previous output device: {e}");
                self.output_retry_at = Some(Instant::now() + Duration::from_secs(1));
            }
        }
    }

    fn refresh_devices(&mut self) -> Result<(), String> {
        let sinks = self.sinks()?;
        self.devices = hardware_sinks(&sinks);
        Ok(())
    }

    fn describe(&self, status: &mut RoutingStatus) {
        status.state = ServiceState::Running;
        status.message = None;
        status.devices = self.devices.clone();
        status.output = Some(self.output.clone());
        status.preferred = self.preferred.clone();
    }
}

/// Whether FXSound may play to a sink: anything but its own device and the
/// "Dummy Output" placeholder servers list when there is no real device
/// (pipewire-pulse refuses streams to it, and it would play nowhere anyway).
fn is_output_candidate(name: &str) -> bool {
    name != VIRTUAL_SINK && name != "auto_null"
}

/// Short, plain-language version of an internal error for the status bar; the
/// full reason goes to the log.
fn user_message(reason: &str) -> &'static str {
    let reason = reason.to_ascii_lowercase();
    if reason.contains("already running") {
        "FXSound is already running"
    } else if reason.contains("no audio output device") {
        "No output device found"
    } else if reason.contains("refused to create the fxsound device") {
        "The audio server won't create the FXSound device"
    } else if reason.contains("audio server") || reason.contains("pulseaudio") {
        "Waiting for the audio server…"
    } else {
        "Audio setup failed — retrying"
    }
}

/// Devices FXSound may play to.
fn hardware_sinks(sinks: &[SinkEntry]) -> Vec<AudioSink> {
    sinks
        .iter()
        .filter(|s| is_output_candidate(&s.name))
        .map(|s| AudioSink {
            name: s.name.clone(),
            description: s.description.clone(),
        })
        .collect()
}

/// Pick the device to play to: the user's choice if it is plugged in, else
/// the current one, else whatever was the default before FXSound, else the
/// highest-priority device left.
fn choose_output(
    sinks: &[SinkEntry],
    preferred: Option<&str>,
    current: Option<&str>,
    original: Option<&str>,
) -> Option<String> {
    let usable = |name: &&str| is_output_candidate(name) && sinks.iter().any(|s| s.name == *name);
    preferred
        .filter(usable)
        .or_else(|| current.filter(usable))
        .or_else(|| original.filter(usable))
        .map(str::to_string)
        .or_else(|| {
            sinks
                .iter()
                .filter(|s| is_output_candidate(&s.name))
                .max_by_key(|s| s.priority)
                .map(|s| s.name.clone())
        })
}

/// Whether a process exists. Only "no such process" means it doesn't: a
/// permission error (EPERM, or EACCES from AppArmor under snap confinement)
/// still proves it is there.
fn process_alive(pid: i32) -> bool {
    // SAFETY: signal 0 performs only the existence/permission check.
    let rc = unsafe { libc::kill(pid, 0) };
    rc == 0 || std::io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sink(name: &str, priority: i64) -> SinkEntry {
        SinkEntry {
            name: name.into(),
            description: name.into(),
            owner_module: None,
            monitor_source: 0,
            monitor_name: None,
            priority,
        }
    }

    #[test]
    fn never_chooses_its_own_sink() {
        let sinks = [sink(VIRTUAL_SINK, 9999)];
        assert_eq!(
            choose_output(
                &sinks,
                Some(VIRTUAL_SINK),
                Some(VIRTUAL_SINK),
                Some(VIRTUAL_SINK)
            ),
            None
        );
    }

    #[test]
    fn output_choice_order() {
        let sinks = [
            sink(VIRTUAL_SINK, 0),
            sink("speakers", 1000),
            sink("headphones", 900),
        ];
        // The user's choice wins while it is plugged in...
        assert_eq!(
            choose_output(&sinks, Some("headphones"), Some("speakers"), None).as_deref(),
            Some("headphones")
        );
        // ...otherwise stay put rather than jumping around...
        assert_eq!(
            choose_output(&sinks, Some("usb"), Some("headphones"), Some("speakers")).as_deref(),
            Some("headphones")
        );
        // ...then the device that was the default before FXSound...
        assert_eq!(
            choose_output(&sinks, None, None, Some("headphones")).as_deref(),
            Some("headphones")
        );
        // ...then the highest priority device.
        assert_eq!(
            choose_output(&sinks, None, None, None).as_deref(),
            Some("speakers")
        );
    }

    #[test]
    fn device_list_hides_the_fxsound_sink_and_placeholders() {
        let sinks = [
            sink("speakers", 0),
            sink(VIRTUAL_SINK, 0),
            sink("auto_null", 0),
        ];
        let names: Vec<_> = hardware_sinks(&sinks).into_iter().map(|s| s.name).collect();
        assert_eq!(names, ["speakers"]);
    }

    #[test]
    fn errors_read_plainly_in_the_ui() {
        assert_eq!(
            user_message("could not reach the audio server (Access denied)"),
            "Waiting for the audio server…"
        );
        assert_eq!(
            user_message("timed out waiting for the audio server"),
            "Waiting for the audio server…"
        );
        assert_eq!(
            user_message("no PulseAudio or PipeWire server is running (x)"),
            "Waiting for the audio server…"
        );
        assert_eq!(
            user_message("no audio output device was found"),
            "No output device found"
        );
        assert_eq!(
            user_message("FXSound is already running (another copy ...)"),
            "FXSound is already running"
        );
        assert_eq!(
            user_message("something unexpected"),
            "Audio setup failed — retrying"
        );
    }

    #[test]
    fn process_liveness() {
        assert!(process_alive(std::process::id() as i32));
        assert!(
            !process_alive(i32::MAX - 7),
            "an unused PID must read as dead"
        );
    }

    #[test]
    fn buffer_sizes_are_whole_frames() {
        assert_eq!(bytes_for_ms(10), 480 * 8);
        assert_eq!(bytes_for_ms(PLAYBACK_TARGET_MS) % 8, 0);
    }
}
