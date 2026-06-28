## 2024-05-18 - Prevent unbound memory allocation in Tauri IPC commands
**Vulnerability:** The Tauri IPC command `apply_preset_state` accepted `Vec<f32>` and `HashMap<String, f32>`, allowing a malicious frontend to cause a Denial of Service via unbounded memory allocation (OOM).
**Learning:** Even if the command body checks sizes, `serde_json` allocates memory for the entire unbounded array/map before the body runs.
**Prevention:** Use fixed-size arrays (e.g., `[f32; 10]`) and strictly typed structs with `Option` fields (e.g., `PresetEffects`) instead of `Vec` and `HashMap` at the Tauri IPC boundary to ensure memory allocation is bounded during deserialization.
