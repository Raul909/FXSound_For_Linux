## 2024-05-24 - [Optimize FFT Processing]
**Learning:** Re-instantiating `FftPlanner` and allocating new `Vec<Complex<f32>>` buffers within a real-time audio loop (e.g., in `src-tauri/src/audio.rs`) causes unnecessary heap allocations and negatively impacts processing latency.
**Action:** Cache the planned FFT processor (`Arc<dyn rustfft::Fft<f32>>`) and pre-allocate complex/scratch buffers within the processing engine struct (like `AudioEngine`). Use methods like `zip` or manual loops to update the pre-allocated buffers in-place.
