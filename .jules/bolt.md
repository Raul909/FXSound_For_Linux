
## 2026-05-05 - Add early return to DSP loop
**Learning:** Even when inner processing loops are avoided, iterating over a large array (like an audio buffer) still incurs O(N) overhead for enumeration, variable assignments, and modulo arithmetic.
**Action:** Always verify if stateful loops can be bypassed entirely with an early return when there is no work to perform (e.g., flat EQ state).
