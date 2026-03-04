# FXSound for Linux 🎧

> A free, open-source audio enhancer for Linux — with a 10-band equalizer, effects, presets, and real-time audio visualization. The Linux alternative to FXSound.

[![Live Demo](https://img.shields.io/badge/Live%20Demo-Visit%20App-e53935?style=for-the-badge)](https://YOUR_DEPLOY_URL_HERE)
[![License](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](./LICENSE)
[![React](https://img.shields.io/badge/React-18-61dafb?style=for-the-badge&logo=react)](https://reactjs.org)
[![PulseAudio](https://img.shields.io/badge/PulseAudio-Compatible-orange?style=for-the-badge)](https://www.freedesktop.org/wiki/Software/PulseAudio/)

---

## What is this?

FXSound is a popular Windows audio enhancer — but it has **no Linux version**. This project is a **full Linux-native recreation** with real PulseAudio audio processing, built with Tauri (Rust + React).

If you've been searching for **FXSound on Linux**, **an FXSound Linux alternative**, or a **free audio equalizer for Linux** — this is it.

**✨ Now with real-time audio processing!** This isn't just a UI mockup — it actually processes your system audio.

---

## Screenshots

<p align="center">
  <img src="./public/screenshots/screenshot-eq.png" alt="FXSound Linux Equalizer" width="400"/>
  <img src="./public/screenshots/screenshot-effects.png" alt="FXSound Linux Effects" width="400"/>
</p>

> **Note:** Screenshots show the authentic FXSound UI design with real-time audio processing.

---

## Features

- 🎚️ **10-Band Parametric Equalizer** — fine-tune from 32Hz to 16kHz, -12dB to +12dB per band
- 🎛️ **5 Audio Effects** — Fidelity, Ambiance, Dynamic Boost, 3D Surround, HyperBass
- 🎵 **10 Built-in Presets** — Music, Movies, Gaming, Podcast, Bass Boost, Vocal Boost, Deep Bass, Treble Boost, Night Mode, Flat
- 📊 **Real-time Audio Visualizer** — FFT-based spectrum display with 32 frequency bins
- 🔊 **PulseAudio Integration** — Processes system audio in real-time
- ⚡ **Power Toggle** — instantly bypass all audio processing
- 🖤 **Authentic FXSound Dark UI** — black and red theme matching the original Windows app
- 🐧 **Native Linux App** — Built with Tauri (Rust + React) for optimal performance

---

## Download & Install

### Quick Install (Choose One)

**AppImage (Universal - All Distros):**
```bash
wget https://github.com/YOUR_USERNAME/fxsound-linux/releases/latest/download/fxsound-linux_1.0.0_amd64.AppImage
chmod +x fxsound-linux_1.0.0_amd64.AppImage
./fxsound-linux_1.0.0_amd64.AppImage
```

**Debian/Ubuntu/Pop!_OS/Mint:**
```bash
wget https://github.com/YOUR_USERNAME/fxsound-linux/releases/latest/download/fxsound-linux_1.0.0_amd64.deb
sudo dpkg -i fxsound-linux_1.0.0_amd64.deb
```

**Fedora/RHEL/openSUSE:**
```bash
wget https://github.com/YOUR_USERNAME/fxsound-linux/releases/latest/download/fxsound-linux-1.0.0-1.x86_64.rpm
sudo rpm -i fxsound-linux-1.0.0-1.x86_64.rpm
```

**Arch Linux (AUR):**
```bash
yay -S fxsound-linux
```

**Flatpak:**
```bash
flatpak install flathub com.fxsound.linux
```

📥 **[Download Latest Release](https://github.com/YOUR_USERNAME/fxsound-linux/releases/latest)**

---

## Build from Source

```bash
# Clone the repo
git clone https://github.com/YOUR_USERNAME/fxsound-linux.git
cd fxsound-linux

# Install system dependencies
./scripts/setup-deps.sh

# Install Node dependencies
npm install

# Run in development mode
npm run tauri:dev

# OR build production binary
npm run tauri:build

# Run the AppImage
chmod +x src-tauri/target/release/bundle/appimage/*.AppImage
./src-tauri/target/release/bundle/appimage/*.AppImage
```

For detailed build instructions, see **[BUILD.md](./BUILD.md)**

---

## Testing Locally

### Quick Start

```bash
# 1. Check if your system is ready
./scripts/test-locally.sh

# 2. Run the app in development mode
npm run tauri:dev

# 3. Play some music and test:
#    - Adjust EQ sliders
#    - Toggle effects
#    - Watch the visualizer
#    - Toggle power button
```

### Detailed Testing Steps

1. **Install Dependencies**
   ```bash
   ./scripts/setup-deps.sh
   npm install
   ```

2. **Start Development Server**
   ```bash
   npm run tauri:dev
   ```
   
   Expected output:
   ```
   [INFO] Starting PulseAudio processor...
   [INFO] PulseAudio streams created successfully
   [INFO] Audio processor started successfully
   ```

3. **Test Audio Processing**
   - Play music (YouTube, Spotify, local files)
   - Move 32Hz slider → Bass should increase
   - Move 16kHz slider → Treble should increase
   - Set Fidelity to 100 → Audio should brighten
   - Set HyperBass to 100 → Bass should boost
   - Toggle power off → Audio bypasses (no processing)

4. **Check Logs**
   - Look for `[INFO]` messages in terminal
   - No `[ERROR]` messages should appear
   - Visualizer bars should move with audio

### Troubleshooting

**No audio output?**
```bash
# Check PulseAudio
pulseaudio --check || pulseaudio --start

# List audio devices
pactl list sinks short
```

**Build errors?**
```bash
# Reinstall dependencies
./scripts/setup-deps.sh

# Update Rust
rustup update
```

**High CPU usage?**
- Normal: 8-15% during processing
- If higher, check for other audio apps

---

## Live Demo

👉 **[Try the UI Demo](https://YOUR_DEPLOY_URL_HERE)** (Web version - UI only, no audio processing)

**Note:** The web demo is for preview only. Download the native app for real audio processing.

---

## Who is this for?

- Linux users who miss FXSound from Windows
- Anyone looking for a **free FXSound alternative on Linux**
- Ubuntu, Fedora, Arch, Debian, Pop!\_OS, Linux Mint users
- People using PulseAudio or PipeWire who want visual EQ control
- Budget audiophiles, gamers, podcast listeners, music lovers
- Anyone tired of flat, lifeless laptop speaker audio on Linux

---

## Getting Started

### Install and Run (Native App)

```bash
# Clone the repo
git clone https://github.com/YOUR_USERNAME/fxsound-linux.git
cd fxsound-linux

# Install system dependencies
./scripts/setup-deps.sh

# Install Node dependencies
npm install

# Run in development mode
npm run tauri:dev

# Build production binary
npm run tauri:build
```

### Quick Test

```bash
# Check if your system is ready
./scripts/test-audio.sh

# Start the app
npm run tauri:dev
```

**Play some music and adjust the EQ!** 🎵

---

## Tech Stack

| Technology | Purpose |
|------------|---------|
| Tauri 2.0  | Native app framework |
| Rust       | Audio processing backend |
| React 18   | UI framework |
| Vite       | Build tool |
| PulseAudio | Linux audio system |
| RustFFT    | Real-time spectrum analysis |

---

## Roadmap

- [x] 10-band EQ UI with draggable controls
- [x] 5 effect sliders with presets
- [x] Real-time visualizer
- [x] Output device selector
- [x] Power toggle / bypass
- [x] Authentic FXSound black & red theme
- [x] Real PulseAudio integration
- [x] Real-time audio processing
- [x] FFT-based visualizer
- [ ] Proper biquad EQ filters
- [ ] Reverb (Ambiance effect)
- [ ] HRTF 3D Surround
- [ ] Save and export custom presets
- [ ] Per-application audio routing
- [ ] System tray integration
- [ ] Flatpak packaging

---

## Distribution

### For Users

**Download pre-built packages:**
- Go to [Releases](https://github.com/YOUR_USERNAME/fxsound-linux/releases)
- Download AppImage (universal) or Deb/RPM for your distro
- Install and run

See **[INSTALL.md](./INSTALL.md)** for detailed installation instructions.

### For Developers

**Build and distribute your own version:**

1. **Build locally:**
   ```bash
   npm run tauri:build
   ```

2. **Output files:**
   - `src-tauri/target/release/bundle/appimage/*.AppImage` (universal)
   - `src-tauri/target/release/bundle/deb/*.deb` (Ubuntu/Debian)
   - `src-tauri/target/release/bundle/rpm/*.rpm` (Fedora/RHEL)

3. **Create GitHub Release:**
   ```bash
   # Tag and push
   git tag v1.0.0
   git push origin v1.0.0
   
   # GitHub Actions will automatically build and create release
   ```

4. **Alternative distribution channels:**
   - **Flathub:** Universal Linux package manager
   - **Snap Store:** Ubuntu's app store
   - **AUR:** Arch User Repository

See **[DEPLOYMENT_GUIDE.md](./DEPLOYMENT_GUIDE.md)** for complete deployment instructions including Flathub, Snap Store, and AUR submission.

---

## Contributing

Contributions are welcome! Open an issue or submit a PR.

```bash
git checkout -b feature/your-feature-name
git commit -m "Add your feature"
git push origin feature/your-feature-name
# Open a Pull Request
```

---

## FAQ

**Is this the official FXSound for Linux?**
No. This is an independent open-source project inspired by FXSound (Windows only). The official FXSound does not have a Linux version.

**Does this actually process my system audio?**
Yes! The app uses PulseAudio to capture, process, and output system audio in real-time with EQ and effects.

**What Linux distros does this work on?**
Any distro with PulseAudio — Ubuntu, Arch, Fedora, Debian, Mint, Pop!\_OS, Manjaro, and more.

**Is it free?**
Yes, completely free and open-source (MIT license).

---

## Documentation

### Quick Start
- **[QUICK_START.md](./QUICK_START.md)** - Get started in 5 minutes
- **[PROJECT_STRUCTURE.md](./PROJECT_STRUCTURE.md)** - Project organization

### Guides
- **[Testing & Distribution](./docs/TESTING_AND_DISTRIBUTION.md)** - Complete guide
- **[Distribution Strategies](./docs/DISTRIBUTION_GUIDE.md)** - How to distribute
- **[Audio Implementation](./docs/AUDIO_IMPLEMENTATION.md)** - Technical details

### Project Info
- **[VERSION.md](./VERSION.md)** - Version information and roadmap
- **[CHANGELOG.md](./CHANGELOG.md)** - Version history

### Scripts
- **[scripts/](./scripts/)** - Helper scripts for setup, testing, and building

---

## Related

- [FXSound (Official, Windows only)](https://www.fxsound.com)
- [FXSound GitHub](https://github.com/fxsound2/fxsound-app)
- [PulseAudio](https://www.freedesktop.org/wiki/Software/PulseAudio/)
- [PipeWire](https://pipewire.org/)
- [EasyEffects](https://github.com/wwmm/easyeffects) — another great Linux audio tool

---

## License

MIT © 2025 — Free to use, modify, and distribute.

---

<p align="center">
  Made for the Linux audio community 🐧<br/>
  <a href="https://YOUR_DEPLOY_URL_HERE">Live Demo</a> ·
  <a href="https://github.com/YOUR_USERNAME/fxsound-linux/issues">Report Bug</a> ·
  <a href="https://github.com/YOUR_USERNAME/fxsound-linux/issues">Request Feature</a>
</p>
