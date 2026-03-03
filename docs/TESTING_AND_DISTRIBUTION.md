# 🚀 Complete Guide: Test Locally & Distribute

## Part 1: Testing Locally

### Prerequisites

Before you start, make sure you have:
- Linux system (Ubuntu, Fedora, Arch, etc.)
- Internet connection
- Terminal access

### Step 1: Install System Dependencies

Run the automated setup script:

```bash
./scripts/setup-deps.sh
```

Or install manually based on your distro:

**Ubuntu/Debian/Mint/Pop!_OS:**
```bash
sudo apt update
sudo apt install -y \
    libpulse-dev \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

**Fedora:**
```bash
sudo dnf install -y \
    pulseaudio-libs-devel \
    webkit2gtk4.1-devel \
    openssl-devel \
    curl \
    wget \
    file \
    libappindicator-gtk3-devel \
    librsvg2-devel
```

**Arch/Manjaro:**
```bash
sudo pacman -S --needed \
    libpulse \
    webkit2gtk-4.1 \
    base-devel \
    curl \
    wget \
    file \
    openssl \
    libappindicator-gtk3 \
    librsvg
```

### Step 2: Install Rust (if not installed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Step 3: Install Node Dependencies

```bash
npm install
```

### Step 4: Verify System Readiness

```bash
./scripts/test-locally.sh
```

Expected output:
```
✅ Node.js v20.x.x
✅ npm 10.x.x
✅ Rust 1.x.x
✅ Cargo 1.x.x
✅ PulseAudio is running
✅ libpulse-dev installed
✅ webkit2gtk-4.1 installed
✅ Node modules installed
```

### Step 5: Run Development Mode

```bash
npm run tauri:dev
```

**What happens:**
1. Vite starts the React dev server
2. Rust backend compiles (first time takes 2-5 minutes)
3. Native window opens
4. PulseAudio streams are created
5. Audio processing starts

**Expected terminal output:**
```
[INFO] Starting PulseAudio processor...
[INFO] PulseAudio streams created successfully
[INFO] Audio processor started successfully
```

### Step 6: Test Audio Features

**Test EQ:**
1. Play some music (YouTube, Spotify, local player)
2. Move the 32Hz slider to +12dB → Bass increases
3. Move the 16kHz slider to +12dB → Treble increases
4. Move the 1kHz slider to -12dB → Mids decrease

**Test Effects:**
1. Set Fidelity to 100 → Audio sounds brighter
2. Set Dynamic Boost to 100 → Loud parts compress
3. Set HyperBass to 100 → Bass boosts significantly

**Test Presets:**
1. Select "Bass Boost" → EQ and effects change
2. Select "Gaming" → Settings update
3. Select "Flat" → Everything resets to 0

**Test Visualizer:**
1. Play music → Bars animate with audio
2. Stop music → Bars drop to minimum
3. Toggle power off → Bars fade

**Test Power Toggle:**
1. Click power button → Audio bypasses (no processing)
2. Click again → Processing resumes

### Step 7: Check for Issues

**Common issues:**

**No audio output:**
```bash
# Check PulseAudio
pulseaudio --check
echo $?  # Should output 0

# If not running
pulseaudio --start

# List devices
pactl list sinks short
```

**Build errors:**
```bash
# Clean and rebuild
cargo clean
npm run tauri:dev
```

**High CPU usage:**
- Normal: 8-15% during processing
- If higher: Close other audio apps

---

## Part 2: Build Production Version

### Step 1: Build Release

```bash
./scripts/build-release.sh
```

Or manually:
```bash
npm run tauri:build
```

**Build time:** 5-10 minutes (first time)

**Output location:**
```
src-tauri/target/release/bundle/
├── appimage/
│   └── fxsound-linux_1.0.0_amd64.AppImage
├── deb/
│   └── fxsound-linux_1.0.0_amd64.deb
└── rpm/
    └── fxsound-linux-1.0.0-1.x86_64.rpm
