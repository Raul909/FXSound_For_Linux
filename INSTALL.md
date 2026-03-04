# Installation Guide

## For Users

### Quick Install

**AppImage (Recommended - Works on all distros)**
```bash
# Download
wget https://github.com/YOUR_USERNAME/fxsound-linux/releases/latest/download/fxsound-linux_1.0.0_amd64.AppImage

# Make executable
chmod +x fxsound-linux_1.0.0_amd64.AppImage

# Run
./fxsound-linux_1.0.0_amd64.AppImage
```

**Ubuntu/Debian/Pop!_OS/Linux Mint**
```bash
# Download
wget https://github.com/YOUR_USERNAME/fxsound-linux/releases/latest/download/fxsound-linux_1.0.0_amd64.deb

# Install
sudo dpkg -i fxsound-linux_1.0.0_amd64.deb

# Fix dependencies if needed
sudo apt-get install -f

# Run
fxsound-linux
```

**Fedora/RHEL/openSUSE**
```bash
# Download
wget https://github.com/YOUR_USERNAME/fxsound-linux/releases/latest/download/fxsound-linux-1.0.0-1.x86_64.rpm

# Install
sudo rpm -i fxsound-linux-1.0.0-1.x86_64.rpm

# Or with dnf (Fedora)
sudo dnf install fxsound-linux-1.0.0-1.x86_64.rpm

# Run
fxsound-linux
```

**Arch Linux (AUR)**
```bash
# Using yay
yay -S fxsound-linux

# Using paru
paru -S fxsound-linux

# Manual
git clone https://aur.archlinux.org/fxsound-linux.git
cd fxsound-linux
makepkg -si
```

**Flatpak (Universal)**
```bash
# Install from Flathub
flatpak install flathub com.fxsound.linux

# Run
flatpak run com.fxsound.linux
```

**Snap**
```bash
# Install
sudo snap install fxsound-linux

# Run
fxsound-linux
```

---

## System Requirements

- **OS:** Any Linux distribution with PulseAudio or PipeWire
- **Architecture:** x86_64 (64-bit)
- **RAM:** 100 MB minimum
- **Disk:** 50 MB

### Dependencies

Most dependencies are bundled, but you may need:

**Ubuntu/Debian:**
```bash
sudo apt install libpulse0 libwebkit2gtk-4.0-37 libgtk-3-0
```

**Fedora:**
```bash
sudo dnf install pulseaudio-libs webkit2gtk3 gtk3
```

**Arch:**
```bash
sudo pacman -S pulseaudio webkit2gtk gtk3
```

---

## First Run

1. **Start the app**
2. **Select your output device** from the dropdown
3. **Choose a preset** (Music, Movies, Gaming, etc.)
4. **Adjust EQ and effects** to your liking
5. **Play some audio** to see the visualizer

---

## Troubleshooting

### No audio output?

```bash
# Check PulseAudio is running
pulseaudio --check || pulseaudio --start

# List audio devices
pactl list sinks short

# Restart PulseAudio
pulseaudio -k && pulseaudio --start
```

### App won't start?

```bash
# Check dependencies (Ubuntu/Debian)
ldd /path/to/fxsound-linux

# Run from terminal to see errors
./fxsound-linux_1.0.0_amd64.AppImage
```

### Permission issues?

```bash
# Make sure you're in audio group
sudo usermod -aG audio $USER

# Log out and back in
```

---

## Uninstall

**AppImage:**
```bash
rm fxsound-linux_1.0.0_amd64.AppImage
```

**Debian/Ubuntu:**
```bash
sudo apt remove fxsound-linux
```

**Fedora/RHEL:**
```bash
sudo dnf remove fxsound-linux
```

**Arch:**
```bash
sudo pacman -R fxsound-linux
```

**Flatpak:**
```bash
flatpak uninstall com.fxsound.linux
```

**Snap:**
```bash
sudo snap remove fxsound-linux
```

---

## Build from Source

See [BUILD.md](./BUILD.md) for detailed build instructions.

Quick build:
```bash
git clone https://github.com/YOUR_USERNAME/fxsound-linux.git
cd fxsound-linux
./scripts/setup-deps.sh
npm install
npm run tauri:build
```

---

## Support

- **Issues:** https://github.com/YOUR_USERNAME/fxsound-linux/issues
- **Discussions:** https://github.com/YOUR_USERNAME/fxsound-linux/discussions
- **Documentation:** https://github.com/YOUR_USERNAME/fxsound-linux/wiki
