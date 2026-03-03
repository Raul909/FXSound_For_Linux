#!/bin/bash
# Build FXSound Linux for distribution

set -e

echo "🔨 Building FXSound Linux..."
echo ""

# Check dependencies
echo "1. Checking dependencies..."
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust not installed"
    echo "Install: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

if ! pkg-config --exists libpulse; then
    echo "❌ PulseAudio dev libraries not found"
    echo "Run: ./setup-deps.sh"
    exit 1
fi

echo "✅ Dependencies OK"
echo ""

# Build
echo "2. Building Tauri app..."
npm run tauri:build

echo ""
echo "✅ Build complete!"
echo ""
echo "📦 Output files:"
echo ""

# List AppImage
if ls src-tauri/target/release/bundle/appimage/*.AppImage 1> /dev/null 2>&1; then
    echo "AppImage (universal):"
    ls -lh src-tauri/target/release/bundle/appimage/*.AppImage | awk '{print "  " $9 " (" $5 ")"}'
fi

# List Deb
if ls src-tauri/target/release/bundle/deb/*.deb 1> /dev/null 2>&1; then
    echo "Deb (Ubuntu/Debian):"
    ls -lh src-tauri/target/release/bundle/deb/*.deb | awk '{print "  " $9 " (" $5 ")"}'
fi

echo ""
echo "🚀 Next steps:"
echo "  1. Test the AppImage:"
echo "     chmod +x src-tauri/target/release/bundle/appimage/*.AppImage"
echo "     ./src-tauri/target/release/bundle/appimage/*.AppImage"
echo ""
echo "  2. Create GitHub release:"
echo "     - Go to: https://github.com/YOUR_USERNAME/fxsound-linux/releases/new"
echo "     - Tag: v0.1.0"
echo "     - Upload files from src-tauri/target/release/bundle/"
echo ""
