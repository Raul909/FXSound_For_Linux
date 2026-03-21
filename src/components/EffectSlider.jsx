import { useRef, memo } from "react";

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
const EffectSlider = memo(function EffectSlider({ label, value, onChange, disabled }) {
    const trackRef = useRef(null);

    // Convert a mouse X position to an effect value (0–100)
    function xToValue(mouseX) {
        const rect = trackRef.current.getBoundingClientRect();
        const ratio = (mouseX - rect.left) / rect.width;
        return Math.round(Math.max(0, Math.min(100, ratio * 100)));
    }

    // Handle keyboard navigation (W3C compliant)
    function handleKeyDown(event) {
        if (disabled) return;

        let newValue = value;
        const step = event.shiftKey ? 10 : 1;

        switch (event.key) {
            case "ArrowRight":
            case "ArrowUp":
                newValue = Math.min(100, value + step);
                break;
            case "ArrowLeft":
            case "ArrowDown":
                newValue = Math.max(0, value - step);
                break;
            case "PageUp":
                newValue = Math.min(100, value + 10);
                break;
            case "PageDown":
                newValue = Math.max(0, value - 10);
                break;
            case "Home":
                newValue = 0;
                break;
            case "End":
                newValue = 100;
                break;
            default:
                return; // Let other keys behave normally
        }

        event.preventDefault(); // Prevent scrolling
        if (newValue !== value) {
            onChange(newValue);
        }
    }

    // Handle drag interaction on the slider track
    function handleMouseDown(event) {
        if (disabled) return;

        let lastValue = xToValue(event.clientX);
        onChange(lastValue);

        function handleMouseMove(moveEvent) {
            const newValue = xToValue(moveEvent.clientX);
            if (newValue !== lastValue) {
                lastValue = newValue;
                onChange(newValue);
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
                role="slider"
                tabIndex={disabled ? -1 : 0}
                aria-valuenow={value}
                aria-valuemin={0}
                aria-valuemax={100}
                aria-label={label}
                ref={trackRef}
                onMouseDown={handleMouseDown}
                onKeyDown={handleKeyDown}
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
});

export default EffectSlider;
