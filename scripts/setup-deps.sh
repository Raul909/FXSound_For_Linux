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

# Check if running in Flatpak/container
if [ "$DISTRO" = "org.freedesktop.platform" ] || [ -f /.flatpak-info ]; then
    echo "⚠️  Running inside Flatpak/container environment"
    echo ""
    echo "You need to install dependencies on your HOST system."
    echo ""
    echo "Exit this container and run on your host:"
    echo ""
    echo "  Ubuntu/Debian/Mint/Pop!_OS:"
    echo "    sudo apt install libpulse-dev libwebkit2gtk-4.1-dev build-essential"
    echo ""
    echo "  Fedora:"
    echo "    sudo dnf install pulseaudio-libs-devel webkit2gtk4.1-devel"
    echo ""
    echo "  Arch/Manjaro:"
    echo "    sudo pacman -S libpulse webkit2gtk-4.1 base-devel"
    echo ""
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
    fedora|bazzite)
        # For Bazzite/Fedora Atomic, use rpm-ostree
        if command -v rpm-ostree &> /dev/null; then
            echo "Detected Fedora Atomic/Bazzite - using rpm-ostree"
            rpm-ostree install \
                pulseaudio-libs-devel \
                webkit2gtk4.1-devel \
                openssl-devel \
                gcc \
                gcc-c++ \
                make \
                pkg-config
            echo ""
            echo "⚠️  System packages installed via rpm-ostree"
            echo "You need to REBOOT for changes to take effect:"
            echo "  sudo systemctl reboot"
        else
            # Regular Fedora
            sudo dnf install -y \
                pulseaudio-libs-devel \
                webkit2gtk4.1-devel \
                openssl-devel \
                curl \
                wget \
                file \
                libappindicator-gtk3-devel \
                librsvg2-devel
        fi
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
        echo ""
        echo "Please install these packages manually:"
        echo "  - PulseAudio development libraries (libpulse-dev)"
        echo "  - WebKit2GTK 4.1 development libraries"
        echo "  - Build tools (gcc, make, pkg-config)"
        echo "  - OpenSSL development libraries"
        echo ""
        exit 1
        ;;
esac

echo "✅ System dependencies installed!"
echo ""
echo "Next steps:"
echo "  1. Run: npm run tauri:dev"
echo "  2. Test the app"
echo "  3. Build: npm run tauri:build"
