# Contributing to FXSound for Linux

Thanks for your interest in improving FXSound for Linux! 🎧 Whether you're fixing a
bug, adding a preset, improving the DSP, or polishing docs — contributions of
every size are welcome.

## Ways to contribute

- 🐞 **Report bugs** — open an [issue](https://github.com/Raul909/FXSound_For_Linux/issues) with your distro, desktop environment, and audio server (PipeWire or PulseAudio).
- 🎚️ **Add or tune presets** — see `src/constants.js` (`PRESET_EQ` / `PRESET_FX`).
- 🎨 **UI polish** — the frontend lives in `src/` (React 19).
- 🔊 **DSP improvements** — the audio engine is `src-tauri/src/audio.rs` (Rust).
- 📦 **Packaging** — help improve the AppImage / Snap / `.deb` / `.rpm` / AUR builds.
- 📖 **Docs** — the README, `GUIDE.md`, and this file.

New here? Look for issues labelled **good first issue**.

## Project layout

| Path | What it is |
|------|-----------|
| `src/` | React frontend (UI, EQ sliders, effect sliders, visualizer) |
| `src/constants.js` | Presets, EQ band labels, effect definitions |
| `src-tauri/src/audio.rs` | Rust audio engine — EQ, effects, limiter, FFT |
| `src-tauri/src/lib.rs` | Tauri commands (the frontend ↔ backend IPC bridge) |
| `landing/` | Marketing site deployed to Cloudflare Pages |
| `.github/workflows/` | Release automation (GitHub Releases, Snap, Pages) |

## Development setup

**Prerequisites:** Rust (stable), Node.js 20+, and the Linux audio/webkit dev libraries.

```bash
# Debian / Ubuntu
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev \
  libayatana-appindicator3-dev librsvg2-dev libpulse-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel gtk3-devel \
  libappindicator-gtk3-devel librsvg2-devel pulseaudio-libs-devel
```

Then:

```bash
git clone https://github.com/Raul909/FXSound_For_Linux.git
cd FXSound_For_Linux
npm install
npm run tauri:dev      # hot-reloading dev build
```

## Before you open a PR

Please make sure these pass:

```bash
npm run lint                      # frontend lint
npm run build                     # frontend build
cd src-tauri && cargo test        # Rust unit tests (audio engine)
cargo fmt --check && cargo clippy # Rust formatting + lints
```

If you touch the audio engine, add or update a test in `src-tauri/src/audio.rs`
(see the existing `#[cfg(test)]` module — it covers pipeline identity, mono
preservation, and reverb stability).

## Pull request guidelines

1. **Branch** off `main` with a descriptive name (`fix/…`, `feat/…`, `docs/…`).
2. Keep PRs **focused** — one logical change per PR is easiest to review.
3. Write a clear description of **what** changed and **why**.
4. Match the surrounding code style; keep comments meaningful.
5. Update `CHANGELOG.md` (under a suitable version) when behavior changes.

## Code style

- **JavaScript/React:** follows the repo ESLint config (`npm run lint`).
- **Rust:** `cargo fmt` defaults; prefer clear, allocation-free hot paths in the
  audio loop (it runs per audio buffer in real time).

## Reporting security issues

Please **do not** open a public issue for security vulnerabilities. Instead, report
them privately via [GitHub Security Advisories](https://github.com/Raul909/FXSound_For_Linux/security/advisories/new).

## License

By contributing, you agree that your contributions will be licensed under the
project's [MIT License](https://github.com/Raul909/FXSound_For_Linux/blob/main/LICENSE).

---

Happy hacking, and thank you for making Linux audio better! 🐧
