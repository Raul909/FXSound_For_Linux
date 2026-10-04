#!/bin/bash
# Install a freshly built snap and check that, under strict confinement, it
# starts without the EGL abort, creates the FXSound device and routes audio.
#
# CI only: it installs snaps and enables a lingering systemd user session for
# the current user. snapd places every app process in a systemd scope and
# needs the user's own systemd instance for that — CI job steps run inside a
# system service, where `snap run` refuses to start ("... is not a snap cgroup
# for tag ..."). So the snap is launched as a transient unit of that user
# session, and PipeWire comes from the same session, as on a desktop.
#
# Usage: tests/snap-smoke.sh path/to/fxsound-linux_*.snap
set -u
[ -n "${CI:-}" ] || { echo "snap-smoke.sh installs snaps and changes the user session; CI only." >&2; exit 1; }
SNAP_FILE="$1"
HERE="$(cd "$(dirname "$0")" && pwd)"
SHOTS="${SHOTS:-$PWD/snap-smoke}"
mkdir -p "$SHOTS"
LOG="$SHOTS/snap-app.log"
: >"$LOG"
PASS=0; FAIL=0
expect() { local what="$1"; shift; if "$@"; then echo "  PASS: $what"; PASS=$((PASS + 1)); else echo "  FAIL: $what"; FAIL=$((FAIL + 1)); fi; }
wait_for() { local t=$1; shift; for _ in $(seq 1 $((t * 10))); do "$@" >/dev/null 2>&1 && return 0; sleep 0.1; done; return 1; }

echo "=== installing"
sudo snap install --dangerous "$SNAP_FILE"
# Content snaps from the gnome extension: installed automatically from the
# store for published snaps, a local install needs them explicitly.
sudo snap install gnome-42-2204 gtk-common-themes >/dev/null 2>&1 || true
sudo snap connect fxsound-linux:gnome-42-2204 gnome-42-2204 2>/dev/null || true
sudo snap connect fxsound-linux:gtk-3-themes gtk-common-themes:gtk-3-themes 2>/dev/null || true
sudo snap connect fxsound-linux:icon-themes gtk-common-themes:icon-themes 2>/dev/null || true
snap connections fxsound-linux

echo "=== user session"
sudo loginctl enable-linger "$(id -un)"
export XDG_RUNTIME_DIR="/run/user/$(id -u)"
wait_for 30 test -S "$XDG_RUNTIME_DIR/bus" || { echo "no user session bus"; exit 1; }
export DBUS_SESSION_BUS_ADDRESS="unix:path=$XDG_RUNTIME_DIR/bus"
systemctl --user start pipewire.socket pipewire-pulse.socket wireplumber.service
wait_for 30 pactl info || { echo "no audio server"; systemctl --user status pipewire pipewire-pulse wireplumber --no-pager; exit 1; }
pactl info | grep -E "Server Name|Server Version"

# Fake sound cards, as in audio-routing/session.sh: native PipeWire null
# sinks whose monitors record exactly what they would play.
for hw in hw_speakers:Speakers hw_headphones:Headphones; do
    pw-cli create-node adapter "{ factory.name=support.null-audio-sink node.name=${hw%%:*} node.description=${hw##*:} media.class=Audio/Sink object.linger=true audio.position=[FL FR] }" >/dev/null
done
wait_for 10 sh -c 'pactl list short sinks | grep -q hw_headphones'
pactl set-default-sink hw_speakers
pactl list short sinks

Xvfb :99 -screen 0 1280x900x24 >/dev/null 2>&1 &
export DISPLAY=:99
python3 "$HERE/audio-routing/analyze.py" gen /tmp/probe.wav

running() { systemctl --user is-active --quiet fxsound-smoke; }
launch() {
    systemd-run --user --collect --unit=fxsound-smoke -E DISPLAY=:99 \
        -p StandardOutput="append:$LOG" -p StandardError="append:$LOG" \
        snap run fxsound-linux
}

# Phase 1, a fresh install: only the automatically connected plugs, so no
# permission to record. FXSound must stay up and say what to do.
echo "=== launching the snap without audio-record"
launch
sleep 25
expect "keeps running without recording permission" running
expect "does not take over the default output" [ "$(pactl get-default-sink)" = hw_speakers ]
echo "  what it reported:"; grep -E "WARN|ERROR|Routing" "$LOG" | tail -5 | sed 's/^/    /'
import -window root "$SHOTS/snap-window-no-record.png" 2>/dev/null || true
systemctl --user stop fxsound-smoke
wait_for 10 sh -c '! systemctl --user is-active --quiet fxsound-smoke'
expect "no FXSound device left behind" sh -c '! pactl list short sinks | grep -q fxsound_sink'
pactl set-default-sink hw_speakers
: >"$LOG"

# Phase 2: exactly what the docs tell users to run.
sudo snap connect fxsound-linux:audio-record
snap connections fxsound-linux | grep -E "audio-|pulseaudio"
echo "=== launching the snap with audio-record connected"
launch
fx_output() {
    local idx
    idx=$(pactl list sink-inputs | awk '/^Sink Input #/{s=""} /^\tSink: /{s=$2} /media.name = "FXSound Output"/{print s; exit}')
    pactl list short sinks | awk -v i="$idx" '$1==i{print $2}'
}
expect "routes system audio within 120 s" wait_for 120 grep -q "Routing system audio through FXSound" "$LOG"
sleep 3
expect "no EGL/Mesa abort" sh -c "! grep -qE 'libEGL fatal|DRI_Mesa|EGL_BAD|Aborting' '$LOG'"
expect "still running" running
expect "FXSound device is the system default" [ "$(pactl get-default-sink)" = fxsound_sink ]
expect "FXSound plays to the speakers" [ "$(fx_output)" = hw_speakers ]
import -window root "$SHOTS/snap-window.png" 2>/dev/null || true
OUT=$(cd "$HERE/audio-routing" && ./probe.sh hw_speakers --json | tail -1)
echo "  probe: $OUT"
expect "speakers receive exactly one copy, through FXSound" \
    [ "$(echo "$OUT" | python3 -c 'import json,sys; print(json.load(sys.stdin)["n_arrivals"])')" -eq 1 ]
systemctl --user stop fxsound-smoke
expect "exits cleanly when stopped" wait_for 10 sh -c '! systemctl --user is-active --quiet fxsound-smoke'
expect "default output restored" [ "$(pactl get-default-sink)" = hw_speakers ]
expect "FXSound device removed" sh -c '! pactl list short sinks | grep -q fxsound_sink'

echo "=== snap log"; grep -vE "dbind-WARNING" "$LOG" | tail -40
echo "RESULT (snap): $PASS passed, $FAIL failed"
[ "$FAIL" -eq 0 ]
