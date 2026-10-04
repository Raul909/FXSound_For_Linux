#!/bin/bash
# Quick test script for FXSound audio processing

echo "🎵 FXSound Audio Test"
echo "===================="
echo ""

# Check the audio server. Most distros now run PipeWire with its PulseAudio
# compatibility layer, which `pulseaudio --check` does not detect — and
# starting a real PulseAudio daemon alongside it would break audio.
echo "1. Checking the audio server..."
if pactl info >/dev/null 2>&1; then
    echo "   ✅ $(pactl info | sed -n 's/^Server Name: //p')"
else
    echo "   ❌ No PulseAudio/PipeWire server answering"
    echo "   PipeWire: systemctl --user restart pipewire pipewire-pulse wireplumber"
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