```

### Step 2: Test the AppImage

```bash
chmod +x src-tauri/target/release/bundle/appimage/*.AppImage
./src-tauri/target/release/bundle/appimage/*.AppImage
```

**Test checklist:**
- [ ] App opens
- [ ] Audio processing works
- [ ] EQ sliders work
- [ ] Effects work
- [ ] Visualizer works
- [ ] Power toggle works
- [ ] No crashes

### Step 3: Test the Deb Package (Ubuntu/Debian)

```bash
sudo dpkg -i src-tauri/target/release/bundle/deb/*.deb

# Run from terminal
fxsound-linux

# Or find in applications menu
```

**Uninstall:**
```bash
sudo dpkg -r fxsound-linux
```

---

## Part 3: Distribute Your App

### Option A: GitHub Releases (Easiest)

**Step 1: Commit your code**
```bash
git add .
git commit -m "Release v1.0.0"
git push origin main
```

**Step 2: Create a tag**
```bash
git tag v1.0.0
git push origin v1.0.0
```

**Step 3: Create GitHub Release**
1. Go to: `https://github.com/YOUR_USERNAME/fxsound-linux/releases/new`
2. Tag: `v1.0.0`
3. Title: `FXSound Linux v1.0.0 - First Stable Release`
4. Description:
   ```markdown
   ## FXSound for Linux v1.0.0
   
   Full-featured audio enhancer with real PulseAudio processing.
   
   ### Features
   - 10-band parametric equalizer
   - 3 audio effects (Fidelity, Dynamic Boost, HyperBass)
   - Real-time FFT visualizer
   - 10 built-in presets
   
   ### Installation
   
   **AppImage (all distros):**
   ```bash
   chmod +x fxsound-linux_1.0.0_amd64.AppImage
   ./fxsound-linux_1.0.0_amd64.AppImage
   ```
   
   **Deb (Ubuntu/Debian):**
   ```bash
   sudo dpkg -i fxsound-linux_1.0.0_amd64.deb
   ```
   ```
5. Upload files:
   - `fxsound-linux_1.0.0_amd64.AppImage`
   - `fxsound-linux_1.0.0_amd64.deb`
   - `fxsound-linux-1.0.0-1.x86_64.rpm`
6. Click "Publish release"

**Step 4: Share the link**
```
https://github.com/YOUR_USERNAME/fxsound-linux/releases/tag/v1.0.0
```

### Option B: Automated GitHub Actions

The `.github/workflows/release.yml` file is already set up.

**How it works:**
1. Push a tag: `git tag v1.0.0 && git push origin v1.0.0`
2. GitHub Actions automatically:
   - Builds the app
   - Creates a release
   - Uploads artifacts

**No manual upload needed!**

### Option C: Flatpak (Flathub)

**For wider distribution:**

1. **Test locally:**
   ```bash
   flatpak-builder build-dir com.fxsound.linux.yml --force-clean
   flatpak-builder --run build-dir com.fxsound.linux.yml fxsound-linux
   ```

2. **Submit to Flathub:**
   - Fork: https://github.com/flathub/flathub
   - Add your `com.fxsound.linux.yml`
   - Submit PR
   - Wait for approval (1-2 weeks)

3. **Users install:**
   ```bash
   flatpak install flathub com.fxsound.linux
   ```

### Option D: AUR (Arch Linux)

**For Arch users:**

1. **Test PKGBUILD:**
   ```bash
   makepkg -si
   ```

2. **Submit to AUR:**
   ```bash
   git clone ssh://aur@aur.archlinux.org/fxsound-linux.git
   cd fxsound-linux
   cp /path/to/PKGBUILD .
   makepkg --printsrcinfo > .SRCINFO
   git add PKGBUILD .SRCINFO
   git commit -m "Initial commit"
   git push
   ```

3. **Users install:**
   ```bash
   yay -S fxsound-linux
   ```

---

## Part 4: Update Your README

Add download links to your README:

```markdown
## Download

### Latest Release: v1.0.0

- [AppImage (Universal)](https://github.com/YOUR_USERNAME/fxsound-linux/releases/download/v1.0.0/fxsound-linux_1.0.0_amd64.AppImage)
- [Deb (Ubuntu/Debian)](https://github.com/YOUR_USERNAME/fxsound-linux/releases/download/v1.0.0/fxsound-linux_1.0.0_amd64.deb)
- [RPM (Fedora/RHEL)](https://github.com/YOUR_USERNAME/fxsound-linux/releases/download/v1.0.0/fxsound-linux-1.0.0-1.x86_64.rpm)

### Installation

**AppImage:**
```bash
chmod +x fxsound-linux_1.0.0_amd64.AppImage
./fxsound-linux_1.0.0_amd64.AppImage
```

**Deb:**
```bash
sudo dpkg -i fxsound-linux_1.0.0_amd64.deb
```

**RPM:**
```bash
sudo rpm -i fxsound-linux-1.0.0-1.x86_64.rpm
```
```

---

## Quick Reference

### Testing Commands
```bash
./scripts/test-locally.sh          # Check system
npm run tauri:dev          # Run dev mode
./scripts/verify-version.sh        # Verify versions
```

### Build Commands
```bash
./scripts/build-release.sh         # Build production
npm run tauri:build        # Manual build
```

### Distribution Commands
```bash
git tag v1.0.0            # Create tag
git push origin v1.0.0    # Push tag (triggers CI)
```

---

## Troubleshooting

### Build Issues

**"webkit2gtk not found"**
```bash
sudo apt install libwebkit2gtk-4.1-dev
```

**"libpulse not found"**
```bash
sudo apt install libpulse-dev
```

**Rust errors**
```bash
rustup update
cargo clean
```

### Runtime Issues

**"Failed to create input"**
```bash
pulseaudio --start
```

**No audio**
```bash
pactl list sinks short
pactl set-default-sink <sink-name>
```

**High CPU**
- Close other audio apps
- Reduce buffer size in audio.rs

---

## Success Checklist

- [ ] System dependencies installed
- [ ] Rust installed
- [ ] Node modules installed
- [ ] `npm run tauri:dev` works
- [ ] Audio processing works
- [ ] Production build succeeds
- [ ] AppImage tested
- [ ] GitHub release created
- [ ] Download links added to README

---

## Next Steps

1. **Test locally** → `./scripts/test-locally.sh`
2. **Build** → `./scripts/build-release.sh`
3. **Distribute** → Create GitHub release
4. **Share** → Reddit, Discord, Twitter

**You're ready to release!** 🎉
