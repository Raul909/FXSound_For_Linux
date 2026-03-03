# FXSound for Linux 🎧

> A free, open-source audio enhancer for Linux — with a 10-band equalizer, effects, presets, and real-time audio visualization. The Linux alternative to FXSound.

![FXSound Linux Screenshot](./screenshot.png)

[![Live Demo](https://img.shields.io/badge/Live%20Demo-Visit%20App-00bcd4?style=for-the-badge)](https://YOUR_DEPLOY_URL_HERE)
[![License](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](./LICENSE)
[![React](https://img.shields.io/badge/React-18-61dafb?style=for-the-badge&logo=react)](https://reactjs.org)
[![PulseAudio](https://img.shields.io/badge/PulseAudio-Compatible-orange?style=for-the-badge)](https://www.freedesktop.org/wiki/Software/PulseAudio/)

---

## What is this?

FXSound is a popular Windows audio enhancer — but it has **no Linux version**. This project is a full Linux-native recreation of the FXSound interface and feature set, built as a web app that runs entirely in your browser with zero installation required.

If you've been searching for **FXSound on Linux**, **an FXSound Linux alternative**, or a **free audio equalizer for Linux** — this is it.

---

## Features

- 🎚️ **10-Band Parametric Equalizer** — fine-tune from 32Hz to 16kHz, -12dB to +12dB per band
- 🎛️ **5 Audio Effects** — Fidelity, Ambiance, Dynamic Boost, 3D Surround, Bass Boost
- 🎵 **10 Built-in Presets** — Music, Movies, Gaming, Podcast, Bass Boost, Vocal Boost, Deep Bass, Treble Boost, Night Mode, Flat
- 📊 **Real-time Audio Visualizer** — animated spectrum display
- 🔊 **Output Device Selector** — supports Built-in Speakers, Headphones, USB Audio, HDMI, Bluetooth
- ⚡ **Power Toggle** — instantly bypass all audio processing
- 🌙 **Dark UI** — clean, modern dark interface built for Linux desktops
- 🖥️ **No installation required** — runs in any modern browser

---

## Live Demo

👉 **[Try it now — fxsound-linux.vercel.app](https://YOUR_DEPLOY_URL_HERE)**

No sign-up. No install. Works on any Linux distro.

---

## Screenshots

| Equalizer | Effects |
|-----------|---------|
| ![EQ View](./screenshots/eq.png) | ![Effects View](./screenshots/effects.png) |

---

## Who is this for?

- Linux users who miss FXSound from Windows
- Anyone looking for a **free FXSound alternative on Linux**
- Ubuntu, Fedora, Arch, Debian, Pop!\_OS, Mint users
- People using PulseAudio or PipeWire who want visual EQ control
- Budget audiophiles, gamers, podcast listeners, music lovers
- Anyone tired of flat, lifeless laptop speaker audio on Linux

---

## Getting Started

### Use the live app (easiest)

Just open the live demo link above in your browser. No setup needed.

### Run locally

```bash
# Clone the repo
git clone https://github.com/YOUR_USERNAME/fxsound-linux.git
cd fxsound-linux

# Install dependencies
npm install

# Start the dev server
npm run dev
```

Open `http://localhost:5173` in your browser.

### Build for production

```bash
npm run build
# Output is in the /dist folder — deploy anywhere
```

---

## Tech Stack

| Technology | Purpose |
|------------|---------|
| React 18 | UI framework |
| Vite | Build tool |
| CSS-in-JS | Styling |
| Web Audio API | (planned) Real audio processing |

---

## Roadmap

- [x] 10-band EQ UI with draggable controls
- [x] 5 effect sliders with presets
- [x] Real-time visualizer
- [x] Output device selector
- [x] Power toggle / bypass
- [ ] Real PulseAudio / PipeWire integration via WebSockets
- [ ] Save and export custom presets
- [ ] Per-application audio routing
- [ ] System tray integration (Electron build)
- [ ] AppImage / Flatpak packaging for desktop install

---

## Contributing

Contributions are welcome! If you want to help bring real PulseAudio integration or build a desktop Electron version, open an issue or submit a PR.

```bash
# Fork the repo, then:
git checkout -b feature/your-feature-name
git commit -m "Add your feature"
git push origin feature/your-feature-name
# Open a Pull Request
```

---

## Frequently Asked Questions

**Is this the official FXSound for Linux?**
No. This is an independent open-source project inspired by FXSound (Windows). The official FXSound does not have a Linux version.

**Does this actually process my system audio?**
The current version is a UI/UX replica with planned real audio processing via PulseAudio/PipeWire. System-level integration is on the roadmap.

**What Linux distros does this work on?**
Any Linux distro with a modern browser — Ubuntu, Arch, Fedora, Debian, Mint, Pop!\_OS, Manjaro, etc.

**Is it free?**
Yes, completely free and open-source under the MIT license.

---

## Related

- [FXSound (Official, Windows only)](https://www.fxsound.com)
- [PulseAudio](https://www.freedesktop.org/wiki/Software/PulseAudio/)
- [PipeWire](https://pipewire.org/)
- [EasyEffects](https://github.com/wwmm/easyeffects) — another great Linux audio tool

---

## License

MIT © 2025 — Free to use, modify, and distribute.

---

<p align="center">
  Made for the Linux audio community 🐧
  <br/>
  <a href="https://YOUR_DEPLOY_URL_HERE">Live Demo</a> · 
  <a href="https://github.com/YOUR_USERNAME/fxsound-linux/issues">Report Bug</a> · 
  <a href="https://github.com/YOUR_USERNAME/fxsound-linux/issues">Request Feature</a>
</p>
