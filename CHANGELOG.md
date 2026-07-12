# Changelog

All notable changes to FXSound Linux will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


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

[1.1.2]: https://github.com/Raul909/FXSound_For_Linux/releases/tag/v1.1.2
[1.1.1]: https://github.com/Raul909/FXSound_For_Linux/releases/tag/v1.1.1
[1.1.0]: https://github.com/Raul909/FXSound_For_Linux/releases/tag/v1.1.0
[1.0.3]: https://github.com/Raul909/FXSound_For_Linux/releases/tag/v1.0.3
[1.0.2]: https://github.com/Raul909/FXSound_For_Linux/releases/tag/v1.0.2
[1.0.0]: https://github.com/Raul909/FXSound_For_Linux/releases/tag/v1.0.0
