# Changelog

All notable changes to FXSound Linux will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

### [1.1.0] - Planned
- Proper biquad EQ filters
- Preset persistence (save/load custom presets)
- System tray menu with quick controls
- Auto-updater integration
- Bug fixes and performance improvements

### [1.2.0] - Planned
- Reverb implementation (Ambiance effect)
- HRTF 3D Surround processing
- Per-application audio routing
- Device selection from PulseAudio

### [2.0.0] - Future
- PipeWire native support
- Plugin system for custom effects
- Custom effect chains
- Advanced visualizer modes
- Remote control API

---

[1.0.0]: https://github.com/YOUR_USERNAME/fxsound-linux/releases/tag/v1.0.0
[0.1.0]: https://github.com/YOUR_USERNAME/fxsound-linux/releases/tag/v1.0.0
[0.0.0]: https://github.com/YOUR_USERNAME/fxsound-linux/releases/tag/v1.0.0
