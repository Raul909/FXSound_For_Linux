# Complete Deployment & Distribution Guide

## Table of Contents
1. [Building Release Binaries](#building-release-binaries)
2. [GitHub Releases (Recommended)](#github-releases-recommended)
3. [Flathub (Universal Linux)](#flathub-universal-linux)
4. [Snap Store](#snap-store)
5. [AUR (Arch Linux)](#aur-arch-linux)
6. [Personal Website Hosting](#personal-website-hosting)
7. [Auto-Update Setup](#auto-update-setup)

---

## Building Release Binaries

### 1. Build All Formats

```bash
# Build production binaries
npm run tauri:build

# Output files will be in:
# - src-tauri/target/release/bundle/appimage/*.AppImage
# - src-tauri/target/release/bundle/deb/*.deb
# - src-tauri/target/release/bundle/rpm/*.rpm
```

### 2. Test Before Release

```bash
# Test AppImage
chmod +x src-tauri/target/release/bundle/appimage/*.AppImage
./src-tauri/target/release/bundle/appimage/*.AppImage

# Test Deb (Ubuntu/Debian)
sudo dpkg -i src-tauri/target/release/bundle/deb/*.deb

# Test RPM (Fedora)
sudo rpm -i src-tauri/target/release/bundle/rpm/*.rpm
```

---

## GitHub Releases (Recommended - 100% Free)

### Setup

1. **Create GitHub Repository**
```bash
# Initialize git if not already done
git init
git add .
git commit -m "Initial commit"

# Create repo on GitHub, then:
git remote add origin https://github.com/YOUR_USERNAME/fxsound-linux.git
git branch -M main
git push -u origin main
```

2. **Create Release Workflow**

Create `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    strategy:
      fail-fast: false
      matrix:
        platform: [ubuntu-22.04]
    
    runs-on: ${{ matrix.platform }}
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: 20
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Install dependencies (Ubuntu)
        if: matrix.platform == 'ubuntu-22.04'
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.0-dev \
            build-essential \
            curl \
            wget \
            file \
            libssl-dev \
            libgtk-3-dev \
            libayatana-appindicator3-dev \
            librsvg2-dev \
            libpulse-dev
      
      - name: Install frontend dependencies
        run: npm install
      
      - name: Build
        run: npm run tauri:build
      
      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            src-tauri/target/release/bundle/appimage/*.AppImage
            src-tauri/target/release/bundle/deb/*.deb
            src-tauri/target/release/bundle/rpm/*.rpm
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

3. **Create a Release**

```bash
# Tag your version
git tag v1.0.0
git push origin v1.0.0

# GitHub Actions will automatically build and create release
```

4. **Update README with Download Link**

```markdown
## Download

**Latest Release:** [v1.0.0](https://github.com/YOUR_USERNAME/fxsound-linux/releases/latest)

### AppImage (Universal)
```bash
wget https://github.com/YOUR_USERNAME/fxsound-linux/releases/download/v1.0.0/fxsound-linux_1.0.0_amd64.AppImage
chmod +x fxsound-linux_1.0.0_amd64.AppImage
./fxsound-linux_1.0.0_amd64.AppImage
```

### Debian/Ubuntu
```bash
wget https://github.com/YOUR_USERNAME/fxsound-linux/releases/download/v1.0.0/fxsound-linux_1.0.0_amd64.deb
sudo dpkg -i fxsound-linux_1.0.0_amd64.deb
```

### Fedora/RHEL
```bash
wget https://github.com/YOUR_USERNAME/fxsound-linux/releases/download/v1.0.0/fxsound-linux-1.0.0-1.x86_64.rpm
sudo rpm -i fxsound-linux-1.0.0-1.x86_64.rpm
```
```

---

## Flathub (Universal Linux - 100% Free)

### Benefits
- Works on ALL Linux distros
- Sandboxed security
- Automatic updates
- Centralized distribution

### Setup

1. **Create Flatpak Manifest**

Create `com.fxsound.linux.yml`:

```yaml
app-id: com.fxsound.linux
runtime: org.freedesktop.Platform
runtime-version: '23.08'
sdk: org.freedesktop.Sdk
sdk-extensions:
  - org.freedesktop.Sdk.Extension.rust-stable
  - org.freedesktop.Sdk.Extension.node20

command: fxsound-linux

finish-args:
  - --share=ipc
  - --socket=x11
  - --socket=wayland
  - --socket=pulseaudio
  - --device=dri
  - --filesystem=xdg-music:ro

modules:
  - name: fxsound-linux
    buildsystem: simple
    build-commands:
      - npm install
      - npm run tauri:build
      - install -Dm755 src-tauri/target/release/fxsound-linux /app/bin/fxsound-linux
      - install -Dm644 src-tauri/icons/icon.png /app/share/icons/hicolor/256x256/apps/com.fxsound.linux.png
      - install -Dm644 com.fxsound.linux.desktop /app/share/applications/com.fxsound.linux.desktop
      - install -Dm644 com.fxsound.linux.metainfo.xml /app/share/metainfo/com.fxsound.linux.metainfo.xml
    sources:
      - type: git
        url: https://github.com/YOUR_USERNAME/fxsound-linux.git
        tag: v1.0.0
```

2. **Create Desktop File**

Create `com.fxsound.linux.desktop`:

```desktop
[Desktop Entry]
Name=FXSound Linux
Comment=Audio enhancer with EQ and effects
Exec=fxsound-linux
Icon=com.fxsound.linux
Type=Application
Categories=AudioVideo;Audio;
Keywords=audio;equalizer;effects;sound;
```

3. **Create AppStream Metadata**

Create `com.fxsound.linux.metainfo.xml`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<component type="desktop-application">
  <id>com.fxsound.linux</id>
  <name>FXSound Linux</name>
  <summary>Audio enhancer with 10-band EQ and effects</summary>
  <metadata_license>CC0-1.0</metadata_license>
  <project_license>MIT</project_license>
  
  <description>
    <p>
      FXSound is a free, open-source audio enhancer for Linux with a 10-band equalizer,
      effects, presets, and real-time audio visualization.
    </p>
    <p>Features:</p>
    <ul>
      <li>10-Band Parametric Equalizer (32Hz - 16kHz)</li>
      <li>5 Audio Effects (Fidelity, Ambiance, Dynamic Boost, 3D Surround, HyperBass)</li>
      <li>10 Built-in Presets</li>
      <li>Real-time Audio Visualizer</li>
      <li>PulseAudio Integration</li>
    </ul>
  </description>
  
  <screenshots>
    <screenshot type="default">
      <image>https://raw.githubusercontent.com/YOUR_USERNAME/fxsound-linux/main/public/screenshots/screenshot-eq.png</image>
    </screenshot>
    <screenshot>
      <image>https://raw.githubusercontent.com/YOUR_USERNAME/fxsound-linux/main/public/screenshots/screenshot-effects.png</image>
    </screenshot>
  </screenshots>
  
  <url type="homepage">https://github.com/YOUR_USERNAME/fxsound-linux</url>
  <url type="bugtracker">https://github.com/YOUR_USERNAME/fxsound-linux/issues</url>
  
  <releases>
    <release version="1.0.0" date="2025-03-04"/>
  </releases>
  
  <content_rating type="oars-1.1"/>
</component>
```

4. **Submit to Flathub**

```bash
# Fork https://github.com/flathub/flathub
# Add your manifest to the fork
# Submit pull request

# Or test locally first:
flatpak-builder build-dir com.fxsound.linux.yml --force-clean --install --user
flatpak run com.fxsound.linux
```

**Official Guide:** https://docs.flathub.org/docs/for-app-authors/submission

---

## Snap Store (100% Free)

### Setup

1. **Create snapcraft.yaml**

Create `snap/snapcraft.yaml`:

```yaml
name: fxsound-linux
version: '1.0.0'
summary: Audio enhancer with EQ and effects
description: |
  FXSound is a free, open-source audio enhancer for Linux with a 10-band equalizer,
  effects, presets, and real-time audio visualization.

base: core22
confinement: strict
grade: stable

apps:
  fxsound-linux:
    command: bin/fxsound-linux
    extensions: [gnome]
    plugs:
      - audio-playback
      - audio-record
      - pulseaudio
      - desktop
      - desktop-legacy
      - wayland
      - x11
      - opengl

parts:
  fxsound-linux:
    plugin: nil
    source: .
    build-packages:
      - curl
      - wget
      - file
      - libssl-dev
      - libgtk-3-dev
      - libwebkit2gtk-4.0-dev
      - librsvg2-dev
      - libpulse-dev
    override-build: |
      # Install Node.js
      curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
      apt-get install -y nodejs
      
      # Install Rust
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
      source $HOME/.cargo/env
      
      # Build
      npm install
      npm run tauri:build
      
      # Install
      mkdir -p $SNAPCRAFT_PART_INSTALL/bin
      cp src-tauri/target/release/fxsound-linux $SNAPCRAFT_PART_INSTALL/bin/
```

2. **Build and Publish**

```bash
# Install snapcraft
sudo snap install snapcraft --classic

# Build snap
snapcraft

# Test locally
sudo snap install fxsound-linux_1.0.0_amd64.snap --dangerous

# Login to Snap Store
snapcraft login

# Upload
snapcraft upload fxsound-linux_1.0.0_amd64.snap --release=stable
```

**Register your snap:** https://snapcraft.io/register

---

## AUR (Arch Linux - 100% Free)

### Setup

1. **Create PKGBUILD**

Create `PKGBUILD`:

```bash
# Maintainer: Your Name <your.email@example.com>
pkgname=fxsound-linux
pkgver=1.0.0
pkgrel=1
pkgdesc="Audio enhancer with 10-band EQ and effects"
arch=('x86_64')
url="https://github.com/YOUR_USERNAME/fxsound-linux"
license=('MIT')
depends=('webkit2gtk' 'gtk3' 'libpulse')
makedepends=('rust' 'cargo' 'npm' 'git')
source=("$pkgname-$pkgver.tar.gz::https://github.com/YOUR_USERNAME/fxsound-linux/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$pkgname-$pkgver"
    npm install
    npm run tauri:build
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 "src-tauri/target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 "src-tauri/icons/icon.png" "$pkgdir/usr/share/pixmaps/$pkgname.png"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
```

2. **Submit to AUR**

```bash
# Create AUR account at https://aur.archlinux.org

# Clone AUR repo
git clone ssh://aur@aur.archlinux.org/fxsound-linux.git
cd fxsound-linux

# Add PKGBUILD and .SRCINFO
cp /path/to/PKGBUILD .
makepkg --printsrcinfo > .SRCINFO

# Commit and push
git add PKGBUILD .SRCINFO
git commit -m "Initial commit: fxsound-linux 1.0.0"
git push
```

**Guide:** https://wiki.archlinux.org/title/AUR_submission_guidelines

---

## Personal Website Hosting (100% Free)

### Option 1: GitHub Pages

1. **Create `docs/index.html`**:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>FXSound Linux - Download</title>
    <style>
        body { font-family: system-ui; max-width: 800px; margin: 50px auto; padding: 20px; }
        .download-btn { display: inline-block; padding: 15px 30px; background: #e63462; color: white; text-decoration: none; border-radius: 8px; margin: 10px; }
        .download-btn:hover { background: #d51535; }
    </style>
</head>
<body>
    <h1>FXSound Linux</h1>
    <p>Free, open-source audio enhancer for Linux</p>
    
    <h2>Download v1.0.0</h2>
    
    <a href="https://github.com/YOUR_USERNAME/fxsound-linux/releases/download/v1.0.0/fxsound-linux_1.0.0_amd64.AppImage" class="download-btn">
        Download AppImage (Universal)
    </a>
    
    <a href="https://github.com/YOUR_USERNAME/fxsound-linux/releases/download/v1.0.0/fxsound-linux_1.0.0_amd64.deb" class="download-btn">
        Download DEB (Ubuntu/Debian)
    </a>
    
    <a href="https://github.com/YOUR_USERNAME/fxsound-linux/releases/download/v1.0.0/fxsound-linux-1.0.0-1.x86_64.rpm" class="download-btn">
        Download RPM (Fedora/RHEL)
    </a>
    
    <h2>Installation</h2>
    <pre><code># AppImage
chmod +x fxsound-linux_1.0.0_amd64.AppImage
./fxsound-linux_1.0.0_amd64.AppImage

# Debian/Ubuntu
sudo dpkg -i fxsound-linux_1.0.0_amd64.deb

# Fedora/RHEL
sudo rpm -i fxsound-linux-1.0.0-1.x86_64.rpm</code></pre>
</body>
</html>
```

2. **Enable GitHub Pages**:
   - Go to repo Settings → Pages
   - Source: Deploy from branch → `main` → `/docs`
   - Your site: `https://YOUR_USERNAME.github.io/fxsound-linux`

### Option 2: Netlify (Free)

```bash
# Install Netlify CLI
npm install -g netlify-cli

# Deploy
netlify deploy --prod --dir=docs
```

### Option 3: Vercel (Free)

```bash
# Install Vercel CLI
npm install -g vercel

# Deploy
vercel --prod
```

---

## Auto-Update Setup

### Update Tauri Config

Edit `src-tauri/tauri.conf.json`:

```json
{
  "tauri": {
    "updater": {
      "active": true,
      "endpoints": [
        "https://github.com/YOUR_USERNAME/fxsound-linux/releases/latest/download/latest.json"
      ],
      "dialog": true,
      "pubkey": "YOUR_PUBLIC_KEY_HERE"
    }
  }
}
```

### Generate Update Keys

```bash
# Generate keypair
npm run tauri:signer:generate

# Add public key to tauri.conf.json
# Keep private key secret (add to .gitignore)
```

### Create Update JSON

Create `.github/workflows/updater.yml`:

```yaml
name: Update JSON

on:
  release:
    types: [published]

jobs:
  update-json:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Create update JSON
        run: |
          cat > latest.json << EOF
          {
            "version": "${{ github.event.release.tag_name }}",
            "notes": "${{ github.event.release.body }}",
            "pub_date": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
            "platforms": {
              "linux-x86_64": {
                "signature": "",
                "url": "https://github.com/${{ github.repository }}/releases/download/${{ github.event.release.tag_name }}/fxsound-linux_${{ github.event.release.tag_name }}_amd64.AppImage"
              }
            }
          }
          EOF
      
      - name: Upload to release
        uses: softprops/action-gh-release@v1
        with:
          files: latest.json
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

---

## Quick Start Checklist

- [ ] Build release binaries (`npm run tauri:build`)
- [ ] Test all formats (AppImage, DEB, RPM)
- [ ] Create GitHub repository
- [ ] Push code to GitHub
- [ ] Create release workflow (`.github/workflows/release.yml`)
- [ ] Tag version and push (`git tag v1.0.0 && git push origin v1.0.0`)
- [ ] Update README with download links
- [ ] (Optional) Submit to Flathub
- [ ] (Optional) Submit to Snap Store
- [ ] (Optional) Submit to AUR
- [ ] (Optional) Setup GitHub Pages for website

---

## Support & Distribution Channels

| Channel | Cost | Reach | Difficulty |
|---------|------|-------|------------|
| GitHub Releases | Free | High | Easy |
| Flathub | Free | Very High | Medium |
| Snap Store | Free | High | Medium |
| AUR | Free | Medium (Arch users) | Easy |
| GitHub Pages | Free | Medium | Easy |

**Recommended:** Start with **GitHub Releases** (easiest), then add **Flathub** (widest reach).

---

## Need Help?

- Tauri Docs: https://tauri.app/v1/guides/distribution/
- Flathub Docs: https://docs.flathub.org/
- Snap Docs: https://snapcraft.io/docs
- AUR Wiki: https://wiki.archlinux.org/title/AUR
