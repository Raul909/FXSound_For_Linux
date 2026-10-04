# FXSound for Linux — Comprehensive Guide

## Features

- **10-Band Parametric Equalizer** — biquad peak filters from 32Hz to 16kHz, ±12dB per band
- **5 Audio Effects** — Fidelity, Ambiance, Dynamic Boost, 3D Surround, HyperBass
- **10 Built-in Presets** — Music, Movies, Gaming, Podcast, Bass Boost, Vocal Boost, Deep Bass, Treble Boost, Night Mode, Flat
- **Real-time Audio Visualizer** — canvas-based FFT spectrum display at 60fps
- **System-wide processing** — adds an "FXSound" output device; everything your system plays goes through it, exactly once (PipeWire or PulseAudio)
- **Follows your devices** — pick an output in FXSound or in your sound settings, plug in headphones; FXSound plays wherever you choose
- **Remembers your settings** — EQ, effects, preset, power state and output device
- **Power Toggle** — instant bypass: audio keeps playing, unprocessed
- **Authentic FXSound Dark UI** — black and red theme matching the original Windows app
- **Keyboard Accessible** — full keyboard navigation with ARIA roles on all sliders
- **Native Linux App** — built with Tauri (Rust + React) for low resource usage

---

## Requirements

- **Linux** — Any distro with a modern desktop environment
- **Audio System** — PulseAudio or PipeWire (with PulseAudio compatibility layer)
  - Most modern distros ship with PipeWire by default (Ubuntu 22.10+, Fedora 34+, Arch)
  - Older distros use PulseAudio natively — both work
- **Architecture** — x86_64 (amd64)

---

## Download & Install Details

### Debian / Ubuntu / Pop!_OS / Mint

```bash
wget https://github.com/Raul909/FXSound_For_Linux/releases/latest/download/fxsound-linux_1.2.0_amd64.deb
sudo apt install ./fxsound-linux_1.2.0_amd64.deb
```

### Fedora / RHEL / openSUSE

```bash
wget https://github.com/Raul909/FXSound_For_Linux/releases/latest/download/fxsound-linux-1.2.0-1.x86_64.rpm
sudo dnf install ./fxsound-linux-1.2.0-1.x86_64.rpm   # openSUSE: sudo zypper install ./fxsound-linux-1.2.0-1.x86_64.rpm
```

### Snap

```bash
sudo snap install fxsound-linux
sudo snap connect fxsound-linux:audio-record   # needed on some systems to process system audio
```

### Arch Linux (AUR)

```bash
yay -S fxsound-linux
```

---

## How It Works

While FXSound runs, it adds an output device called **FXSound** and makes it
your default, so applications play into it. FXSound processes that audio and
plays the result on your real speakers or headphones — the device chosen in
FXSound's **Output Device** menu. Choosing a device in your desktop's sound
settings, or plugging in headphones, moves FXSound's output there too.

Because applications play into FXSound rather than straight to the speakers,
you always hear each sound once. When FXSound quits, the FXSound device is
removed and your previous output becomes the default again. The power button
is a bypass: audio keeps flowing, unprocessed.

---

## Build from Source

### Ubuntu / Debian

```bash
# Install system dependencies
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev \
  librsvg2-dev libpulse-dev build-essential curl wget pkg-config

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone https://github.com/Raul909/FXSound_For_Linux.git
cd FXSound_For_Linux
npm install
npm run tauri:dev       # Development mode (hot reload)
# npm run tauri:build   # Production binary (output in src-tauri/target/release)
```

### Fedora / RHEL

```bash
# Install system dependencies
sudo dnf install webkit2gtk4.1-devel gtk3-devel libappindicator-gtk3-devel \
  librsvg2-devel pulseaudio-libs-devel gcc pkg-config

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone https://github.com/Raul909/FXSound_For_Linux.git
cd FXSound_For_Linux
npm install
npm run tauri:dev
```

### Arch Linux / Manjaro

```bash
# Install system dependencies
sudo pacman -S webkit2gtk-4.1 gtk3 libappindicator-gtk3 librsvg libpulse \
  base-devel curl wget pkg-config

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone https://github.com/Raul909/FXSound_For_Linux.git
cd FXSound_For_Linux
npm install
npm run tauri:dev
```

