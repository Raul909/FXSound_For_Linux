#!/bin/bash
# Install dependencies for FXSound Linux (Tauri + PulseAudio)

echo "🔧 Installing system dependencies..."

# Detect distro
if [ -f /etc/os-release ]; then
    . /etc/os-release
    DISTRO=$ID
else
    echo "❌ Cannot detect Linux distribution"
    exit 1
fi

case $DISTRO in
    ubuntu|debian|pop|linuxmint)
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
        ;;
    fedora)
        sudo dnf install -y \
            pulseaudio-libs-devel \
            webkit2gtk4.1-devel \
            openssl-devel \
            curl \
            wget \
            file \
            libappindicator-gtk3-devel \
            librsvg2-devel
        ;;
    arch|manjaro)
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
        ;;
    *)
        echo "⚠️  Unsupported distro: $DISTRO"
        echo "Please install PulseAudio development libraries manually"
        exit 1
        ;;
esac

echo "✅ System dependencies installed!"
echo ""
echo "Next steps:"
echo "  1. Run: npm run tauri:dev"
echo "  2. Test the app"
echo "  3. Build: npm run tauri:build"
