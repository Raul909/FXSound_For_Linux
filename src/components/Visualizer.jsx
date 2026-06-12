import { useEffect, useRef, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";

const BAR_COUNT = 32;
const CANVAS_HEIGHT = 100;

/**
 * Real-time audio spectrum visualizer (canvas-based).
 *
 * Uses a <canvas> instead of 40 DOM divs to eliminate per-frame
 * object allocation and layout thrashing.
 *
 * Props:
 *   powered — when false, bars flatten and capture stops
 */
export default function Visualizer({ powered }) {
    const canvasRef = useRef(null);
    const targetData = useRef(new Float32Array(BAR_COUNT).fill(2));
    const displayData = useRef(new Float32Array(BAR_COUNT).fill(2));
    const animFrameRef = useRef(null);
    const audioCtxRef = useRef(null);
    const analyserRef = useRef(null);
    const streamRef = useRef(null);
    const poweredRef = useRef(powered);
    const stylesCacheRef = useRef(null);

    // Keep poweredRef in sync without re-running the main effect
    useEffect(() => { poweredRef.current = powered; }, [powered]);


    // Pre-calculate gradients and colors to avoid GC overhead in draw loop
    useEffect(() => {
        const canvas = canvasRef.current;
        if (!canvas) return;
        const ctx = canvas.getContext("2d");
        const center = BAR_COUNT / 2;
        const cache = new Array(BAR_COUNT);

        for (let i = 0; i < BAR_COUNT; i++) {
            cache[i] = new Array(CANVAS_HEIGHT + 1);
            const distFromCenter = Math.abs(i - center) / center;
            const hue = 340 + distFromCenter * 15;
            const sat = 75 + (1 - distFromCenter) * 25;

            for (let barH = 0; barH <= CANVAS_HEIGHT; barH++) {
                const intensity = Math.min(barH / 55, 1);
                const lit = 45 + intensity * 20;
                const y = CANVAS_HEIGHT - barH;

                const grad = ctx.createLinearGradient(0, y + barH, 0, y);
                grad.addColorStop(0, `hsl(${hue},${sat}%,${lit - 15}%)`);
                grad.addColorStop(1, `hsl(${hue},${sat}%,${lit}%)`);

                cache[i][barH] = {
                    grad,
                    alpha: 0.55 + intensity * 0.45,
                    reflectionColor: `hsl(${hue},${sat}%,${lit}%)`
                };
            }
        }
        stylesCacheRef.current = cache;
    }, []);

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

        // Interpolate display toward target
        for (let i = 0; i < BAR_COUNT; i++) {
            const t = target[i] || (isPowered ? 0 : 2);
            const speed = t > display[i] ? 14 : 5;
            display[i] += (t - display[i]) * Math.min(speed * dt, 1);
        }

        ctx.clearRect(0, 0, w, h);

        const gap = 2;
        const barW = (w - gap * (BAR_COUNT - 1)) / BAR_COUNT;
        const center = BAR_COUNT / 2;

        for (let i = 0; i < BAR_COUNT; i++) {
            const barH = isPowered ? Math.max(2, display[i] * 0.85) : 2;
            const x = i * (barW + gap);
            const y = h - barH;

            if (!isPowered) {
                ctx.fillStyle = "#1e1e2a";
                ctx.globalAlpha = 0.3;
                ctx.fillRect(x, y, barW, barH);
                ctx.globalAlpha = 1;
                continue;
            }

            const barHInt = Math.min(Math.max(Math.round(barH), 0), CANVAS_HEIGHT);
            const cached = stylesCacheRef.current?.[i]?.[barHInt];

            if (cached) {
                ctx.globalAlpha = cached.alpha;
                ctx.fillStyle = cached.grad;
                ctx.beginPath();
                ctx.roundRect(x, y, barW, barH, [2, 2, 0, 0]);
                ctx.fill();

                // Reflection
                if (barH > 3) {
                    const refH = Math.max(1, barH * 0.25);
                    ctx.globalAlpha = 0.12;
                    ctx.fillStyle = cached.reflectionColor;
                    ctx.fillRect(x, h, barW, refH);
                }
            } else {
                // Fallback for missing cache or custom heights
                const intensity = Math.min(barH / 55, 1);
                const distFromCenter = Math.abs(i - center) / center;
                const hue = 340 + distFromCenter * 15;
                const sat = 75 + (1 - distFromCenter) * 25;
                const lit = 45 + intensity * 20;

                const grad = ctx.createLinearGradient(0, y + barH, 0, y);
                grad.addColorStop(0, `hsl(${hue},${sat}%,${lit - 15}%)`);
                grad.addColorStop(1, `hsl(${hue},${sat}%,${lit}%)`);
                ctx.globalAlpha = 0.55 + intensity * 0.45;
                ctx.fillStyle = grad;
                ctx.beginPath();
                ctx.roundRect(x, y, barW, barH, [2, 2, 0, 0]);
                ctx.fill();

                if (barH > 3) {
                    const refH = Math.max(1, barH * 0.25);
                    ctx.globalAlpha = 0.12;
                    ctx.fillStyle = `hsl(${hue},${sat}%,${lit}%)`;
                    ctx.fillRect(x, h, barW, refH);
                }
            }
        }
        ctx.globalAlpha = 1;
    }, []);

    useEffect(() => {
        let cancelled = false;
        let pollInterval = null;
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
                analyser.fftSize = 64;
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
                pollInterval = setInterval(async () => {
                    try {
                        const data = await invoke("get_visualizer_data");
                        const src = targetData.current;
                        const ratio = data.length / BAR_COUNT;
                        for (let i = 0; i < BAR_COUNT; i++) {
                            const si = Math.floor(i * ratio);
                            const ni = Math.min(si + 1, data.length - 1);
                            const f = (i * ratio) - si;
                            src[i] = data[si] * (1 - f) + data[ni] * f;
                        }
                    } catch { /* ignore */ }
                }, 50);
                return;
            }

            if (await tryWebAudio()) {
                const freqData = new Uint8Array(analyserRef.current.frequencyBinCount);
                function readAnalyser() {
                    if (cancelled || !analyserRef.current) return;
                    analyserRef.current.getByteFrequencyData(freqData);
                    const ratio = freqData.length / BAR_COUNT;
                    const tgt = targetData.current;
                    for (let i = 0; i < BAR_COUNT; i++) {
                        const si = Math.floor(i * ratio);
                        const ni = Math.min(si + 1, freqData.length - 1);
                        const f = (i * ratio) - si;
                        tgt[i] = ((freqData[si] * (1 - f) + freqData[ni] * f) / 255) * 100;
                    }
                    requestAnimationFrame(readAnalyser);
                }
                readAnalyser();
                return;
            }

            // Idle animation
            let phase = 0;
            pollInterval = setInterval(() => {
                phase += 0.08;
                const tgt = targetData.current;
                for (let i = 0; i < BAR_COUNT; i++) {
                    tgt[i] = (Math.sin(phase + i * 0.3) * 0.5 + 0.5 +
                               Math.sin(phase * 1.3 + i * 0.2) * 0.3 + 0.3) * 35 + 5;
                }
            }, 50);
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
            if (pollInterval) clearInterval(pollInterval);
            if (animFrameRef.current) cancelAnimationFrame(animFrameRef.current);
            cleanupWebAudio();
        };
    }, [drawFrame, cleanupWebAudio]);

    return (
        <div className="visualizer">
            <canvas
                ref={canvasRef}
                width={456}
                height={CANVAS_HEIGHT}
                style={{ display: "block", width: "100%", height: `${CANVAS_HEIGHT}px` }}
            />
        </div>
    );
}
