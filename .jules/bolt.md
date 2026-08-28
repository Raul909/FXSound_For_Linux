## 2024-07-11 - Optimized RMS silence detection
**Learning:** In audio processing loops, full-buffer RMS calculations for silence detection waste significant CPU when audio is playing.
**Action:** Use a running sum and short-circuit evaluation. By multiplying the threshold by `buffer.len()` instead of dividing the sum, you can break the loop the moment the sum exceeds the threshold. This turns an O(N) operation into O(1) for typical non-silent audio frames.
