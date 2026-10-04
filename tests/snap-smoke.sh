#!/bin/bash
# Install a freshly built snap and check that, under strict confinement, it
# starts without the EGL abort, creates the FXSound device and routes audio.
# CI only: it installs snaps and starts its own audio session.
# Usage: tests/snap-smoke.sh path/to/fxsound-linux_*.snap
set -u
SNAP_FILE="$1"
HERE="$(cd "$(dirname "$0")" && pwd)"
SHOTS="${SHOTS:-$PWD/snap-smoke}"
mkdir -p "$SHOTS"
PASS=0; FAIL=0
expect() { local what="$1"; shift; if "$@"; then echo "  PASS: $what"; PASS=$((PASS + 1)); else echo "  FAIL: $what"; FAIL=$((FAIL + 1)); fi; }
wait_for() { local t=$1; shift; for _ in $(seq 1 $((t * 10))); do "$@" && return 0; sleep 0.1; done; return 1; }

sudo snap install --dangerous "$SNAP_FILE"
# Content snaps from the gnome extension (installed automatically from the
# store for published snaps; a local install needs them explicitly).
sudo snap install gnome-42-2204 gtk-common-themes >/dev/null 2>&1 || true
sudo snap connect fxsound-linux:gnome-42-2204 gnome-42-2204 2>/dev/null || true
sudo snap connect fxsound-linux:gtk-3-themes gtk-common-themes:gtk-3-themes 2>/dev/null || true
sudo snap connect fxsound-linux:icon-themes gtk-common-themes:icon-themes 2>/dev/null || true
# Recording is what FXSound does; audio-record is not connected automatically.
sudo snap connect fxsound-linux:audio-record
snap connections fxsound-linux

export FX_RUNTIME_DIR="/run/user/$(id -u)"
sudo mkdir -p "$FX_RUNTIME_DIR"; sudo chown "$(id -u):$(id -g)" "$FX_RUNTIME_DIR"
cd "$HERE/audio-routing"
source ./session.sh pw || exit 1
python3 analyze.py gen /tmp/probe.wav

echo "=== launching the snap"
snap run fxsound-linux >"$SHOTS/snap-app.log" 2>&1 &
APP=$!
expect "routes system audio within 120 s" wait_for 120 grep -q "Routing system audio through FXSound" "$SHOTS/snap-app.log"
sleep 3
expect "no EGL/Mesa abort" sh -c "! grep -qE 'libEGL fatal|DRI_Mesa|EGL_BAD|Aborting' '$SHOTS/snap-app.log'"
expect "still running" kill -0 "$APP"
expect "FXSound device is the system default" [ "$(pactl get-default-sink)" = fxsound_sink ]
import -window root "$SHOTS/snap-window.png" 2>/dev/null || true
OUT=$(./probe.sh hw_speakers --json | tail -1)
echo "  probe: $OUT"
expect "speakers receive exactly one copy" [ "$(echo "$OUT" | python3 -c 'import json,sys; print(json.load(sys.stdin)["n_arrivals"])')" -eq 1 ]
kill -TERM "$APP"
expect "exits cleanly on SIGTERM" wait_for 10 sh -c "! kill -0 $APP 2>/dev/null"
expect "default output restored" [ "$(pactl get-default-sink)" = hw_speakers ]

echo "=== snap log"; grep -vE "dbind-WARNING" "$SHOTS/snap-app.log" | tail -30
echo "RESULT (snap): $PASS passed, $FAIL failed"
[ "$FAIL" -eq 0 ]
