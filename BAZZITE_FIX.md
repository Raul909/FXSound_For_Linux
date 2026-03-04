# 🎮 Quick Fix for Bazzite Users

## The Issue

Bazzite is an **immutable distro** - system packages require a reboot to take effect.

## Quick Solution

### Step 1: Install Dependencies

```bash
./scripts/bazzite-setup.sh
```

This will install:
- PulseAudio development libraries
- WebKit2GTK development libraries  
- Build tools (gcc, make, pkg-config)

### Step 2: Reboot

```bash
sudo systemctl reboot
```

### Step 3: After Reboot

```bash
# Install Node dependencies
npm install

# Run the app
npm run tauri:dev
```

---

## Alternative: Use Distrobox (No Reboot Needed)

If you don't want to reboot:

```bash
# Create Ubuntu container
distrobox create --name fxsound --image ubuntu:22.04

# Enter container
distrobox enter fxsound

# Inside container:
sudo apt update
sudo apt install -y libpulse-dev libwebkit2gtk-4.1-dev build-essential nodejs npm

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Run the app
npm install
npm run tauri:dev
```

---

## Why This Happens

Bazzite uses `rpm-ostree` for system packages, which:
- ✅ Makes your system more stable
- ✅ Allows atomic updates
- ⚠️ Requires reboot for changes

This is normal for immutable distros like Bazzite, Fedora Silverblue, etc.

---

## Need Help?

See full guide: [docs/BAZZITE_SETUP.md](./docs/BAZZITE_SETUP.md)
