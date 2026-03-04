#!/bin/bash
# FXSound Audio Setup Script for PipeWire/PulseAudio

echo "🎧 FXSound Audio Setup"
echo "======================"
echo ""

# Check if PipeWire or PulseAudio is running
if pgrep -x pipewire > /dev/null; then
    echo "✓ PipeWire detected"
    AUDIO_SERVER="pipewire"
elif pgrep -x pulseaudio > /dev/null; then
    echo "✓ PulseAudio detected"
    AUDIO_SERVER="pulseaudio"
else
    echo "✗ No audio server detected"
    echo "Please start PipeWire or PulseAudio first"
    exit 1
fi

echo ""
echo "Setting up audio routing for FXSound..."
echo ""

# Get the default sink (output device)
DEFAULT_SINK=$(pactl get-default-sink)
echo "Default audio output: $DEFAULT_SINK"

# Create a null sink for FXSound
echo "Creating FXSound virtual sink..."
pactl load-module module-null-sink sink_name=fxsound_sink sink_properties=device.description="FXSound"

# Create loopback from default sink monitor to FXSound sink
echo "Creating audio loopback..."
pactl load-module module-loopback source="$DEFAULT_SINK.monitor" sink=fxsound_sink latency_msec=1

# Set FXSound sink as default
echo "Setting FXSound as default output..."
pactl set-default-sink fxsound_sink

echo ""
echo "✓ Audio setup complete!"
echo ""
echo "Now run FXSound app. To restore original audio:"
echo "  pactl set-default-sink $DEFAULT_SINK"
echo "  pactl unload-module module-null-sink"
echo "  pactl unload-module module-loopback"
