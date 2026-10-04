#!/bin/bash
# Put your audio back to normal if FXSound could not do it itself.
#
# FXSound adds an "FXSound" output device while it runs and removes it when it
# quits. If it is killed outright (or crashes), that device can be left as the
# default output with nothing listening to it, so you hear no sound. Starting
# FXSound again also cleans this up; this script does it without FXSound.
#
# Usage: ./scripts/reset-audio.sh [output-device-name]
#   With no argument, the first output device that is not FXSound's is used.

set -euo pipefail

if ! pactl info >/dev/null 2>&1; then
    echo "❌ No PulseAudio/PipeWire server answering (is pactl installed?)"
    exit 1
fi

TARGET="${1:-$(pactl list short sinks | awk '$2 != "fxsound_sink" && $2 != "auto_null" {print $2; exit}')}"

# Unload FXSound's device, plus any loopback early development builds wired
# into it (scripts/setup-audio.sh in 1.1.x).
MODULES=$(pactl list short modules | awk '/sink_name=fxsound_sink/ || (/module-loopback/ && /fxsound_sink/) {print $1}')
if [ -z "$MODULES" ]; then
    echo "✓ No FXSound device present."
else
    for m in $MODULES; do
        pactl unload-module "$m" && echo "✓ Removed FXSound module #$m"
    done
fi

if [ -n "$TARGET" ]; then
    pactl set-default-sink "$TARGET"
    echo "✓ Default output: $TARGET"
else
    echo "⚠️  No other output device found; choose one in your sound settings."
fi
