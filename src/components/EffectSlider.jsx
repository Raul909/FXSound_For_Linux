import { useRef, memo } from "react";

/**
 * A horizontal effect slider.
 *
 * Renders a draggable horizontal slider for one audio effect.
 * The slider maps mouse position to an intensity value between 0 and 100.
 *
 * Props:
 *   effectKey— key representing the effect
 *   label    — effect name displayed to the left (e.g. "Fidelity")
 *   value    — current intensity (0–100)
 *   onChange — callback receiving the key and new value
 *   disabled — whether the slider is interactive
 */
function EffectSlider({ effectKey, label, value, onChange, disabled }) {
    const trackRef = useRef(null);
    const valueRef = useRef(value);

    // Convert a mouse X position to an effect value (0–100)
    function xToValue(mouseX) {
        if (!trackRef.current) return 0;
        const rect = trackRef.current.getBoundingClientRect();
        const ratio = (mouseX - rect.left) / rect.width;
        return Math.round(Math.max(0, Math.min(100, ratio * 100)));
    }

    // Handle drag interaction on the slider track
    function handleMouseDown(event) {
        if (disabled) return;

        valueRef.current = value;
        const initialValue = xToValue(event.clientX);
        if (initialValue !== valueRef.current) {
            valueRef.current = initialValue;
            onChange(effectKey, initialValue);
        }

        function handleMouseMove(moveEvent) {
            const newValue = xToValue(moveEvent.clientX);
            if (newValue !== valueRef.current) {
                valueRef.current = newValue;
                onChange(effectKey, newValue);
            }
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

export default memo(EffectSlider);
