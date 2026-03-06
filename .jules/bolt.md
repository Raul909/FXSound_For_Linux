## 2024-05-24 - Pre-allocate and Cache FFT Processor
**Learning:** In hot audio processing loops (like `process_audio` calling `update_fft` frequently), initializing `rustfft::FftPlanner` and allocating new `Vec<Complex<f32>>` buffers for each invocation causes significant overhead and unnecessary heap allocations.
**Action:** Cache the planned FFT processor (`Arc<dyn Fft<f32>>`) and pre-allocate the complex scratch buffer within the persistent state (e.g., `AudioEngine` struct) to reuse them during runtime and eliminate intermediate allocations.
