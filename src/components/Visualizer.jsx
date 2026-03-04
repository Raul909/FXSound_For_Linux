import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";

/**
 * Real-time audio spectrum visualizer.
 *
 * Fetches FFT data from the Rust backend every 50ms and renders
 * 32 vertical bars representing frequency magnitudes with smooth
 * interpolation between frames for a fluid feel.
 *
 * Props:
 *   powered — when false, bars flatten and polling stops
 */
export default function Visualizer({ powered }) {
    // Displayed FFT magnitude data (interpolated for smooth rendering)
    const [displayData, setDisplayData] = useState(new Array(32).fill(3));
    // Reference to the latest raw FFT data from the backend
    const targetData = useRef(new Array(32).fill(3));
    // Animation frame ID for cleanup
    const animFrameRef = useRef(null);

    useEffect(() => {
        if (!powered) {
            setDisplayData(new Array(32).fill(3));
            targetData.current = new Array(32).fill(3);
            return;
        }

        // Poll the backend for FFT data at ~20 FPS
        const interval = setInterval(async () => {
            try {
                const data = await invoke("get_visualizer_data");
                targetData.current = data;
            } catch (error) {
                console.error("Failed to get visualizer data:", error);
            }
        }, 50);

        // Smoothly interpolate displayed data toward target data at 60 FPS
        let lastTime = performance.now();
        function animate(now) {
            const dt = Math.min((now - lastTime) / 1000, 0.1); // delta in seconds, capped
            lastTime = now;

            setDisplayData(prev => {
                const next = new Array(prev.length);
                for (let i = 0; i < prev.length; i++) {
                    const target = targetData.current[i] || 0;
                    // Smoothing: rise fast (lerp factor 12), fall slower (factor 6)
                    const speed = target > prev[i] ? 12 : 6;
                    next[i] = prev[i] + (target - prev[i]) * Math.min(speed * dt, 1);
                }
                return next;
            });

            animFrameRef.current = requestAnimationFrame(animate);
        }
        animFrameRef.current = requestAnimationFrame(animate);

        return () => {
            clearInterval(interval);
            if (animFrameRef.current) {
                cancelAnimationFrame(animFrameRef.current);
            }
        };
    }, [powered]);

    return (
        <div className="visualizer">
            {displayData.map((magnitude, index) => {
                const barHeight = powered ? Math.max(3, magnitude * 0.75) : 3;
                const intensity = Math.min(barHeight / 50, 1);

                return (
                    <div
                        key={index}
                        className={`visualizer__bar ${powered ? "visualizer__bar--active" : ""}`}
                        style={{
                            height: `${barHeight}px`,
                            boxShadow: powered && barHeight > 8
                                ? `0 0 ${4 + intensity * 8}px rgba(230, 52, 98, ${0.2 + intensity * 0.4})`
                                : "none",
                            opacity: powered ? 0.6 + intensity * 0.4 : 0.3,
                        }}
                    />
                );
            })}
        </div>
    );
}
