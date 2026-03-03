#!/bin/bash
# Quick test script for FXSound audio processing

echo "🎵 FXSound Audio Test"
echo "===================="
echo ""

# Check PulseAudio
echo "1. Checking PulseAudio..."
if pulseaudio --check; then
    echo "   ✅ PulseAudio is running"
else
    echo "   ❌ PulseAudio not running"
    echo "   Starting PulseAudio..."
    pulseaudio --start
fi
echo ""

# Check dependencies
echo "2. Checking dependencies..."
if pkg-config --exists libpulse; then
    echo "   ✅ libpulse found"
else
    echo "   ❌ libpulse not found - run ./setup-deps.sh"
fi

if pkg-config --exists webkit2gtk-4.1; then
    echo "   ✅ webkit2gtk-4.1 found"
else
    echo "   ❌ webkit2gtk-4.1 not found - run ./setup-deps.sh"
fi
echo ""

# List audio devices
echo "3. Available audio devices:"
pactl list sinks short | awk '{print "   - " $2}'
echo ""

# Check if Rust is installed
echo "4. Checking Rust..."
if command -v rustc &> /dev/null; then
    echo "   ✅ Rust $(rustc --version | awk '{print $2}')"
else
    echo "   ❌ Rust not installed"
    echo "   Install: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
fi
echo ""

echo "===================="
echo "Ready to test!"
echo ""
echo "Run: npm run tauri:dev"
echo ""
echo "Then:"
echo "  1. Play some music"
echo "  2. Adjust EQ sliders"
echo "  3. Watch the visualizer"
echo "  4. Toggle effects"
echo ""
