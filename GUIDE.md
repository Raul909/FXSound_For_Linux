# FXSound for Linux — Comprehensive Guide

## Features

- **10-Band Parametric Equalizer** — biquad peak filters from 32Hz to 16kHz, ±12dB per band
- **5 Audio Effects** — Fidelity, Ambiance, Dynamic Boost, 3D Surround, HyperBass
- **10 Built-in Presets** — Music, Movies, Gaming, Podcast, Bass Boost, Vocal Boost, Deep Bass, Treble Boost, Night Mode, Flat
- **Real-time Audio Visualizer** — canvas-based FFT spectrum display at 60fps
- **PipeWire/PulseAudio Integration** — processes system audio in real time
- **Power Toggle** — instantly bypass all audio processing
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
wget https://github.com/Raul909/FXSound_For_Linux/releases/latest/download/fxsound-linux_1.1.2_amd64.deb
sudo dpkg -i fxsound-linux_1.1.2_amd64.deb
# If you get dependency errors:
sudo apt-get install -f
```

### Fedora / RHEL / openSUSE

```bash
wget https://github.com/Raul909/FXSound_For_Linux/releases/latest/download/fxsound-linux-1.1.2-1.x86_64.rpm
sudo rpm -i fxsound-linux-1.1.2-1.x86_64.rpm
```

### Arch Linux (AUR)

```bash
yay -S fxsound-linux
```

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
- **Backend** (`src-tauri/src/audio.rs`) — biquad IIR filters per EQ band, FFT via RustFFT, PulseAudio capture/playback loop
- **IPC** — Tauri `invoke()` calls: `set_eq_band`, `set_effect`, `set_power`, `get_visualizer_data`, `get_audio_devices`

---

## Troubleshooting

### "404 Not Found" when downloading

Make sure you're downloading from the [latest release page](https://github.com/Raul909/FXSound_For_Linux/releases/latest). The download URLs contain the version number — if a newer version is released, old URLs may stop working.

### No audio output / Double audio

- Make sure only **one instance** of FXSound is running
- Check that PulseAudio/PipeWire is running: `pactl info`
- Restart PulseAudio if needed: `pulseaudio -k && pulseaudio --start`
- For PipeWire: `systemctl --user restart pipewire pipewire-pulse`

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

### "Failed to start audio processor" error

This usually means PulseAudio can't find a monitor source. Check:
```bash
# List available sources (look for one ending in .monitor)
pactl list short sources

# If no monitor source exists, you may need to load the module:
pactl load-module module-loopback
```

### Build errors

- Make sure all system dependencies are installed (see [Build from Source](#build-from-source))
- Ensure Rust is up to date: `rustup update`
- Ensure Node.js 18+ is installed: `node --version`
- Clear build cache: `cd src-tauri && cargo clean && cd .. && npm run tauri:build`

---

## FAQ

**Is this the official FXSound for Linux?**
No. This is an independent open-source project inspired by FXSound (Windows only).

**Does this actually process my system audio?**
Yes. The app uses PulseAudio to capture, process, and output system audio in real time with EQ and effects.

**What Linux distros does this work on?**
Any distro with PulseAudio or PipeWire — Ubuntu, Arch, Fedora, Debian, Mint, Pop!_OS, Manjaro, and more.

**Is it free?**
Yes, completely free and open-source (MIT license).

**Can I change the visualizer colors or theme?**
The app uses the authentic FXSound dark theme with red accents. Custom themes are not yet supported but are being considered for future releases.

---

## License

MIT © 2025 — Free to use, modify, and distribute.