---

## Tech Stack

| Technology | Purpose |
|------------|---------|
| Tauri 2.0  | Native app framework |
| Rust       | Audio processing backend |
| React 19   | UI framework |
| Vite       | Build tool |
| PulseAudio | Linux audio system integration |
| RustFFT    | Real-time spectrum analysis |

---

## Architecture

- **Frontend** (`src/`) — React 19 with `useCallback`/`useMemo` throughout; canvas-based visualizer (zero DOM mutations at 60fps)
- **DSP** (`src-tauri/src/audio.rs`) — biquad IIR filters per EQ band, the effects chain and limiter, FFT via RustFFT
- **Routing** (`src-tauri/src/pulse.rs`) — creates the FXSound device, records its monitor and plays to the chosen output; one dedicated thread owns all audio-server communication, with every wait time-limited
- **Settings** (`src-tauri/src/settings.rs`) — `~/.config/com.fxsound.linux/settings.json`
- **IPC** — Tauri `invoke()` calls: `set_eq_band`, `set_effect`, `apply_preset_state`, `set_power`, `get_settings`, `get_audio_status`, `set_output_device`, `get_visualizer_data`; routing changes arrive as `audio-status` events

---

## Troubleshooting

### "404 Not Found" when downloading

Make sure you're downloading from the [latest release page](https://github.com/Raul909/FXSound_For_Linux/releases/latest). The download URLs contain the version number — if a newer version is released, old URLs may stop working.

### No sound after FXSound crashed or was killed

FXSound removes its device when it quits. If it was killed outright, the
FXSound device can be left as your default output with nothing behind it.
Start FXSound again (it cleans up), choose your speakers in the sound settings,
or run:

```bash
pactl unload-module $(pactl list short modules | awk '/sink_name=fxsound_sink/{print $1}')
```

(From a source checkout, `./scripts/reset-audio.sh` does the same.)

### Status bar says "Waiting for the audio server…"

FXSound can't reach PipeWire/PulseAudio and keeps retrying.

- Check the server is running: `pactl info`
- PipeWire: `systemctl --user restart pipewire pipewire-pulse wireplumber`
- Snap: allow recording with `sudo snap connect fxsound-linux:audio-record`

### Sound comes from the wrong device

Choose the device in FXSound's **Output Device** menu, or in your desktop's
sound settings — FXSound follows either and remembers the choice. Apps you
explicitly sent to a specific device (e.g. in `pavucontrol`) stay there and
bypass FXSound.

### AppImage won't launch

```bash
# Make it executable
chmod +x fxsound-linux_*.AppImage

# If you get a FUSE error on newer distros:
sudo apt install libfuse2    # Ubuntu 22.04+
# or extract and run directly:
./fxsound-linux_*.AppImage --appimage-extract
./squashfs-root/AppRun
```

### Reporting a bug

Run `fxsound-linux` from a terminal and include its output, or attach the log
from `~/.local/share/com.fxsound.linux/logs/`.

### Build errors

- Make sure all system dependencies are installed (see [Build from Source](#build-from-source))
- Ensure Rust is up to date: `rustup update`
- Ensure Node.js 20+ is installed: `node --version`
- Clear build cache: `cd src-tauri && cargo clean && cd .. && npm run tauri:build`

---

## FAQ

**Is this the official FXSound for Linux?**
No. This is an independent open-source project inspired by FXSound (Windows only).

**Does this actually process my system audio?**
Yes. Everything your system plays goes through the FXSound output device and is processed in real time with the EQ and effects. See [How It Works](#how-it-works).

**What Linux distros does this work on?**
Any distro with PulseAudio or PipeWire — Ubuntu, Arch, Fedora, Debian, Mint, Pop!_OS, Manjaro, and more.

**Is it free?**
Yes, completely free and open-source (MIT license).

**Can I change the visualizer colors or theme?**
The app uses the authentic FXSound dark theme with red accents. Custom themes are not yet supported but are being considered for future releases.

---

## License

MIT © 2025 — Free to use, modify, and distribute.
