## 2026-04-30 - Replace HashMap with struct for audio effects
**Learning:** In high-frequency real-time audio loops, using dynamic collections like `HashMap` for state configuration causes significant overhead due to constant hashing and heap lookups on every frame/sample.
**Action:** Replace `HashMap` with a dedicated struct containing explicit fields for fixed configuration items to enable fast, direct memory access.
