import { useState, useEffect, useRef, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";

const BAR_COUNT = 40;

/**
 * Real-time audio spectrum visualizer.
 *
 * Attempts three audio capture strategies in order:
 *   1. Tauri backend FFT data (system audio via PulseAudio)
 *   2. Web Audio API via screen/tab audio capture (getDisplayMedia)
 *   3. Animated idle visualization when no audio source is available
 *
 * Props:
 *   powered — when false, bars flatten and capture stops
 */
export default function Visualizer({ powered }) {
    const [displayData, setDisplayData] = useState(new Array(BAR_COUNT).fill(2));
    const targetData = useRef(new Array(BAR_COUNT).fill(2));
    const animFrameRef = useRef(null);
    const sourceRef = useRef(null); // "backend" | "webaudio" | "idle"
    const audioCtxRef = useRef(null);
    const analyserRef = useRef(null);
    const streamRef = useRef(null);

    // Clean up Web Audio resources
    const cleanupWebAudio = useCallback(() => {
        if (streamRef.current) {
            streamRef.current.getTracks().forEach(t => t.stop());
            streamRef.current = null;
        }
        if (audioCtxRef.current && audioCtxRef.current.state !== "closed") {
            audioCtxRef.current.close().catch(() => { });
            audioCtxRef.current = null;
        }
        analyserRef.current = null;
    }, []);

    useEffect(() => {
        let cancelled = false;
        let pollTimeout = null;

        if (!powered) {
            // eslint-disable-next-line react-hooks/set-state-in-effect
            setDisplayData(new Array(BAR_COUNT).fill(2));
            targetData.current = new Array(BAR_COUNT).fill(2);
            cleanupWebAudio();
            // Start a simple animation to reset the bars
            let lastTime = performance.now();
            function animateReset(now) {
                if (cancelled) return;
                const dt = Math.min((now - lastTime) / 1000, 0.1);
                lastTime = now;
                let done = true;
                setDisplayData(prev => {
                    const next = new Array(prev.length);
                    for (let i = 0; i < prev.length; i++) {
                        const target = targetData.current[i] || 2;
                        const speed = 14;
                        const diff = target - prev[i];
                        if (Math.abs(diff) > 0.1) {
                            done = false;
                        }
                        next[i] = prev[i] + diff * Math.min(speed * dt, 1);
                    }
                    return next;
                });
                if (!done) {
                    animFrameRef.current = requestAnimationFrame(animateReset);
                }
            }
            animFrameRef.current = requestAnimationFrame(animateReset);
            return () => {
                cancelled = true;
                if (animFrameRef.current) cancelAnimationFrame(animFrameRef.current);
            };
        }

        // Strategy 1: Try backend FFT data
        async function tryBackend() {
            try {
                const data = await invoke("get_visualizer_data");
                if (data && data.some(v => v > 1)) {
                    sourceRef.current = "backend";
                    return true;
                }
            } catch { /* ignore */ }
            return false;
        }

        // Strategy 2: Try Web Audio API capture
        async function tryWebAudio() {
            try {
                // Try to capture desktop/tab audio
                const stream = await navigator.mediaDevices.getDisplayMedia({
                    audio: true,
                    video: true // required by some browsers to get audio
                });

                // Stop video tracks immediately — we only need audio
                stream.getVideoTracks().forEach(t => t.stop());

                const audioTracks = stream.getAudioTracks();
                if (audioTracks.length === 0) return false;

                streamRef.current = stream;
                const ctx = new (window.AudioContext || window.webkitAudioContext)();
                const source = ctx.createMediaStreamSource(stream);
                const analyser = ctx.createAnalyser();
                analyser.fftSize = 128;
                analyser.smoothingTimeConstant = 0.75;
                source.connect(analyser);

                audioCtxRef.current = ctx;
                analyserRef.current = analyser;
                sourceRef.current = "webaudio";
                return true;
            } catch {
                return false;
            }
        }

        // Initialize: try each strategy
        async function init() {
            if (cancelled) return;

            const backendOk = await tryBackend();
            if (backendOk && !cancelled) {
                // Poll backend at ~20fps
                const pollBackend = async () => {
                    if (cancelled) return;
                    try {
                        const data = await invoke("get_visualizer_data");
                        // Resample 32 bins → BAR_COUNT
                        const resampled = resampleData(data, BAR_COUNT);
                        targetData.current = resampled;
                    } catch { /* ignore */ }
                    if (!cancelled) {
                        pollTimeout = setTimeout(pollBackend, 50);
                    }
                };
                pollTimeout = setTimeout(pollBackend, 50);
                return;
            }

            // Web Audio attempt — user will see a dialog to share audio
            const webOk = await tryWebAudio();
            if (webOk && !cancelled) {
                // Read analyser data on each animation frame
                const freqData = new Uint8Array(analyserRef.current.frequencyBinCount);
                function readAnalyser() {
                    if (cancelled || !analyserRef.current) return;
                    analyserRef.current.getByteFrequencyData(freqData);
                    const resampled = resampleUint8(freqData, BAR_COUNT);
                    targetData.current = resampled;
                    requestAnimationFrame(readAnalyser);
                }
                readAnalyser();
                return;
            }

            // Fallback: animated idle mode
            if (!cancelled) {
                sourceRef.current = "idle";
                let phase = 0;
                const pollIdle = () => {
                    if (cancelled) return;
                    phase += 0.08;
                    const idle = new Array(BAR_COUNT);
                    for (let i = 0; i < BAR_COUNT; i++) {
                        const wave = Math.sin(phase + i * 0.3) * 0.5 + 0.5;
                        const wave2 = Math.sin(phase * 1.3 + i * 0.2) * 0.3 + 0.3;
                        idle[i] = (wave + wave2) * 35 + 5;
                    }
                    targetData.current = idle;
                    if (!cancelled) {
                        pollTimeout = setTimeout(pollIdle, 50);
                    }
                };
                pollTimeout = setTimeout(pollIdle, 50);
            }
        }

        init();

        // Smooth interpolation at 60fps
        let lastTime = performance.now();
        function animate(now) {
            if (cancelled) return;
            const dt = Math.min((now - lastTime) / 1000, 0.1);
            lastTime = now;

            setDisplayData(prev => {
                const next = new Array(prev.length);
                for (let i = 0; i < prev.length; i++) {
                    const target = targetData.current[i] || 0;
                    const speed = target > prev[i] ? 14 : 5;
                    next[i] = prev[i] + (target - prev[i]) * Math.min(speed * dt, 1);
                }
                return next;
            });

            animFrameRef.current = requestAnimationFrame(animate);
        }
        animFrameRef.current = requestAnimationFrame(animate);

        return () => {
            cancelled = true;
            if (pollTimeout) clearTimeout(pollTimeout);
            if (animFrameRef.current) cancelAnimationFrame(animFrameRef.current);
            cleanupWebAudio();
        };
    }, [powered, cleanupWebAudio]);

    return (
        <div className="visualizer">
            {displayData.map((magnitude, index) => {
                const barHeight = powered ? Math.max(2, magnitude * 0.85) : 2;
                const intensity = Math.min(barHeight / 55, 1);
                // Color shifts from deep red at edges to bright pink-white in center
                const center = BAR_COUNT / 2;
                const distFromCenter = Math.abs(index - center) / center;
                const hue = 340 + distFromCenter * 15; // 340-355 range
                const saturation = 75 + (1 - distFromCenter) * 25;
                const lightness = 45 + intensity * 20;

                return (
                    <div
                        key={index}
                        className={`visualizer__bar ${powered ? "visualizer__bar--active" : ""}`}
                        style={{
                            height: `${barHeight}px`,
                            background: powered
                                ? `linear-gradient(to top, hsl(${hue}, ${saturation}%, ${lightness - 15}%), hsl(${hue}, ${saturation}%, ${lightness}%))`
                                : undefined,
                            boxShadow: powered && barHeight > 6
                                ? `0 0 ${3 + intensity * 10}px hsla(${hue}, 85%, 55%, ${0.15 + intensity * 0.45})`
                                : "none",
                            opacity: powered ? 0.55 + intensity * 0.45 : 0.3,
                        }}
                    />
                );
            })}
            {/* Reflection layer */}
            {powered && (
                <div className="visualizer__reflection">
                    {displayData.map((magnitude, index) => {
                        const barHeight = Math.max(1, magnitude * 0.25);
                        return (
                            <div
                                key={index}
                                className="visualizer__bar visualizer__bar--reflection"
                                style={{
                                    height: `${barHeight}px`,
                                    opacity: 0.15,
                                }}
                            />
                        );
                    })}
                </div>
            )}
        </div>
    );
}

// Resample f32 data (from backend, typically 32 bins) to target count
function resampleData(data, targetCount) {
    const result = new Array(targetCount);
    const ratio = data.length / targetCount;
    for (let i = 0; i < targetCount; i++) {
        const srcIdx = Math.floor(i * ratio);
        const nextIdx = Math.min(srcIdx + 1, data.length - 1);
        const frac = (i * ratio) - srcIdx;
        result[i] = data[srcIdx] * (1 - frac) + data[nextIdx] * frac;
    }
    return result;
}

// Convert Uint8Array frequency data (0-255) to float (0-100) and resample
function resampleUint8(data, targetCount) {
    const result = new Array(targetCount);
    const ratio = data.length / targetCount;
    for (let i = 0; i < targetCount; i++) {
        const srcIdx = Math.floor(i * ratio);
        const nextIdx = Math.min(srcIdx + 1, data.length - 1);
        const frac = (i * ratio) - srcIdx;
        const val = data[srcIdx] * (1 - frac) + data[nextIdx] * frac;
        result[i] = (val / 255) * 100;
    }
    return result;
}
