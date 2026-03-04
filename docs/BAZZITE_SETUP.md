# FXSound Linux on Bazzite/Fedora Atomic

## Important: Bazzite is an Immutable Distro

Bazzite uses `rpm-ostree` for system packages, which requires a reboot after installation.

## Option 1: Install System Dependencies (Recommended)

```bash
# Install required packages
rpm-ostree install \
    pulseaudio-libs-devel \
    webkit2gtk4.1-devel \
    openssl-devel \
    gcc \
    gcc-c++ \
    make \
    pkg-config

# Reboot to apply changes
sudo systemctl reboot
```

After reboot:
```bash
# Install Node dependencies
npm install

# Run the app
npm run tauri:dev
```

## Option 2: Use Toolbox/Distrobox (Alternative)

If you don't want to modify your base system, use a container:

```bash
# Create Ubuntu toolbox
distrobox create --name fxsound --image ubuntu:22.04

# Enter the toolbox
distrobox enter fxsound

# Inside the toolbox, install dependencies
sudo apt update
sudo apt install -y \
    libpulse-dev \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    nodejs \
    npm

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Navigate to project and run
cd /path/to/fxsound-app
npm install
npm run tauri:dev
```

## Option 3: Use Flatpak (Future)

Once FXSound is packaged as Flatpak, you can install it directly:

```bash
flatpak install flathub com.fxsound.linux
```

(Not yet available - coming soon)

## Troubleshooting

### "rpm-ostree: command not found"

You're not on Bazzite/Fedora Atomic. Use regular Fedora commands:
```bash
sudo dnf install pulseaudio-libs-devel webkit2gtk4.1-devel
```

### "Cannot access PulseAudio from container"

If using Distrobox, you need to export the PulseAudio socket:
```bash
distrobox create --name fxsound --image ubuntu:22.04 \
    --additional-flags "--volume /run/user/$UID/pulse:/run/user/$UID/pulse"
```

### Build works but no audio

Make sure PulseAudio is running on your HOST system:
```bash
pulseaudio --check || pulseaudio --start
```

## Recommended Approach for Bazzite

**For development:** Use Option 1 (rpm-ostree) - one-time reboot, then native performance

**For quick testing:** Use Option 2 (Distrobox) - no reboot needed, but slightly slower

**For end users:** Wait for Option 3 (Flatpak) - easiest installation
