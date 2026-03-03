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
Test PulseAudio system and list audio devices

```bash
./scripts/test-audio.sh
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
