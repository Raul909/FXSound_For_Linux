<div align="center">
  <img src="./public/screenshots/promo-graphic.png" width="100%" alt="FXSound for Linux">

  <h1>FXSound for Linux</h1>
  <p><b>The legendary FXSound audio enhancer, reborn for Linux.</b><br/>
  A free, open-source, <b>system-wide</b> equalizer &amp; effects suite — a 10-band EQ, five studio effects, 10 presets, and a real-time visualizer that process <i>all</i> your audio. No ads. No telemetry. Ever.</p>

  <p>
    <a href="https://github.com/Raul909/FXSound_For_Linux/releases/latest"><img src="https://img.shields.io/github/v/release/Raul909/FXSound_For_Linux?style=for-the-badge&color=e63462" alt="Latest Release"></a>
    <img src="https://img.shields.io/badge/license-MIT-blue?style=for-the-badge" alt="MIT License">
    <img src="https://img.shields.io/badge/platform-Linux-0d1117?style=for-the-badge&logo=linux&logoColor=white" alt="Linux">
    <a href="./CONTRIBUTING.md"><img src="https://img.shields.io/badge/PRs-welcome-brightgreen?style=for-the-badge" alt="PRs Welcome"></a>
  </p>

  <p>
    <a href="https://fxsound-linux.pages.dev" target="_blank" rel="noopener noreferrer">
      <img src="https://img.shields.io/badge/Official_Website-Download_Here-e63462?style=for-the-badge" alt="Official Website">
    </a>
  </p>

  <p>
    <a href="https://snapcraft.io/fxsound-linux">
      <img alt="Get it from the Snap Store" src="https://snapcraft.io/static/images/badges/en/snap-store-black.svg" width="180">
    </a>
  </p>

  <p>
    <a href="https://github.com/Raul909/FXSound_For_Linux/releases/latest">Download AppImage / .deb / .rpm →</a>
  </p>
</div>

---

<div align="center">
  <table>
    <tr>
      <td align="center"><b>10-Band Equalizer</b></td>
      <td align="center"><b>Studio Effects</b></td>
    </tr>
    <tr>
      <td><img src="./public/screenshots/screenshot-eq.png" width="100%" alt="FXSound Linux Equalizer"></td>
      <td><img src="./public/screenshots/screenshot-effects.png" width="100%" alt="FXSound Linux Effects"></td>
    </tr>
  </table>
</div>

---

## ✨ Why FXSound for Linux?

- **System-wide, not per-app.** Enhances everything your system plays — browser, music player, games — in real time via PipeWire / PulseAudio.
- **Real DSP, not a mockup.** A Rust audio engine with genuine biquad EQ filters, a full effects chain, a hard limiter, and FFT spectrum analysis. See [How it works](#-how-it-works).
- **Lightweight & native.** Built with Tauri (Rust + React) — a small binary with low memory and CPU use.
- **Private & free forever.** MIT-licensed. No ads, no accounts, no telemetry.

## 🎛️ Features

- **10-band parametric EQ** — 32 Hz → 16 kHz, ±12 dB per band, real biquad peaking filters.
- **5 audio effects** — Fidelity, Ambiance (reverb), Dynamic Boost, 3D Surround, and HyperBass.
- **10 built-in presets** — Music, Movies, Gaming, Podcast, Bass Boost, Vocal Boost, Deep Bass, Treble Boost, Night Mode, Flat — plus fully custom.
- **Real-time visualizer** — 32-bin FFT spectrum with smooth animation.
- **One-click bypass** — instant A/B with the power toggle.
- **Universal packaging** — AppImage, `.deb`, `.rpm`, Snap, and AUR.

---

## ⚡ Quick Start

The fastest way to install is via the **Snap Store** or the universal **AppImage**.

### 🌐 Official Downloads
Grab the right build for your system from the landing page:
**[👉 fxsound-linux.pages.dev](https://fxsound-linux.pages.dev)**

### 📦 Snap Store (Ubuntu, Linux Mint, etc.)
```bash
sudo snap install fxsound-linux
```

### 🐧 AppImage (Universal Linux — works everywhere, incl. Arch)
```bash
chmod +x fxsound-linux_*.AppImage
./fxsound-linux_*.AppImage
```

Debian/Ubuntu (`.deb`), Fedora/RHEL (`.rpm`) and Arch (AUR) instructions are in the **[Comprehensive Guide](./GUIDE.md)**.

---

## 🔧 Build from Source

For contributors and tinkerers:

```bash
# Prerequisites: Rust (stable) + Node.js 20+ + Linux audio/webkit dev libs.
# Debian/Ubuntu:
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev \
  libayatana-appindicator3-dev librsvg2-dev libpulse-dev

git clone https://github.com/Raul909/FXSound_For_Linux.git
cd FXSound_For_Linux
npm install
npm run tauri:dev      # run in dev mode with hot reload
npm run tauri:build    # produce AppImage / .deb / .rpm
```

See the [Comprehensive Guide](./GUIDE.md) for distro-specific dependencies and troubleshooting.

---

## 🧠 How it works

The React UI sends every EQ and effect change to the Rust backend over Tauri IPC. The engine ([`src-tauri/src/audio.rs`](./src-tauri/src/audio.rs)) captures system audio, runs it through ten biquad EQ filters, the effects chain (fidelity → dynamic → bass → 3D surround → ambiance reverb) and a limiter, then plays it back — while updating a shared FFT buffer that drives the visualizer.

It's **real audio processing**, not a UI demo, and it's unit-tested (`cargo test`) end to end.

---

## 🤝 Contributing

Contributions of every size are welcome — bug fixes, new presets, DSP improvements, packaging, or docs. Start with **[CONTRIBUTING.md](./CONTRIBUTING.md)**.

Good first areas: **presets**, **UI polish**, and **distro packaging**. Browse the [issues](https://github.com/Raul909/FXSound_For_Linux/issues) for ideas.

> ⭐ If FXSound makes your audio better, **star the repo** — it genuinely helps others discover the project.

---

## 🗺️ Roadmap

See the [CHANGELOG](./CHANGELOG.md). On deck: **virtual-sink capture** (true system-wide processing without audio doubling), **advanced HRTF 3D surround**, **per-app audio routing**, and **native PipeWire** support.

---

## 🚀 Releases & Deployment

Releases are fully automated. Pushing a `v*` tag triggers CI to build and publish:

- The `AppImage`, `.deb`, and `.rpm` to **GitHub Releases**
- The Snap to the **Snap Store**
- The landing page to **Cloudflare Pages**

See [DEPLOYMENT.md](./DEPLOYMENT.md) for details.

---

<p align="center">
  Made for the Linux audio community 🐧<br/>
  <a href="https://github.com/Raul909/FXSound_For_Linux/issues">Report Bug</a> ·
  <a href="https://github.com/Raul909/FXSound_For_Linux/issues">Request Feature</a> ·
  <a href="./CONTRIBUTING.md">Contribute</a> ·
  <a href="https://fxsound-linux.pages.dev">Website</a>
</p>
