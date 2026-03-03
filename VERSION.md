# FXSound Linux - Version 1.0.0

## Release Information

**Version:** 1.0.0  
**Release Date:** March 2026  
**Codename:** "First Stable Release"

---

## What's New in 1.0.0

### ✅ Core Features
- Real-time PulseAudio audio processing
- 10-band parametric equalizer (32Hz - 16kHz)
- 3 working audio effects (Fidelity, Dynamic Boost, HyperBass)
- Real-time FFT visualizer (32 bins)
- Power toggle with bypass mode
- 10 built-in presets

### ✅ Technical
- Native Linux application (Tauri 2.0)
- Rust audio engine with libpulse
- React 18 UI
- Low latency (~20-30ms)
- Low resource usage (~30-50MB RAM, 8-15% CPU)

### ✅ Distribution
- AppImage (universal)
- Deb package (Ubuntu/Debian)
- RPM package (Fedora/RHEL)

---

## Version History

### 1.0.0 (March 2026) - First Stable Release
- Initial release with full audio processing
- PulseAudio integration
- 10-band EQ
- 3 effects (Fidelity, Dynamic Boost, HyperBass)
- Real-time visualizer
- Native Tauri application

### 0.1.0 (March 2026) - Development
- Tauri migration
- Basic audio engine structure
- UI implementation

### 0.0.0 (March 2026) - Initial
- Web-only UI mockup
- No audio processing

---

## Upgrade Notes

### From Web Version (0.0.0)
- Download and install native app
- Web version remains as UI demo only

### From Development (0.1.0)
- Rebuild from source or download new release
- No breaking changes

---

## Known Issues

1. **Simplified EQ** - Uses basic gain, not true biquad filters
   - Workaround: Use moderate gain values
   - Fix planned for v1.1.0

2. **Ambiance & 3D Surround** - Not yet implemented
   - Placeholder values stored but not applied
   - Planned for v1.2.0

3. **No Preset Persistence** - Settings reset on restart
   - Workaround: Manually reapply settings
   - Fix planned for v1.1.0

---

## Roadmap

### v1.1.0 (Planned)
- [ ] Proper biquad EQ filters
- [ ] Preset persistence (save/load)
- [ ] System tray menu
- [ ] Auto-updater

### v1.2.0 (Planned)
- [ ] Reverb (Ambiance effect)
- [ ] HRTF 3D Surround
- [ ] Per-application audio routing

### v2.0.0 (Future)
- [ ] PipeWire native support
- [ ] Plugin system
- [ ] Custom effect chains

---

## Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- **Major (1.x.x)** - Breaking changes, major features
- **Minor (x.1.x)** - New features, backwards compatible
- **Patch (x.x.1)** - Bug fixes, minor improvements

---

## Support

- **Issues:** https://github.com/YOUR_USERNAME/fxsound-linux/issues
- **Discussions:** https://github.com/YOUR_USERNAME/fxsound-linux/discussions
- **Documentation:** See README.md and docs/

---

## License

MIT License - See LICENSE file for details

---

**Current Version:** 1.0.0  
**Last Updated:** March 4, 2026
