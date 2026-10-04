# Changelog

All notable changes to FXSound Linux will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [1.2.0] - 2026-10-04

### Fixed
- **The window no longer freezes a few seconds after launch ([#510](https://github.com/Raul909/FXSound_For_Linux/issues/510)).** Listing the output devices waited on the audio server from the UI thread, for a signal that never came, so every launch hung until the desktop offered to force-quit it — the `Killed` / exit code 137 in #511. The UI no longer talks to the audio server at all, every wait on the server has a time limit, and a stopped or restarting server now shows "Waiting for the audio server…" instead of freezing the window.
- **No more doubled audio or feedback loop ([#511](https://github.com/Raul909/FXSound_For_Linux/issues/511)).** FXSound recorded what the speakers were playing and played its processed copy into the same speakers: you heard the original and the processed version on top of each other, and FXSound re-recorded its own output, so the sound echoed every couple of seconds and grew louder each time. FXSound now adds its own **FXSound** output device and makes it the default; applications play into it and FXSound plays the result on your real speakers or headphones — exactly once.
- **The Snap no longer aborts at launch** with `libEGL fatal: did not find extension DRI_Mesa version 1` (#511). It bundled its own Mesa 23.2 next to the GNOME runtime's Mesa 25.2, and the two clashed; it now uses the runtime's libraries only, which also makes it far smaller. It now also shows up in the applications menu — it shipped without a menu entry or icon.
- **No more clicks when moving the EQ sliders or switching presets.** Every slider step rebuilt that band's filter from scratch, restarting it from silence mid-waveform — an audible click on each step of a drag, worst in the bass. Filters now keep their state and glide to new settings over 20 ms, and the power button crossfades instead of cutting.
- **Quiet passages and fade-outs are no longer muted.** The "silence" gate cut in below −60 dBFS, which is quiet music rather than silence, and chopped the Ambiance reverb tail the moment a track ended. Only true digital silence pauses processing now, and only after the tails have rung out.
- **Power off is now a true bypass**: audio keeps playing, unprocessed. It used to output silence, which only made sense while the original audio was also playing directly — the doubling above.
- The visualizer only updated when audio arrived in large blocks; it now works however the audio server delivers it.
- The `.deb` and `.rpm` now declare their dependency on libpulse, without which the app could not start on a minimal install. The unused tray-icon dependency (libayatana-appindicator) is gone.

### Added
- **Settings are remembered.** EQ, effects, preset, power state and output device are restored at launch; every launch used to start over on the Music preset.
- **The output device follows your system.** Pick a device in your sound settings or plug in headphones and FXSound plays there; unplug it and FXSound falls back to another device, then returns when it is plugged back in. The device list updates live.
- Launching FXSound while it is already running brings the open window to the front instead of starting a competing copy.
- FXSound restores your audio exactly as it was when it quits — including on logout, `Ctrl+C` or `kill` (SIGTERM/SIGINT/SIGHUP). If it was killed outright, launching it again cleans up, as does the new `scripts/reset-audio.sh`.
- The status bar says when audio is unavailable and why.
- Logs are written to `~/.local/share/com.fxsound.linux/logs/`, so bug reports can include them.
- Continuous integration on every pull request: lint, unit tests, end-to-end routing tests against real PipeWire and PulseAudio servers (`tests/audio-routing/`), and a Snap that is built, installed and launched under strict confinement.

### Changed
- `scripts/setup-audio.sh` is replaced by `scripts/reset-audio.sh`. The old script wired a loopback into a sink of the same name, which combined with the app fed audio back into itself.

### Known limitations
- If FXSound is killed outright (`kill -9`, a crash), its device stays the default output and you hear nothing until FXSound is started again or you choose your speakers in the sound settings (or run `scripts/reset-audio.sh`).
- On native PulseAudio (not PipeWire) the FXSound device and the sound card run on separate clocks. FXSound keeps the delay in check by skipping ahead if its output queue grows, which can rarely be heard as a tiny skip; under PipeWire both run on one clock and this does not happen.

## [1.1.4] - 2026-08-17

### Fixed
- **Startup no longer lies about what you are hearing.** The window opened on the Music preset while the audio engine was still flat with every effect at zero, so the sliders described a curve that was never applied until you touched a control. The engine is now seeded with the startup preset.
- **HyperBass actually boosts bass.** It applied a flat broadband gain — a volume knob that changed no tonal balance at all. It is now a low-shelf filter at 110 Hz (measured +5.4 dB at 60 Hz, ±0 dB at 8 kHz at full setting).
- **Dynamic Boost no longer makes audio quieter and grittier.** It was an instantaneous hard-knee waveshaper with no envelope and no makeup gain, so raising it attenuated the signal and added harmonic distortion. It is now a proper compressor with attack/release and makeup gain (+2.7 dB at full setting).
- **Removed pumping on loud material.** The "limiter" normalised each 1024-sample block by that block's own peak, stepping the gain at every block boundary — roughly 90 Hz amplitude modulation on anything driven past 0 dBFS. Replaced with a sample-accurate limiter whose envelope persists across buffers.
- **Fidelity now enhances treble instead of colouring bass.** It saturated the full-band signal; it now drives only the band above 3 kHz (measured ±0 dB at 80 Hz, +5.3 dB at 9 kHz).
- **The Output Device dropdown works.** Selecting a device only changed a React value — playback always went to the system default. It now retargets the live playback stream, lists real sink names, and preselects the sink audio is actually on.
- **The visualizer shows real audio.** It only used the engine's spectrum if audio happened to already be playing loudly at launch; otherwise it requested a screen-capture stream and, failing that, animated an invented sine wave. It now always reads the engine.
- **The visualizer no longer freezes.** Bars latched at their last height when playback stopped or power was switched off; they now fall to the floor.
- Fixed the spectrum smearing across neighbouring bars (an FFT window function is now applied).
- Fixed blurry visualizer rendering on HiDPI and fractionally-scaled desktops.
- Fixed the visualizer crashing on WebKitGTK older than 2.40, which lacks `Canvas.roundRect`.
- Fixed the panel being squeezed 40 px narrower than designed, and the window not fitting on 1366×768 screens. The window is now resizable with sensible minimums.
- Removed the fabricated fallback device list ("Built-in Speakers", "Headphones (3.5mm)"…) shown when detection failed.
- Fixed effect slider labels highlighting while dragging.
- Fixed the visualizer's glow overlay painting on top of the bars instead of behind them.

### Changed
- **No webfont is fetched at launch.** The UI pulled Inter from `fonts.googleapis.com` on every start, which fails under strict snap confinement and offline, delayed first paint while it timed out, and phoned home from a local-only tool. Fonts now resolve locally, and the content security policy no longer allows external hosts.
- Reduced-motion preferences from the desktop are now honoured.
- Denormal protection added to the reverb and EQ feedback paths, avoiding CPU spikes as tails decay on x86.
- The version in the status bar is injected from `package.json` at build time — it had drifted a release behind.

## [1.1.3] - 2026-07-14

### Fixed
- Fixed the app still aborting with `Could not create default EGL display: EGL_BAD_PARAMETER` on systems with no usable GPU acceleration — most visibly the AppImage on a VirtualBox/VMware VM, which stayed on a blank window while the installed `.deb` worked on the same machine. Root cause: WebKitGTK brings up a *shared EGL display at process startup regardless of compositing mode*, so the 1.1.1 (DMABUF) and 1.1.2 (compositing) toggles never prevented that abort. The app now defaults `LIBGL_ALWAYS_SOFTWARE=1`, pointing Mesa at its software rasteriser (llvmpipe) so WebKitGTK's EGL display always initializes. Set `LIBGL_ALWAYS_SOFTWARE=0` before launching to force the GPU path back on.

## [1.1.2] - 2026-07-12

### Fixed
- Fixed the app still failing on newer Mesa/Wayland systems (e.g. Fedora + GNOME) despite the 1.1.1 fix — the AppImage still aborted with `Could not create default EGL display: EGL_BAD_PARAMETER` (blank window) and the `.deb`/`.rpm` builds painted once then went **Not Responding**. WebKitGTK accelerated compositing is now disabled by default (`WEBKIT_DISABLE_COMPOSITING_MODE=1`), so the webview renders in software and never creates an EGL display. Disabling only the DMABUF renderer (1.1.1) was not enough. Set `WEBKIT_DISABLE_COMPOSITING_MODE=0` before launching to re-enable GPU compositing.

## [1.1.1] - 2026-07-12

### Fixed
- Fixed a black, unresponsive window on some Linux systems (Fedora, NVIDIA drivers, certain Mesa/Wayland and virtual-machine setups) where the app aborted with `Could not create default EGL display: EGL_BAD_PARAMETER`. The WebKitGTK DMABUF renderer is now disabled by default, which falls back to a compatible rendering path. Set `WEBKIT_DISABLE_DMABUF_RENDERER=0` before launching to opt back in.

## [1.1.0] - 2026-07-09

### Added
- **Ambiance** effect: a compact stereo reverb (reduced Freeverb — 4 comb + 2 allpass filters per channel) mixed as a parallel wet send, adding a sense of space without hollowing out the dry signal.
- **3D Surround** effect: mid/side stereo widening that preserves the mono (center) component, so mono content and downmix compatibility stay intact.

### Fixed
- The Ambiance and 3D Surround sliders previously had no effect on audio — both are now fully implemented in the Rust audio engine.

### Changed
- `apply_effects` now runs a defined chain: per-sample shaping (fidelity, dynamic, bass) → 3D surround → ambiance reverb, ahead of the limiter.

## [1.0.3] - 2026-06-24

### Fixed
- Fixed visualizer audio synchronization by mixing interleaved stereo output to mono.
- Replaced linear bin indexing with logarithmic/exponential mapping covering the full 20Hz-20kHz audible spectrum.
- Re-aligned visualizer bars responsiveness for a smoother, beat-accurate representation.

### Changed
- Increased Web Audio fallback visualizer `fftSize` to `1024` for higher resolution and identical exponential bar mapping in browser view.
- Captured updated screenshots for Equalizer and Effects tabs representing the authentic theme.

## [1.0.2] - 2026-06-24

### Added
- Authentic FXSound dark theme and true black Windows-matching design tokens.
- Filled EQ region visual feedback showing boost/cut curve.
- Red visualizer gradient bars with reflections and baseline.

### Fixed
- Fixed duplicate keydown handlers, Home/End key assignments, and CSP font-blocking issues.

## [1.0.0] - 2026-03-04

### Added
- Real-time PulseAudio audio processing
- 10-band parametric equalizer (32Hz, 64Hz, 125Hz, 250Hz, 500Hz, 1kHz, 2kHz, 4kHz, 8kHz, 16kHz)
- Fidelity effect (high-frequency enhancement)
- Dynamic Boost effect (compression/limiting)
- HyperBass effect (low-frequency boost)
- Real-time FFT visualizer with 32 frequency bins
- Power toggle with bypass mode
- 10 built-in presets (Music, Movies, Gaming, Podcast, Bass Boost, Vocal Boost, Deep Bass, Treble Boost, Night Mode, Flat)
- Native Linux application using Tauri 2.0
- Rust audio engine with libpulse bindings
- AppImage distribution (universal)
- Deb package (Ubuntu/Debian)
- RPM package (Fedora/RHEL)
- System tray icon support
- Comprehensive documentation

### Changed
- Migrated from web-only UI to native Tauri application
- Replaced fake visualizer with real FFT analysis
- Updated UI to show v1.0.0

### Technical
- Tauri 2.0 framework
- Rust backend with PulseAudio integration
- React 18 frontend
- RustFFT for spectrum analysis
- ~20-30ms audio latency
- ~30-50MB memory usage
- ~8-15% CPU usage during processing

## [0.1.0] - 2026-03-03

### Added
- Initial Tauri project structure
- Basic audio engine skeleton
- Tauri command handlers (set_eq_band, set_effect, set_power)
- React UI with Tauri IPC integration

### Changed
- Migrated from pure web app to Tauri

## [0.0.0] - 2026-03-03

### Added
- Initial web-only UI mockup
- 10-band EQ interface
- 5 effect sliders
- Preset selector
- Fake animated visualizer
- FXSound-inspired dark theme

### Note
- No actual audio processing in this version
- UI demonstration only

---

## Upcoming

### [1.2.0] - Planned
- Virtual-sink capture architecture (eliminate audio doubling/feedback for true system-wide processing)
- Advanced HRTF-based 3D Surround
- Per-application audio routing
- Device selection from PulseAudio

### [2.0.0] - Future
- PipeWire native support
- Plugin system for custom effects
- Custom effect chains
- Advanced visualizer modes
- Remote control API

---

[1.1.3]: https://github.com/Raul909/FXSound_For_Linux/releases/tag/v1.1.3
[1.1.2]: https://github.com/Raul909/FXSound_For_Linux/releases/tag/v1.1.2
[1.1.1]: https://github.com/Raul909/FXSound_For_Linux/releases/tag/v1.1.1
[1.1.0]: https://github.com/Raul909/FXSound_For_Linux/releases/tag/v1.1.0
[1.0.3]: https://github.com/Raul909/FXSound_For_Linux/releases/tag/v1.0.3
[1.0.2]: https://github.com/Raul909/FXSound_For_Linux/releases/tag/v1.0.2
[1.0.0]: https://github.com/Raul909/FXSound_For_Linux/releases/tag/v1.0.0
