import React, { useEffect, useRef, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";

const BAR_COUNT = 32;
const CANVAS_HEIGHT = 120;

/**
 * Real-time audio spectrum visualizer (canvas-based).
 *
 * Uses a <canvas> instead of 40 DOM divs to eliminate per-frame
 * object allocation and layout thrashing.
 * Wrapped in React.memo to prevent unnecessary re-renders.
 *
 * Props:
 *   powered — when false, bars flatten and capture stops
 */
const Visualizer = React.memo(function Visualizer({ powered }) {
    const canvasRef = useRef(null);
    const targetData = useRef(new Float32Array(BAR_COUNT).fill(2));
    const displayData = useRef(new Float32Array(BAR_COUNT).fill(2));
    const animFrameRef = useRef(null);
    const audioCtxRef = useRef(null);
    const analyserRef = useRef(null);
    const streamRef = useRef(null);
    const poweredRef = useRef(powered);

    // Cache the CanvasGradient to avoid recreating it every frame
    const gradientCacheRef = useRef(null);
    const lastCanvasWidthRef = useRef(0);

    // Keep poweredRef in sync without re-running the main effect
    useEffect(() => { poweredRef.current = powered; }, [powered]);

    const cleanupWebAudio = useCallback(() => {
        if (streamRef.current) {
            streamRef.current.getTracks().forEach(t => t.stop());
            streamRef.current = null;
        }
        if (audioCtxRef.current && audioCtxRef.current.state !== "closed") {
            audioCtxRef.current.close().catch(() => {});
            audioCtxRef.current = null;
        }
        analyserRef.current = null;
    }, []);

    // Create or retrieve the cached red gradient for bars
    const getBarGradient = useCallback((ctx, h) => {
        const w = ctx.canvas.width;
        if (gradientCacheRef.current && lastCanvasWidthRef.current === w) {
            return gradientCacheRef.current;
        }
        const grad = ctx.createLinearGradient(0, h, 0, 0);
        grad.addColorStop(0, "#6b0f20");     // Deep dark red at base
        grad.addColorStop(0.3, "#a11d38");   // Rich red
        grad.addColorStop(0.6, "#e63462");   // FXSound accent red
        grad.addColorStop(0.85, "#f7546f");  // Lighter red
        grad.addColorStop(1, "#ff7a8a");     // Bright tip
        gradientCacheRef.current = grad;
        lastCanvasWidthRef.current = w;
        return grad;
    }, []);

    // Draw bars onto the canvas — no React state, no DOM updates
    const drawFrame = useCallback((now, lastTimeRef) => {
        const canvas = canvasRef.current;
        if (!canvas) return;
        const ctx = canvas.getContext("2d");
        const w = canvas.width;
        const h = CANVAS_HEIGHT;

        const dt = Math.min((now - lastTimeRef.current) / 1000, 0.1);
        lastTimeRef.current = now;

        const isPowered = poweredRef.current;
        const target = targetData.current;
        const display = displayData.current;

        let needsDraw = false;

        // Interpolate display toward target
        for (let i = 0; i < BAR_COUNT; i++) {
            const t = target[i] || (isPowered ? 0 : 2);
            const speed = t > display[i] ? 14 : 5;
            display[i] += (t - display[i]) * Math.min(speed * dt, 1);
            if (Math.abs(display[i] - t) > 0.1) {
                needsDraw = true;
            }
        }

        if (!isPowered && !needsDraw) {
            return; // Skip rendering when bypassed and bars have settled
        }

        if (!needsDraw) return; // Skip clear and draw if nothing changed

        ctx.clearRect(0, 0, w, h);

        // Draw subtle horizontal baseline
        ctx.strokeStyle = "rgba(230, 52, 98, 0.12)";
        ctx.lineWidth = 1;
        ctx.beginPath();
        ctx.moveTo(0, h - 1);
        ctx.lineTo(w, h - 1);
        ctx.stroke();

        const gap = 3;
        const barW = (w - gap * (BAR_COUNT - 1)) / BAR_COUNT;

        // Get cached gradient
        const barGrad = isPowered ? getBarGradient(ctx, h) : null;

        for (let i = 0; i < BAR_COUNT; i++) {
            const barH = isPowered ? Math.max(2, display[i] * 1.0) : 2;
            const x = i * (barW + gap);
            const y = h - barH;

            if (!isPowered) {
                ctx.fillStyle = "#1e1e2a";
                ctx.globalAlpha = 0.3;
                ctx.beginPath();
                ctx.roundRect(x, y, barW, barH, [1, 1, 0, 0]);
                ctx.fill();
                ctx.globalAlpha = 1;
                continue;
            }

            const intensity = Math.min(barH / 70, 1);

            ctx.globalAlpha = 0.5 + intensity * 0.5;
            ctx.fillStyle = barGrad;
            ctx.beginPath();
            ctx.roundRect(x, y, barW, barH, [3, 3, 0, 0]);
            ctx.fill();

            // Top glow highlight on taller bars
            if (barH > 8) {
                ctx.globalAlpha = intensity * 0.4;
                ctx.fillStyle = "#ff8a9a";
                ctx.fillRect(x + 1, y, barW - 2, 2);
            }

            // Reflection below baseline
            if (barH > 4) {
                const refH = Math.max(1, barH * 0.2);
                ctx.globalAlpha = 0.08 + intensity * 0.04;
                ctx.fillStyle = barGrad;
                ctx.fillRect(x, h, barW, refH);
            }
        }
        ctx.globalAlpha = 1;
    }, [getBarGradient]);

    useEffect(() => {
        let cancelled = false;
        let pollTimeout = null;
        const lastTimeRef = { current: performance.now() };

        async function tryBackend() {
            try {
                const data = await invoke("get_visualizer_data");
                if (data && data.some(v => v > 1)) return true;
            } catch { /* ignore */ }
            return false;
        }

        async function tryWebAudio() {
            try {
                const stream = await navigator.mediaDevices.getDisplayMedia({ audio: true, video: true });
                stream.getVideoTracks().forEach(t => t.stop());
                if (!stream.getAudioTracks().length) return false;
                streamRef.current = stream;
                const actx = new (window.AudioContext || window.webkitAudioContext)();
                const src = actx.createMediaStreamSource(stream);
                const analyser = actx.createAnalyser();
                analyser.fftSize = 1024;
                analyser.smoothingTimeConstant = 0.75;
                src.connect(analyser);
                audioCtxRef.current = actx;
                analyserRef.current = analyser;
                return true;
            } catch { return false; }
        }

        async function init() {
            if (cancelled) return;

            if (await tryBackend()) {
                // Use recursive setTimeout instead of setInterval to prevent overlapping async calls
                function pollBackend() {
                    if (cancelled) return;
                    if (!poweredRef.current) {
                        pollTimeout = setTimeout(pollBackend, 50);
                        return;
                    }
                    invoke("get_visualizer_data")
                        .then((data) => {
                            const src = targetData.current;
                            const ratio = data.length / BAR_COUNT;
                            for (let i = 0; i < BAR_COUNT; i++) {
                                const si = Math.floor(i * ratio);
                                const ni = Math.min(si + 1, data.length - 1);
                                const f = (i * ratio) - si;
                                src[i] = data[si] * (1 - f) + data[ni] * f;
                            }
                        })
                        .catch(() => { /* ignore */ })
                        .finally(() => {
                            if (!cancelled) {
                                pollTimeout = setTimeout(pollBackend, 50);
                            }
                        });
                }
                pollBackend();
                return;
            }

            if (await tryWebAudio()) {
                const freqData = new Uint8Array(analyserRef.current.frequencyBinCount);
                const totalBins = freqData.length; // 512

                // Precompute FFT bin boundaries to avoid running Math.pow every frame
                const binBoundaries = new Uint16Array(BAR_COUNT + 1);
                binBoundaries[0] = 1;
                let currentLow = 1;
                for (let i = 0; i < BAR_COUNT; i++) {
                    const nextIndex = 1.0 + Math.pow((i + 1) / BAR_COUNT, 1.8) * (totalBins - 1);
                    let binHigh = Math.round(nextIndex);
                    if (binHigh <= currentLow) {
                        binHigh = currentLow + 1;
                    }
                    binHigh = Math.min(binHigh, totalBins);
                    binBoundaries[i + 1] = binHigh;
                    currentLow = binHigh;
                }

                function readAnalyser() {
                    if (cancelled || !analyserRef.current) return;
                    if (!poweredRef.current) {
                        requestAnimationFrame(readAnalyser);
                        return;
                    }
                    analyserRef.current.getByteFrequencyData(freqData);
                    const tgt = targetData.current;

                    for (let i = 0; i < BAR_COUNT; i++) {
                        const binLow = binBoundaries[i];
                        const binHigh = binBoundaries[i + 1];

                        let maxVal = 0;
                        let sumVal = 0;
                        const count = binHigh - binLow;

                        for (let bin = binLow; bin < binHigh; bin++) {
                            const val = freqData[bin];
                            if (val > maxVal) maxVal = val;
                            sumVal += val;
                        }

                        const avgVal = sumVal / count;
                        // Blend average (30%) and peak (70%), normalized to 0-100
                        const normalized = ((avgVal * 0.3 + maxVal * 0.7) / 255) * 100;

                        tgt[i] = normalized;
                    }
                    requestAnimationFrame(readAnalyser);
                }
                readAnalyser();
                return;
            }

            // Idle animation
            let phase = 0;
            function idleAnimation() {
                if (cancelled) return;
                phase += 0.08;
                if (poweredRef.current) {
                    const tgt = targetData.current;
                    for (let i = 0; i < BAR_COUNT; i++) {
                        tgt[i] = (Math.sin(phase + i * 0.3) * 0.5 + 0.5 +
                                   Math.sin(phase * 1.3 + i * 0.2) * 0.3 + 0.3) * 35 + 5;
                    }
                }
                pollTimeout = setTimeout(idleAnimation, 50);
            }
            idleAnimation();
        }

        init();

        function loop(now) {
            if (cancelled) return;
            drawFrame(now, lastTimeRef);
            animFrameRef.current = requestAnimationFrame(loop);
        }
        animFrameRef.current = requestAnimationFrame(loop);

        return () => {
            cancelled = true;
            if (pollTimeout) clearTimeout(pollTimeout);
            if (animFrameRef.current) cancelAnimationFrame(animFrameRef.current);
            cleanupWebAudio();
        };
    }, [drawFrame, cleanupWebAudio]);

    return (
        <div className="visualizer" aria-hidden="true">
            <canvas
                ref={canvasRef}
                width={456}
                height={CANVAS_HEIGHT}
                style={{ display: "block", width: "100%", height: `${CANVAS_HEIGHT}px` }}
            />
        </div>
    );
});

export default Visualizer;
