import React, { useEffect, useRef, useCallback } from "react";
import { call } from "../tauri";

const BAR_COUNT = 32;
const CANVAS_HEIGHT = 120;
const POLL_INTERVAL_MS = 50;

// Canvas.roundRect is missing on WebKitGTK older than 2.40, which still ships
// on long-support distros. Calling it there throws on every animation frame and
// takes the whole visualizer down, so fall back to square bars instead.
const SUPPORTS_ROUND_RECT =
    typeof CanvasRenderingContext2D !== "undefined" &&
    typeof CanvasRenderingContext2D.prototype.roundRect === "function";

/**
 * Real-time audio spectrum visualizer (canvas-based).
 *
 * Bar heights come from the Rust engine's FFT of the processed signal, polled
 * over the Tauri bridge. Uses a <canvas> instead of 32 DOM nodes to eliminate
 * per-frame object allocation and layout thrashing.
 *
 * Props:
 *   powered — when false, bars fall to the floor and polling pauses
 */
const Visualizer = React.memo(function Visualizer({ powered }) {
    const canvasRef = useRef(null);
    const targetData = useRef(new Float32Array(BAR_COUNT));
    const displayData = useRef(new Float32Array(BAR_COUNT));
    const animFrameRef = useRef(null);
    const poweredRef = useRef(powered);
    // Cache the CanvasGradient to avoid recreating it every frame
    const gradientCacheRef = useRef(null);
    // Forces a repaint after something other than bar movement changed the
    // picture (power toggle, canvas resize). Without it the "nothing moved,
    // skip drawing" optimisation left stale bars on screen after power-off.
    const needsRepaintRef = useRef(true);

    // Keep poweredRef in sync without re-running the main effect
    useEffect(() => {
        poweredRef.current = powered;
        needsRepaintRef.current = true;
    }, [powered]);

    // Create or retrieve the cached red gradient for bars
    const getBarGradient = useCallback((ctx, h) => {
        if (gradientCacheRef.current) {
            return gradientCacheRef.current;
        }
        const grad = ctx.createLinearGradient(0, h, 0, 0);
        grad.addColorStop(0, "#6b0f20");     // Deep dark red at base
        grad.addColorStop(0.3, "#a11d38");   // Rich red
        grad.addColorStop(0.6, "#e63462");   // FXSound accent red
        grad.addColorStop(0.85, "#f7546f");  // Lighter red
        grad.addColorStop(1, "#ff7a8a");     // Bright tip
        gradientCacheRef.current = grad;
        return grad;
    }, []);

    /**
     * Match the canvas backing store to its CSS size times the device pixel
     * ratio, then scale the context so drawing code can keep working in CSS
     * pixels. The canvas previously had a fixed 456px buffer stretched by CSS,
     * which rendered soft on HiDPI and fractional-scaled Linux desktops.
     */
    const syncCanvasSize = useCallback((canvas, ctx) => {
        const dpr = window.devicePixelRatio || 1;
        const cssWidth = canvas.clientWidth || canvas.width;
        const pixelWidth = Math.max(1, Math.round(cssWidth * dpr));
        const pixelHeight = Math.max(1, Math.round(CANVAS_HEIGHT * dpr));

        if (canvas.width !== pixelWidth || canvas.height !== pixelHeight) {
            canvas.width = pixelWidth;
            canvas.height = pixelHeight;
            gradientCacheRef.current = null; // gradient is tied to the old size
            needsRepaintRef.current = true;
        }
        ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
        return cssWidth;
    }, []);

    // Draw bars onto the canvas — no React state, no DOM updates
    const drawFrame = useCallback((now, lastTimeRef) => {
        const canvas = canvasRef.current;
        if (!canvas) return;
        const ctx = canvas.getContext("2d");
        if (!ctx) return;

        const w = syncCanvasSize(canvas, ctx);
        const h = CANVAS_HEIGHT;

        const dt = Math.min((now - lastTimeRef.current) / 1000, 0.1);
        lastTimeRef.current = now;

        const isPowered = poweredRef.current;
        const target = targetData.current;
        const display = displayData.current;

        let moving = false;

        // Interpolate display toward target. When bypassed every bar targets
        // zero, so the meter visibly falls instead of freezing on the last
        // frame captured before power-off.
        for (let i = 0; i < BAR_COUNT; i++) {
            const t = isPowered ? target[i] : 0;
            const diff = t - display[i];

            // Snap to target if very close to avoid infinite asymptotic calculations
            if (Math.abs(diff) < 0.05) {
                display[i] = t;
            } else {
                const speed = t > display[i] ? 14 : 5;
                display[i] += diff * Math.min(speed * dt, 1);
                moving = true;
            }
        }

        if (!moving && !needsRepaintRef.current) return;
        needsRepaintRef.current = false;

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
        const barGrad = isPowered ? getBarGradient(ctx, h) : null;

        for (let i = 0; i < BAR_COUNT; i++) {
            const barH = Math.max(2, display[i]);
            const x = i * (barW + gap);
            const y = h - barH;

            ctx.beginPath();
            if (SUPPORTS_ROUND_RECT) {
                ctx.roundRect(x, y, barW, barH, isPowered ? [3, 3, 0, 0] : [1, 1, 0, 0]);
            } else {
                ctx.rect(x, y, barW, barH);
            }

            if (!isPowered) {
                ctx.fillStyle = "#1e1e2a";
                ctx.globalAlpha = 0.3;
                ctx.fill();
                ctx.globalAlpha = 1;
                continue;
            }

            const intensity = Math.min(barH / 70, 1);

            ctx.globalAlpha = 0.5 + intensity * 0.5;
            ctx.fillStyle = barGrad;
            ctx.fill();

            // Top glow highlight on taller bars
            if (barH > 8) {
                ctx.globalAlpha = intensity * 0.4;
                ctx.fillStyle = "#ff8a9a";
                ctx.fillRect(x + 1, y, barW - 2, 2);
            }
        }
        ctx.globalAlpha = 1;
    }, [getBarGradient, syncCanvasSize]);

    useEffect(() => {
        let cancelled = false;
        let pollTimeout = null;
        const lastTimeRef = { current: performance.now() };

        // Poll the engine's FFT output. This is the only real data source: the
        // bars must reflect the audio actually being processed.
        function pollBackend() {
            if (cancelled) return;
            if (!poweredRef.current) {
                pollTimeout = setTimeout(pollBackend, POLL_INTERVAL_MS);
                return;
            }
            call("get_visualizer_data")
                .then((data) => {
                    if (!data || !data.length) return;
                    const src = targetData.current;
                    const ratio = data.length / BAR_COUNT;
                    for (let i = 0; i < BAR_COUNT; i++) {
                        const si = Math.floor(i * ratio);
                        const ni = Math.min(si + 1, data.length - 1);
                        const f = (i * ratio) - si;
                        src[i] = data[si] * (1 - f) + data[ni] * f;
                    }
                })
                .catch(() => { /* transient IPC failure — keep polling */ })
                .finally(() => {
                    if (!cancelled) {
                        pollTimeout = setTimeout(pollBackend, POLL_INTERVAL_MS);
                    }
                });
        }

        // Decorative sweep used only when there is no Tauri backend at all,
        // i.e. `npm run dev` in a plain browser. Never runs in the packaged app,
        // so shipped builds can never show invented spectrum data.
        function idleAnimation() {
            let phase = 0;
            function step() {
                if (cancelled) return;
                phase += 0.08;
                if (poweredRef.current) {
                    const tgt = targetData.current;
                    for (let i = 0; i < BAR_COUNT; i++) {
                        tgt[i] = (Math.sin(phase + i * 0.3) * 0.5 + 0.5 +
                                   Math.sin(phase * 1.3 + i * 0.2) * 0.3 + 0.3) * 35 + 5;
                    }
                }
                pollTimeout = setTimeout(step, POLL_INTERVAL_MS);
            }
            step();
        }

        async function init() {
            if (cancelled) return;
            try {
                // A successful call means the backend is present; its value not
                // being loud yet is irrelevant.
                await call("get_visualizer_data");
                pollBackend();
            } catch {
                idleAnimation();
            }
        }

        init();

        function loop(now) {
            if (cancelled) return;
            drawFrame(now, lastTimeRef);
            animFrameRef.current = requestAnimationFrame(loop);
        }
        animFrameRef.current = requestAnimationFrame(loop);

        const handleResize = () => { needsRepaintRef.current = true; };
        window.addEventListener("resize", handleResize);

        return () => {
            cancelled = true;
            if (pollTimeout) clearTimeout(pollTimeout);
            if (animFrameRef.current) cancelAnimationFrame(animFrameRef.current);
            window.removeEventListener("resize", handleResize);
        };
    }, [drawFrame]);

    return (
        <div className="visualizer" aria-hidden="true">
            <canvas
                ref={canvasRef}
                style={{ display: "block", width: "100%", height: `${CANVAS_HEIGHT}px` }}
            />
        </div>
    );
});

export default Visualizer;
