#!/bin/bash
# Play the probe through the system default output (like any app) and record
# what the given fake sound card (default: hw_speakers) actually output.
REC_SINK="${1:-hw_speakers}"
cd "$(dirname "$0")"
[ -f /tmp/probe.wav ] || python3 analyze.py gen /tmp/probe.wav
parecord --latency-msec=20 --device="$REC_SINK.monitor" --format=float32le --rate=48000 --channels=2 --raw /tmp/rec.raw &
REC=$!
sleep 0.4
paplay --latency-msec=20 /tmp/probe.wav
sleep "${PROBE_TAIL:-0.6}"
kill $REC 2>/dev/null; wait $REC 2>/dev/null
python3 analyze.py check /tmp/rec.raw ${2:-}
