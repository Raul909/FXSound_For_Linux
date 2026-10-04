#!/bin/bash
# Build FXSound and run the audio-routing suite in a throwaway container.
# Usage: tests/audio-routing/run-in-docker.sh [pw|pa]
# Needs the frontend built first (npm ci && npm run build).
set -euo pipefail
MODE="${1:-pw}"
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SERVER=$([ "$MODE" = "pa" ] && echo pulseaudio || echo pipewire)
[ -f "$ROOT/dist/index.html" ] || { echo "Build the frontend first: npm ci && npm run build" >&2; exit 1; }

docker build --build-arg SERVER="$SERVER" -t "fxsound-routing-$MODE" "$ROOT/tests/audio-routing"
docker volume create fxsound-routing-cargo >/dev/null
docker volume create "fxsound-routing-target-$MODE" >/dev/null
docker run --rm -u root -v fxsound-routing-cargo:/c -v "fxsound-routing-target-$MODE":/t \
    "fxsound-routing-$MODE" chown -R tester:tester /c /t
docker run --rm -v "$ROOT":/src \
    -v fxsound-routing-cargo:/home/tester/.cargo/registry \
    -v "fxsound-routing-target-$MODE":/home/tester/target \
    -e CARGO_TARGET_DIR=/home/tester/target \
    -e APP_BIN=/home/tester/target/debug/fxsound-linux \
    "fxsound-routing-$MODE" bash -c \
    "cd /src/src-tauri && cargo build --features tauri/custom-protocol && /src/tests/audio-routing/verify.sh $MODE"
