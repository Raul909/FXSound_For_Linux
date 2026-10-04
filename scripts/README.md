# Helper Scripts

## Setup & Testing

### setup-deps.sh
Install system dependencies (PulseAudio, webkit2gtk, etc.)

```bash
./scripts/setup-deps.sh
```

### test-locally.sh
Check if your system is ready for development

```bash
./scripts/test-locally.sh
```

### test-audio.sh
Check the audio server (PipeWire or PulseAudio) and list output devices

```bash
./scripts/test-audio.sh
```

### reset-audio.sh
Remove a leftover "FXSound" output device and restore your normal default
output — only needed if FXSound was killed or crashed (relaunching FXSound
cleans up too)

```bash
./scripts/reset-audio.sh [output-device-name]
```

### Audio-routing tests
End-to-end routing checks against real PipeWire and PulseAudio servers, run in
Docker — see [`tests/audio-routing/README.md`](../tests/audio-routing/README.md)

```bash
npm ci && npm run build && tests/audio-routing/run-in-docker.sh pw
```

## Building

### build-release.sh
Build production version (AppImage, Deb, RPM)

```bash
./scripts/build-release.sh
```

## Version Management

### verify-version.sh
Verify version consistency across all files

```bash
./scripts/verify-version.sh
```

### version-info.sh
Display current version information

```bash
./scripts/version-info.sh
```

## Documentation

### show-guide.sh
Display complete testing and distribution guide

```bash
./scripts/show-guide.sh
```

## Quick Reference

```bash
# Setup
./scripts/setup-deps.sh && npm install

# Test
./scripts/test-locally.sh

# Build
./scripts/build-release.sh

# Verify
./scripts/verify-version.sh
```
