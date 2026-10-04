#!/usr/bin/env python3
"""Generate a probe signal, or analyse what the speakers actually played.

  analyze.py gen   probe.wav           # 0.5 s white-noise burst + 2.5 s silence
  analyze.py check rec.raw [--json]    # rec.raw: float32le stereo 48 kHz

Every copy of the burst that reached the speakers shows up as a separate
cross-correlation peak, so the report tells apart:
  * one arrival             -> correct (processed audio only)
  * two arrivals ~0 + ~N ms -> doubling (original + FXSound copy)
  * a train of arrivals     -> feedback (FXSound re-capturing its own output)
"""
import json
import struct
import sys
import wave

import numpy as np

RATE = 48000
BURST_S = 0.5
TOTAL_S = 3.0
LEVEL = 0.1  # ~ -20 dBFS RMS


def burst():
    rng = np.random.default_rng(1234)
    return (rng.standard_normal(int(BURST_S * RATE)) * LEVEL).astype(np.float32)


def gen(path):
    b = burst()
    sig = np.zeros(int(TOTAL_S * RATE), dtype=np.float32)
    sig[: len(b)] = b
    stereo = np.repeat(sig[:, None], 2, axis=1)
    pcm = (np.clip(stereo, -1, 1) * 32767).astype("<i2")
    with wave.open(path, "wb") as w:
        w.setnchannels(2)
        w.setsampwidth(2)
        w.setframerate(RATE)
        w.writeframes(pcm.tobytes())


def db(x):
    return 20 * np.log10(max(x, 1e-12))


def check(path, as_json=False):
    raw = np.fromfile(path, dtype="<f4")
    if raw.size < 2 * RATE // 10:
        print(json.dumps({"error": "recording too short", "samples": int(raw.size)}))
        return 2
    rec = raw[: raw.size // 2 * 2].reshape(-1, 2).mean(axis=1)
    ref = burst()

    n = 1 << int(np.ceil(np.log2(len(rec) + len(ref))))
    xc = np.fft.irfft(np.fft.rfft(rec, n) * np.conj(np.fft.rfft(ref, n)), n)[: len(rec)]
    # Normalise so a single unprocessed, unity-gain copy of the burst scores 1.0.
    xc /= float(np.dot(ref, ref))
    env = np.abs(xc)

    peaks = []
    floor = max(env.max() * 0.08, 0.02)
    guard = int(0.004 * RATE)  # peaks closer than 4 ms are one arrival
    order = np.argsort(env)[::-1]
    for i in order:
        if env[i] < floor:
            break
        if any(abs(i - p) < guard for p, _ in peaks):
            continue
        peaks.append((int(i), float(xc[i])))
        if len(peaks) >= 16:
            break
    peaks.sort()

    total_rms = float(np.sqrt(np.mean(rec ** 2)))
    first = peaks[0][0] if peaks else 0
    tail_start = first + len(ref) + int(0.4 * RATE)
    tail = rec[tail_start:] if tail_start < len(rec) else np.zeros(1)
    tail_rms = float(np.sqrt(np.mean(tail ** 2))) if tail.size else 0.0

    report = {
        "arrivals": [
            {"lag_ms": round(i / RATE * 1000, 1), "gain": round(g, 3), "gain_db": round(db(abs(g)), 1)}
            for i, g in peaks
        ],
        "n_arrivals": len(peaks),
        "rec_rms_db": round(db(total_rms), 1),
        "tail_rms_db": round(db(tail_rms), 1),
        "peak_abs": round(float(np.abs(rec).max()), 3),
    }
    if as_json:
        print(json.dumps(report))
    else:
        print(f"  arrivals at speakers: {report['n_arrivals']}")
        for a in report["arrivals"]:
            print(f"    lag {a['lag_ms']:7.1f} ms  gain {a['gain']:+.3f} ({a['gain_db']:+.1f} dB)")
        print(f"  overall level {report['rec_rms_db']} dBFS, peak {report['peak_abs']}")
        print(f"  level 400 ms after the burst ended: {report['tail_rms_db']} dBFS")
    return 0


if __name__ == "__main__":
    if sys.argv[1] == "gen":
        gen(sys.argv[2])
    else:
        sys.exit(check(sys.argv[2], "--json" in sys.argv))
