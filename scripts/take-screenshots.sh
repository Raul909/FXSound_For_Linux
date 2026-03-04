#!/bin/bash

# Screenshot capture script for FXSound Linux
# This script helps you take proper screenshots of the app

echo "📸 FXSound Linux Screenshot Helper"
echo "=================================="
echo ""
echo "Instructions:"
echo "1. Make sure the app is running (npm run tauri:dev)"
echo "2. Position the window nicely on your screen"
echo "3. Use one of these methods to take screenshots:"
echo ""
echo "Method 1: Using GNOME Screenshot (Ubuntu/Fedora)"
echo "  - Press PrtScn or use: gnome-screenshot -w"
echo "  - Click on the FXSound window"
echo ""
echo "Method 2: Using Spectacle (KDE)"
echo "  - Press Shift+PrtScn"
echo "  - Select 'Active Window'"
echo ""
echo "Method 3: Using Flameshot"
echo "  - Run: flameshot gui"
echo "  - Select the window area"
echo ""
echo "Method 4: Using scrot"
echo "  - Run: scrot -u 'screenshot.png' -d 3"
echo "  - Click on the FXSound window within 3 seconds"
echo ""
echo "Screenshots needed:"
echo "  1. Equalizer tab (with some sliders adjusted)"
echo "  2. Effects tab (with some effects enabled)"
echo ""
echo "Save them as:"
echo "  - public/screenshots/screenshot-eq.png"
echo "  - public/screenshots/screenshot-effects.png"
echo ""
echo "Recommended settings for screenshots:"
echo "  - Power: ON"
echo "  - Preset: Music (for EQ) / Movies (for Effects)"
echo "  - Make sure visualizer is showing activity"
echo ""

# Check if common screenshot tools are available
echo "Checking available screenshot tools..."
echo ""

if command -v gnome-screenshot &> /dev/null; then
    echo "✓ gnome-screenshot is available"
    echo "  Run: gnome-screenshot -w"
fi

if command -v spectacle &> /dev/null; then
    echo "✓ spectacle is available"
    echo "  Run: spectacle -a"
fi

if command -v flameshot &> /dev/null; then
    echo "✓ flameshot is available"
    echo "  Run: flameshot gui"
fi

if command -v scrot &> /dev/null; then
    echo "✓ scrot is available"
    echo "  Run: scrot -u 'screenshot.png' -d 3"
fi

echo ""
echo "After taking screenshots, move them to:"
echo "  $(pwd)/public/screenshots/"
