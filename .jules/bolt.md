## 2026-05-02 - Eliminate HashMap lookup overhead in DSP loop
**Learning:** In real-time audio processing loops (like `apply_effects` in `src-tauri/src/audio.rs`), reading values from a dynamic `HashMap` introduces unnecessary lookup overhead.
**Action:** Replace `HashMap` with a dedicated struct (e.g., `AudioEffects`) to store effect intensities, enabling fast direct field access in the DSP pipeline.
