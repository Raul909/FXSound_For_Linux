#!/bin/bash
# Bring up a headless desktop-audio session: a D-Bus session bus, the audio
# server (PipeWire + WirePlumber + pipewire-pulse, or native PulseAudio), two
# fake "hardware" outputs, and an Xvfb display.
#
# Usage: source session.sh pw|pa
#
# Meant for throwaway environments only (a container or CI): it starts its own
# audio server and wipes its runtime directory.
if [ -z "${CI:-}" ] && [ ! -f /.dockerenv ] && [ "${FX_TEST_DISPOSABLE:-}" != 1 ]; then
    echo "session.sh starts its own audio server and wipes its runtime dir;" \
         "run it in a container or CI (or set FX_TEST_DISPOSABLE=1)." >&2
    return 1 2>/dev/null || exit 1
fi
set -u
MODE="${1:-pw}"

# FX_RUNTIME_DIR lets the snap test use the real /run/user/<uid>, the only
# place snap confinement lets an app reach the audio and D-Bus sockets.
export XDG_RUNTIME_DIR="${FX_RUNTIME_DIR:-/tmp/fx-xdg-runtime}"
mkdir -p "$XDG_RUNTIME_DIR" && find "$XDG_RUNTIME_DIR" -mindepth 1 -delete 2>/dev/null
chmod 700 "$XDG_RUNTIME_DIR"
export DBUS_SESSION_BUS_ADDRESS="unix:path=$XDG_RUNTIME_DIR/bus"
dbus-daemon --session --address="$DBUS_SESSION_BUS_ADDRESS" --nofork --nopidfile --syslog-only >/tmp/dbus.log 2>&1 &

if [ "$MODE" = "pw" ]; then
    pipewire >/tmp/pipewire.log 2>&1 &
    sleep 0.5
    wireplumber >/tmp/wireplumber.log 2>&1 &
    pipewire-pulse >/tmp/pipewire-pulse.log 2>&1 &
else
    # The distro's real default.pa: stream-restore, suspend-on-idle,
    # switch-on-connect, always-sink... exactly what a desktop user runs.
    pulseaudio --daemonize=no --exit-idle-time=-1 --log-target=file:/tmp/pulseaudio.log >/dev/null 2>&1 &
fi

for _ in $(seq 1 50); do pactl info >/dev/null 2>&1 && break; sleep 0.2; done
pactl info | grep -E "Server Name|Server Version"

# Fake sound cards. Real ALSA devices are not available in a container; a null
# sink behaves like an output device and its monitor lets us record exactly
# what "the speakers" would have played. Under PipeWire they are native nodes
# (like real ALSA devices they outlive a pipewire-pulse restart); under
# PulseAudio they are null-sink modules.
add_hw() { # <name> <description>
    if [ "$MODE" = "pw" ]; then
        pw-cli create-node adapter "{ factory.name=support.null-audio-sink node.name=$1 node.description=$2 media.class=Audio/Sink object.linger=true audio.position=[FL FR] }" >/dev/null
    else
        pactl load-module module-null-sink sink_name="$1" sink_properties=device.description="$2" >/dev/null
    fi
    for _ in $(seq 1 30); do pactl list short sinks | awk '{print $2}' | grep -qx "$1" && return 0; sleep 0.1; done
}
remove_hw() { # <name>  ("unplug")
    if [ "$MODE" = "pw" ]; then
        pw-cli destroy "$(pw-dump | python3 -c "import json,sys; print(next(o['id'] for o in json.load(sys.stdin) if o.get('info',{}).get('props',{}).get('node.name')=='$1'))")" >/dev/null
    else
        pactl unload-module "$(pactl list short modules | awk -v n="sink_name=$1" 'index($0, n){print $1; exit}')"
    fi
}
add_hw hw_speakers Speakers
add_hw hw_headphones Headphones
# Until WirePlumber has published its "default" metadata, pipewire-pulse
# answers set-default-sink with "Not supported" and the default stays
# wherever WirePlumber put it — so retry until it has actually taken.
for _ in $(seq 1 50); do
    pactl set-default-sink hw_speakers 2>/dev/null
    [ "$(pactl get-default-sink)" = hw_speakers ] && break
    sleep 0.2
done
echo "default sink: $(pactl get-default-sink)"
[ "$(pactl get-default-sink)" = hw_speakers ] || { echo "could not set the default sink" >&2; return 1 2>/dev/null || exit 1; }
pactl list short sinks

Xvfb :99 -screen 0 1280x900x24 >/tmp/xvfb.log 2>&1 &
export DISPLAY=:99
sleep 0.5
