# FXSound for Linux

> A free, open-source audio enhancer for Linux — with a 10-band equalizer, effects, presets, and real-time audio visualization.

[![License](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](./LICENSE)
[![React](https://img.shields.io/badge/React-19-61dafb?style=for-the-badge&logo=react)](https://reactjs.org)
[![PipeWire](https://img.shields.io/badge/PipeWire-Compatible-orange?style=for-the-badge)](https://pipewire.org/)

---

## What is this?

FXSound is a popular Windows audio enhancer — but it has **no Linux version**. This project is a **full Linux-native recreation** with real PipeWire/PulseAudio audio processing, built with Tauri (Rust + React).

**This isn't just a UI mockup — it actually processes your system audio through a 10-band EQ and effects in real time.**

---

## Screenshots

<p align="center">
  <img src="./public/screenshots/screenshot-eq.png" alt="FXSound Linux Equalizer" width="400"/>
  <img src="./public/screenshots/screenshot-effects.png" alt="FXSound Linux Effects" width="400"/>
</p>

---

## Features

- **10-Band Parametric Equalizer** — fine-tune from 32Hz to 16kHz, -12dB to +12dB per band
- **5 Audio Effects** — Fidelity, Ambiance, Dynamic Boost, 3D Surround, HyperBass
- **10 Built-in Presets** — Music, Movies, Gaming, Podcast, Bass Boost, Vocal Boost, Deep Bass, Treble Boost, Night Mode, Flat
- **Real-time Audio Visualizer** — FFT-based spectrum display with 32 frequency bins
- **PipeWire/PulseAudio Integration** — processes system audio in real time
- **Power Toggle** — instantly bypass all audio processing
- **Authentic FXSound Dark UI** — black and red theme matching the original Windows app
- **Native Linux App** — built with Tauri (Rust + React) for low resource usage

---

## Download & Install

### AppImage (Universal — All Distros)

```bash
wget https://github.com/Raul909/FXSound_For_Linux/releases/latest/download/fxsound-linux_1.0.0_amd64.AppImage
chmod +x fxsound-linux_1.0.0_amd64.AppImage
./fxsound-linux_1.0.0_amd64.AppImage
```

### Debian / Ubuntu / Pop!_OS / Mint

```bash
wget https://github.com/Raul909/FXSound_For_Linux/releases/latest/download/fxsound-linux_1.0.0_amd64.deb
sudo dpkg -i fxsound-linux_1.0.0_amd64.deb
```

### Fedora / RHEL / openSUSE

```bash
wget https://github.com/Raul909/FXSound_For_Linux/releases/latest/download/fxsound-linux-1.0.0-1.x86_64.rpm
sudo rpm -i fxsound-linux-1.0.0-1.x86_64.rpm
```

### Arch Linux (AUR)

```bash
yay -S fxsound-linux
```

[Download Latest Release](https://github.com/Raul909/FXSound_For_Linux/releases/latest)

---

## Build from Source

```bash
# Clone the repo
git clone https://github.com/Raul909/FXSound_For_Linux.git
cd FXSound_For_Linux

# Install system dependencies (Ubuntu/Debian)
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev \
  librsvg2-dev libpulse-dev build-essential curl wget

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node dependencies
npm install

# Run in development mode
npm run tauri:dev

# Build production binary
npm run tauri:build
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

## Roadmap

- [x] 10-band EQ with draggable controls
- [x] 5 effect sliders with presets
- [x] Real-time visualizer
- [x] Output device selector
- [x] Power toggle / bypass
- [x] PulseAudio integration
- [x] FFT-based visualizer
- [ ] Biquad EQ filters
- [ ] Reverb (Ambiance effect)
- [ ] HRTF 3D Surround
- [ ] Save/export custom presets
- [ ] Per-application audio routing
- [ ] System tray integration
- [ ] Flatpak packaging

---

## Deployment

See **[DEPLOYMENT.md](./DEPLOYMENT.md)** for guides on publishing to Flathub, Snap Store, and AUR.

---

## Contributing

Contributions are welcome! Open an issue or submit a PR.

```bash
git checkout -b feature/your-feature-name
git commit -m "Add your feature"
git push origin feature/your-feature-name
```

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

---

## Related

- [FXSound (Official, Windows only)](https://www.fxsound.com)
- [FXSound GitHub](https://github.com/fxsound2/fxsound-app)
- [EasyEffects](https://github.com/wwmm/easyeffects) — another great Linux audio tool

---

## License

MIT © 2025 — Free to use, modify, and distribute.

<p align="center">
  Made for the Linux audio community 🐧<br/>
  <a href="https://github.com/Raul909/FXSound_For_Linux/issues">Report Bug</a> ·
  <a href="https://github.com/Raul909/FXSound_For_Linux/issues">Request Feature</a>
</p>
