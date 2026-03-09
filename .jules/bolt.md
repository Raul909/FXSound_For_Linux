
## 2024-03-09 - Cache FFT Planner and Buffer in AudioEngine
**Learning:** In the real-time audio processing loop (`src-tauri/src/audio.rs`), repeated initialization of `FftPlanner` and in-loop heap allocations (`Vec::collect()`) for the complex buffer introduced unnecessary overhead.
**Action:** Cache the planned FFT processor (`Arc<dyn rustfft::Fft<f32>>`) and pre-allocate the complex buffer (`Vec<Complex<f32>>`) inside the `AudioEngine` state to perform in-place updates.
