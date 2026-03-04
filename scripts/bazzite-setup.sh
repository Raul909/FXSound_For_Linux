#!/bin/bash
# Quick fix for Bazzite - Install missing development libraries

echo "🔧 Installing development libraries for Bazzite..."
echo ""

# Check if already installed
if pkg-config --exists libpulse webkit2gtk-4.1; then
    echo "✅ All dependencies already installed!"
    exit 0
fi

echo "Installing packages via rpm-ostree..."
echo ""

rpm-ostree install \
    pulseaudio-libs-devel \
    webkit2gtk4.1-devel \
    gcc \
    gcc-c++ \
    make \
    pkg-config

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Packages queued for installation"
    echo ""
    echo "⚠️  IMPORTANT: You must REBOOT for changes to take effect"
    echo ""
    echo "After reboot, run:"
    echo "  npm install"
    echo "  npm run tauri:dev"
    echo ""
    echo "Reboot now? (y/n)"
    read -r response
    if [[ "$response" =~ ^[Yy]$ ]]; then
        sudo systemctl reboot
    fi
else
    echo ""
    echo "❌ Installation failed"
    echo ""
    echo "Try manually:"
    echo "  rpm-ostree install pulseaudio-libs-devel webkit2gtk4.1-devel"
    echo "  sudo systemctl reboot"
fi
