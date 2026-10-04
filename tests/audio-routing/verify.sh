#!/bin/bash
# End-to-end checks of FXSound's audio routing against a real audio server:
# no doubling, no feedback, device changes, unplugging, a second launch,
# clean exit, crash recovery, server restarts and a frozen server.
#
# Usage: verify.sh pw|pa     (APP_BIN overrides the binary under test)
# See README.md; run it in a container or CI, never on your desktop session.
HERE="$(cd "$(dirname "$0")" && pwd)"
cd "$HERE"
MODE="${1:-pw}"
source ./session.sh "$MODE" || exit 1
python3 analyze.py gen /tmp/probe.wav
APP_BIN="${APP_BIN:-$HERE/../../src-tauri/target/debug/fxsound-linux}"
[ -x "$APP_BIN" ] || { echo "No app binary at $APP_BIN (see README.md)" >&2; exit 1; }
: > /tmp/app.log
PASS=0; FAIL=0
pass() { echo "  PASS: $*"; PASS=$((PASS + 1)); }
fail() { echo "  FAIL: $*"; FAIL=$((FAIL + 1)); }
expect() { local what="$1"; shift; if "$@"; then pass "$what"; else fail "$what"; fi; }

start_app() { "$APP_BIN" >>/tmp/app.log 2>&1 & APP=$!; }
wait_for() { # <seconds> <command...>: poll until the command succeeds
    local t=$1; shift
    for _ in $(seq 1 $((t * 10))); do "$@" && return 0; sleep 0.1; done; return 1
}
log_count() { grep -c "$1" /tmp/app.log; }
routing_count() { log_count "Routing system audio through FXSound"; }
default_is() { [ "$(pactl get-default-sink)" = "$1" ]; }
sink_count() { pactl list short sinks | awk '{print $2}' | grep -cx "$1"; }
no_sink() { [ "$(sink_count "$1")" -eq 0 ]; }
fx_output() {
    local idx
    idx=$(pactl list sink-inputs | awk '/^Sink Input #/{s=""} /^\tSink: /{s=$2} /media.name = "FXSound Output"/{print s; exit}')
    pactl list short sinks | awk -v i="$idx" '$1==i{print $2}'
}
fx_capture() {
    local idx
    idx=$(pactl list source-outputs | awk '/^Source Output #/{s=""} /^\tSource: /{s=$2} /media.name = "FXSound Capture"/{print s; exit}')
    pactl list short sources | awk -v i="$idx" '$1==i{print $2}'
}
output_is() { [ "$(fx_output)" = "$1" ]; }
probe_json() { ./probe.sh "$1" --json | tail -1; }
arrivals() { probe_json "$1" | python3 -c "import json,sys; print(json.load(sys.stdin)['n_arrivals'])"; }
probe_report() { # <sink>: prints arrivals/gain/tail and returns them in globals
    local j; j=$(probe_json "$1")
    N=$(echo "$j" | python3 -c "import json,sys; print(json.load(sys.stdin)['n_arrivals'])")
    GAIN=$(echo "$j" | python3 -c "import json,sys; a=json.load(sys.stdin)['arrivals']; print(a[0]['gain_db'] if a else 'none')")
    TAIL=$(echo "$j" | python3 -c "import json,sys; print(json.load(sys.stdin)['tail_rms_db'])")
    echo "    $1: $N arrival(s), first at ${GAIN} dB, level after the sound ends ${TAIL} dBFS"
}
# Sample where the UI (main) thread sleeps, without pausing the process the
# way attaching gdb would. A healthy GTK main loop idles in poll; the 1.1.4
# hang sat in a futex inside pa_threaded_mainloop_wait.
main_thread_free() {
    local w seen=""
    for _ in $(seq 1 20); do
        w=$(cat "/proc/$APP/task/$APP/wchan" 2>/dev/null)
        seen="$seen $w"
        case "$w" in *poll*) echo "    main thread sleeps in: $w"; return 0 ;; esac
        sleep 0.05
    done
    echo "    main thread never seen polling:$seen"
    return 1
}
# FXSound's playback queue (ms), as the server reports it.
queue_ms() {
    pactl list sink-inputs | python3 -c "
import sys,re
for b in sys.stdin.read().split('Sink Input #'):
    if 'FXSound Output' in b:
        print(int(re.search(r'Buffer Latency: (\d+)',b).group(1))//1000); break
else: print(99999)"
}
# PulseAudio null sinks (our fake hardware) wake up with a large first block
# that leaves a temporary backlog, and parecord starts late on them; let the
# output settle before probing, as a listener would.
settle_output() {
    [ "$MODE" = "pa" ] || return 0
    wait_for 12 bash -c '[ "$(queue_ms)" -le 100 ]' || echo "    (queue still $(queue_ms) ms)"
    sleep 0.5
}
export -f queue_ms
module_of() { pactl list short modules | awk -v n="sink_name=$1" 'index($0, n){print $1; exit}'; }

echo; echo "=== 1. Launch"
start_app
expect "FXSound routes system audio within 10 s" wait_for 10 sh -c '[ "$(grep -c "Routing system audio through FXSound" /tmp/app.log)" -ge 1 ]'
sleep 1
expect "UI main thread is free (not blocked on the audio server)" main_thread_free
expect "FXSound device is the system default" default_is fxsound_sink
expect "exactly one FXSound device exists" [ "$(sink_count fxsound_sink)" -eq 1 ]
expect "FXSound records its own device's monitor" [ "$(fx_capture)" = "fxsound_sink.monitor" ]
expect "FXSound plays to the speakers" output_is hw_speakers
./graph.sh

echo; echo "=== 2. No doubling, no feedback"
probe_report hw_speakers
expect "speakers receive exactly one copy of the sound" [ "$N" -eq 1 ]
expect "nothing keeps sounding after it ends (no feedback)" python3 -c "import sys; sys.exit(0 if $TAIL < -60 else 1)"
expect "headphones stay silent" [ "$(arrivals hw_headphones)" -eq 0 ]

echo; echo "=== 3. User picks Headphones in the system sound settings"
pactl set-default-sink hw_headphones
expect "FXSound follows to the headphones" wait_for 5 output_is hw_headphones
expect "FXSound puts itself back as the default" wait_for 5 default_is fxsound_sink
settle_output
probe_report hw_headphones
expect "headphones receive exactly one copy" [ "$N" -eq 1 ]
expect "speakers are now silent" [ "$(arrivals hw_speakers)" -eq 0 ]

echo; echo "=== 4. Headphones unplugged, then plugged back in"
remove_hw hw_headphones
expect "FXSound falls back to the speakers" wait_for 5 output_is hw_speakers
settle_output
probe_report hw_speakers
expect "speakers receive exactly one copy" [ "$N" -eq 1 ]
add_hw hw_headphones Headphones
expect "FXSound returns to the chosen headphones when they come back" wait_for 5 output_is hw_headphones
expect "FXSound is still the default" default_is fxsound_sink

echo; echo "=== 5. Launching a second copy"
"$APP_BIN" >/tmp/app2.log 2>&1 &
SECOND=$!
expect "second copy exits within 5 s" wait_for 5 sh -c "! kill -0 $SECOND 2>/dev/null"
expect "second copy reports the running one" grep -q "already running" /tmp/app2.log
expect "still exactly one FXSound device" [ "$(sink_count fxsound_sink)" -eq 1 ]
expect "first copy still routes audio" output_is hw_headphones

echo; echo "=== 6. Clean exit (SIGTERM)"
kill -TERM "$APP"
expect "FXSound exits within 5 s" wait_for 5 sh -c "! kill -0 $APP 2>/dev/null"
expect "FXSound device removed" no_sink fxsound_sink
expect "default restored to the device FXSound was playing to" default_is hw_headphones
if [ "$MODE" = "pw" ]; then
    probe_report hw_headphones
    expect "audio plays directly again, unprocessed" python3 -c "import sys; sys.exit(0 if $N == 1 and abs($GAIN) < 0.5 else 1)"
else
    # PulseAudio's null sink does not capture a direct paplay faithfully even
    # when FXSound never ran (harness limitation), so check the routing itself.
    paplay --latency-msec=20 /tmp/probe.wav & P=$!; sleep 0.5
    SINK_IDX=$(pactl list sink-inputs | awk '/^Sink Input #/{s=""} /^\tSink: /{s=$2} /application.name = "paplay"/{print s; exit}')
    wait $P
    expect "apps play directly to the restored device again" [ "$(pactl list short sinks | awk -v i="$SINK_IDX" '$1==i{print $2}')" = "hw_headphones" ]
fi
SETTINGS="$HOME/.config/com.fxsound.linux/settings.json"
expect "settings were saved" test -f "$SETTINGS"
expect "chosen output device remembered" grep -q '"output_device": "hw_headphones"' "$SETTINGS"

echo; echo "=== 7. Crash, then relaunch"
pactl set-default-sink hw_speakers
start_app
wait_for 10 sh -c '[ "$(grep -c "Routing system audio through FXSound" /tmp/app.log)" -ge 2 ]'
kill -KILL "$APP"; sleep 1
expect "(after SIGKILL the leftover device is still there, as expected)" [ "$(sink_count fxsound_sink)" -eq 1 ]
start_app
expect "relaunch cleans up and routes again within 10 s" wait_for 10 sh -c '[ "$(grep -c "Routing system audio through FXSound" /tmp/app.log)" -ge 3 ]'
sleep 1
expect "exactly one FXSound device after recovery" [ "$(sink_count fxsound_sink)" -eq 1 ]
OUT=$(fx_output)
probe_report "$OUT"
expect "audio flows again ($OUT), exactly one copy" [ "$N" -eq 1 ]

echo; echo "=== 8. Audio server restarts underneath FXSound"
BEFORE=$(routing_count)
if [ "$MODE" = "pw" ]; then
    pkill -x pipewire-pulse; sleep 0.5; pipewire-pulse >>/tmp/pipewire-pulse.log 2>&1 &
else
    pulseaudio -k 2>/dev/null; sleep 0.5
    pulseaudio --daemonize=no --exit-idle-time=-1 --log-target=file:/tmp/pulseaudio.log >/dev/null 2>&1 &
    wait_for 5 pactl info >/dev/null
    # A restarted PulseAudio re-detects its sound cards; recreate ours.
    add_hw hw_speakers Speakers
    add_hw hw_headphones Headphones
fi
expect "FXSound reconnects within 20 s" wait_for 20 sh -c "[ \$(grep -c 'Routing system audio through FXSound' /tmp/app.log) -gt $BEFORE ]"
sleep 1
expect "exactly one FXSound device after reconnect" [ "$(sink_count fxsound_sink)" -eq 1 ]
OUT=$(fx_output)
probe_report "${OUT:-hw_speakers}"
expect "audio flows after reconnect, exactly one copy" [ "$N" -eq 1 ]

echo; echo "=== 9. Audio server hangs (stopped) while FXSound starts"
kill -TERM "$APP"; wait_for 5 sh -c "! kill -0 $APP 2>/dev/null"
SERVER=$( [ "$MODE" = "pw" ] && pgrep -x pipewire-pulse || pgrep -x pulseaudio )
kill -STOP $SERVER
BEFORE=$(routing_count)
start_app
sleep 7
expect "app is alive while the server is frozen" kill -0 "$APP"
expect "UI main thread stays free while the server is frozen" main_thread_free
expect "a timeout is reported instead of hanging" grep -q "timed out waiting for" /tmp/app.log
kill -CONT $SERVER
expect "recovers once the server responds (within 25 s)" wait_for 25 sh -c "[ \$(grep -c 'Routing system audio through FXSound' /tmp/app.log) -gt $BEFORE ]"
kill -TERM "$APP"; wait_for 5 sh -c "! kill -0 $APP 2>/dev/null"
expect "clean exit leaves no FXSound device" no_sink fxsound_sink

echo; echo "=== app log (warnings and routing events)"
grep -E "WARN|ERROR|Routing|Playing to|Removing leftover|default changed|Received signal|skipping" /tmp/app.log | sed 's/^/  /' | tail -40
echo; echo "RESULT ($MODE): $PASS passed, $FAIL failed"
[ "$FAIL" -eq 0 ]
