import { useRef } from "react";

/**
 * A horizontal effect slider.
 *
 * Renders a draggable horizontal slider for one audio effect.
 * The slider maps mouse position to an intensity value between 0 and 100.
 *
 * Props:
 *   label    — effect name displayed to the left (e.g. "Fidelity")
 *   value    — current intensity (0–100)
 *   onChange — callback receiving the new value
 *   disabled — whether the slider is interactive
 */
export default function EffectSlider({ label, value, onChange, disabled }) {
    const trackRef = useRef(null);

    // Convert a mouse X position to an effect value (0–100)
    function xToValue(mouseX) {
        const rect = trackRef.current.getBoundingClientRect();
        const ratio = (mouseX - rect.left) / rect.width;
        return Math.round(Math.max(0, Math.min(100, ratio * 100)));
    }

    // Handle drag interaction on the slider track
    function handleMouseDown(event) {
        if (disabled) return;

        onChange(xToValue(event.clientX));

        function handleMouseMove(moveEvent) {
            onChange(xToValue(moveEvent.clientX));
        }

        function handleMouseUp() {
            window.removeEventListener("mousemove", handleMouseMove);
            window.removeEventListener("mouseup", handleMouseUp);
        }

        window.addEventListener("mousemove", handleMouseMove);
        window.addEventListener("mouseup", handleMouseUp);
    }

    return (
        <div className="effect-slider">
            {/* Effect name */}
            <span className="effect-slider__label">{label}</span>

            {/* Horizontal slider track */}
            <div
                ref={trackRef}
                onMouseDown={handleMouseDown}
                className="effect-slider__track"
                style={{ cursor: disabled ? "default" : "pointer" }}
                role="slider"
                aria-label={`Effect ${label}`}
                aria-valuemin="0"
                aria-valuemax="100"
                aria-valuenow={value}
                aria-disabled={disabled}
                tabIndex={disabled ? -1 : 0}
                onKeyDown={(e) => {
                    if (disabled) return;
                    if (e.key === "ArrowUp" || e.key === "ArrowRight") {
                        e.preventDefault();
                        onChange(Math.min(100, value + 5));
                    } else if (e.key === "ArrowDown" || e.key === "ArrowLeft") {
                        e.preventDefault();
                        onChange(Math.max(0, value - 5));
                    }
                }}
            >
                {/* Filled portion of the track */}
                <div
                    className="effect-slider__fill"
                    style={{
                        width: `${value}%`,
                        opacity: disabled ? 0.3 : 0.2,
                    }}
                />

                {/* Draggable thumb */}
                <div
                    className="effect-slider__thumb"
                    style={{
                        left: `${value}%`,
                        opacity: disabled ? 0.3 : 1,
                    }}
                />
            </div>

            {/* Current numeric value */}
            <span className="effect-slider__value">{value}</span>
        </div>
    );
}
