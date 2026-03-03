#!/bin/bash
# Complete local testing guide for FXSound Linux

set -e

echo "🧪 FXSound Linux - Local Testing Guide"
echo "========================================"
echo ""

# Step 1: Check system
echo "Step 1: Checking system requirements..."
echo ""

if ! command -v node &> /dev/null; then
    echo "❌ Node.js not installed"
    echo "Install: https://nodejs.org/"
    exit 1
fi
echo "✅ Node.js $(node --version)"

if ! command -v npm &> /dev/null; then
    echo "❌ npm not installed"
    exit 1
fi
echo "✅ npm $(npm --version)"

if ! command -v rustc &> /dev/null; then
    echo "❌ Rust not installed"
    echo "Install: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi
echo "✅ Rust $(rustc --version | awk '{print $2}')"

if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not installed"
    exit 1
fi
echo "✅ Cargo $(cargo --version | awk '{print $2}')"

echo ""

# Step 2: Check PulseAudio
echo "Step 2: Checking PulseAudio..."
echo ""

if ! pulseaudio --check; then
    echo "⚠️  PulseAudio not running"
    echo "Starting PulseAudio..."
    pulseaudio --start
    sleep 2
fi

if pulseaudio --check; then
    echo "✅ PulseAudio is running"
else
    echo "❌ PulseAudio failed to start"
    exit 1
fi

echo ""

# Step 3: Check dependencies
echo "Step 3: Checking system dependencies..."
echo ""

if pkg-config --exists libpulse; then
    echo "✅ libpulse-dev installed"
else
    echo "❌ libpulse-dev not found"
    echo "Run: ./setup-deps.sh"
    exit 1
fi

if pkg-config --exists webkit2gtk-4.1; then
    echo "✅ webkit2gtk-4.1 installed"
else
    echo "❌ webkit2gtk-4.1 not found"
    echo "Run: ./setup-deps.sh"
    exit 1
fi

echo ""

# Step 4: Check Node modules
echo "Step 4: Checking Node dependencies..."
echo ""

if [ ! -d "node_modules" ]; then
    echo "⚠️  Node modules not installed"
    echo "Installing..."
    npm install
fi
echo "✅ Node modules installed"

echo ""

# Step 5: List audio devices
echo "Step 5: Available audio devices:"
echo ""
pactl list sinks short | awk '{print "  • " $2}'

echo ""
echo "========================================"
echo "✅ System ready for testing!"
echo ""
echo "Next steps:"
echo ""
echo "  1. Run development mode:"
echo "     npm run tauri:dev"
echo ""
echo "  2. Test the app:"
echo "     - Play some music (YouTube, Spotify, etc.)"
echo "     - Adjust EQ sliders"
echo "     - Toggle effects"
echo "     - Watch visualizer"
echo "     - Toggle power button"
echo ""
echo "  3. Check logs in terminal for:"
echo "     [INFO] PulseAudio streams created successfully"
echo "     [INFO] Audio processor started successfully"
echo ""
echo "  4. Build production version:"
echo "     npm run tauri:build"
echo ""
