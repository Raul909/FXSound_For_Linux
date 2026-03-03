# ⚡ Quick Start Guide

## 🧪 Test Locally (5 minutes)

```bash
# 1. Install dependencies
./scripts/setup-deps.sh

# 2. Install Node modules
npm install

# 3. Check system
./scripts/test-locally.sh

# 4. Run the app
npm run tauri:dev

# 5. Play music and test!
```

---

## 📦 Build for Distribution (10 minutes)

```bash
# 1. Build production version
./scripts/build-release.sh

# 2. Test the AppImage
chmod +x src-tauri/target/release/bundle/appimage/*.AppImage
./src-tauri/target/release/bundle/appimage/*.AppImage

# 3. Files are ready in:
#    src-tauri/target/release/bundle/
```

---

## 🚀 Distribute on GitHub (5 minutes)

```bash
# 1. Commit and push
git add .
git commit -m "Release v1.0.0"
git push origin main

# 2. Create and push tag
git tag v1.0.0
git push origin v1.0.0

# 3. Go to GitHub → Releases → New Release
#    - Tag: v1.0.0
#    - Upload files from src-tauri/target/release/bundle/
#    - Publish

# 4. Share the link!
```

---

## 📋 Testing Checklist

When running `npm run tauri:dev`:

- [ ] Window opens (420x680)
- [ ] Play music → Audio is processed
- [ ] Move 32Hz slider → Bass changes
- [ ] Move 16kHz slider → Treble changes
- [ ] Set Fidelity to 100 → Audio brightens
- [ ] Set HyperBass to 100 → Bass boosts
- [ ] Toggle power off → Audio bypasses
- [ ] Visualizer bars move with music
- [ ] Presets change settings
- [ ] No errors in terminal

---

## 🐛 Common Issues

**No audio output?**
```bash
pulseaudio --start
pactl list sinks short
```

**Build errors?**
```bash
./scripts/setup-deps.sh
rustup update
```

**App won't start?**
```bash
npm install
npm run tauri:dev -- --verbose
```

---

## 📚 Full Documentation

- **Testing:** See [TESTING_AND_DISTRIBUTION.md](./TESTING_AND_DISTRIBUTION.md)
- **Distribution:** See [DISTRIBUTION_GUIDE.md](./DISTRIBUTION_GUIDE.md)
- **Audio:** See [AUDIO_IMPLEMENTATION.md](./AUDIO_IMPLEMENTATION.md)

---

## 🎯 Next Steps

1. **Test:** `./scripts/test-locally.sh`
2. **Build:** `./scripts/build-release.sh`
3. **Release:** Create GitHub release
4. **Share:** Tell the world!

**Ready? Let's go!** 🚀
