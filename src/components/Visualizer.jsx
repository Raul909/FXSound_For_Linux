import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

/**
 * Real-time audio spectrum visualizer.
 *
 * Fetches FFT data from the Rust backend every 100ms and renders
 * 32 vertical bars representing frequency magnitudes.
 *
 * Props:
 *   powered — when false, bars flatten and polling stops
 */
export default function Visualizer({ powered }) {
    // FFT magnitude data for 32 frequency bins (values 0–100)
    const [fftData, setFftData] = useState(new Array(32).fill(3));

    useEffect(() => {
        if (!powered) {
            setFftData(new Array(32).fill(3));
            return;
        }

        // Poll the backend for FFT data at ~10 FPS
        const interval = setInterval(async () => {
            try {
                const data = await invoke("get_visualizer_data");
                setFftData(data);
            } catch (error) {
                console.error("Failed to get visualizer data:", error);
            }
        }, 100);

        return () => clearInterval(interval);
    }, [powered]);

    return (
        <div className="visualizer">
            {fftData.map((magnitude, index) => {
                const barHeight = powered ? Math.max(4, magnitude * 0.6) : 4;

                return (
                    <div
                        key={index}
                        className={`visualizer__bar ${powered ? "visualizer__bar--active" : ""}`}
                        style={{
                            height: `${barHeight}px`,
                            boxShadow: powered && barHeight > 10
                                ? "0 0 8px rgba(230, 52, 98, 0.4)"
                                : "none",
                        }}
                    />
                );
            })}
        </div>
    );
}
