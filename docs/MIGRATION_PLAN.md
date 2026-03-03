# FXSound Linux - Tauri Migration Plan

## Phase 1: Setup Tauri (1-2 hours)

```bash
# Install Tauri CLI
cargo install tauri-cli

# Initialize Tauri in existing project
npm install --save-dev @tauri-apps/cli
npm install @tauri-apps/api

# Create Tauri config
npx tauri init
```

**Config answers:**
- App name: `fxsound-linux`
- Window title: `FXSound for Linux`
- Web assets: `../dist`
- Dev server: `http://localhost:5173`
- Frontend dev command: `npm run dev`
- Frontend build command: `npm run build`

## Phase 2: Basic Rust Backend (2-3 hours)

### Dependencies needed:
```toml
# src-tauri/Cargo.toml
[dependencies]
tauri = { version = "1.5", features = ["shell-open"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
libpulse-binding = "2.28"  # PulseAudio
```

### Core commands:
```rust
// src-tauri/src/main.rs
#[tauri::command]
fn set_eq_band(band: usize, gain: f32) -> Result<(), String> {
    // Apply EQ to PulseAudio sink
}

#[tauri::command]
fn set_effect(effect: String, value: f32) -> Result<(), String> {
    // Apply effect processing
}

#[tauri::command]
fn get_audio_devices() -> Result<Vec<String>, String> {
    // List PulseAudio sinks
}
```

## Phase 3: React Integration (1 hour)

```javascript
// src/App.jsx
import { invoke } from '@tauri-apps/api/tauri';

// Replace state updates with Tauri commands
const updateEQBand = async (band, value) => {
  await invoke('set_eq_band', { band, gain: value });
  setEq(prev => { /* update UI */ });
};
```

## Phase 4: Audio Processing (4-6 hours)

Implement in Rust:
- PulseAudio sink creation
- 10-band parametric EQ (biquad filters)
- Effect processing (compression, reverb, etc.)
- Real-time audio routing

## Phase 5: System Integration (2-3 hours)

- System tray icon
- Auto-start on login
- Keyboard shortcuts
- Notifications

## Phase 6: Packaging (1 hour)

```bash
# Build AppImage
npm run tauri build -- --bundles appimage

# Build .deb
npm run tauri build -- --bundles deb

# Build .rpm
npm run tauri build -- --bundles rpm
```

---

## Total Time Estimate: 10-15 hours

## Learning Resources

- [Tauri Docs](https://tauri.app/v1/guides/)
- [libpulse-binding](https://docs.rs/libpulse-binding/)
- [Audio DSP in Rust](https://github.com/RustAudio)

## Next Steps

1. Run `npm install --save-dev @tauri-apps/cli`
2. Run `npx tauri init`
3. Start with Phase 2 (basic commands)
4. Test with simple gain control before full EQ
