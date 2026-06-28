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
    // Uses a cached rect to avoid layout thrashing during drag
    function xToValueWithRect(mouseX, rect) {
        const ratio = (mouseX - rect.left) / rect.width;
        return Math.round(Math.max(0, Math.min(100, ratio * 100)));
    }

    // Handle keyboard interaction for accessibility
    function handleKeyDown(event) {
        if (disabled) return;

        let newValue = value;
        switch (event.key) {
            case "ArrowRight":
            case "ArrowUp":
                newValue = Math.min(100, value + 5);
                event.preventDefault();
                break;
            case "ArrowLeft":
            case "ArrowDown":
                newValue = Math.max(0, value - 5);
                event.preventDefault();
                break;
            case "Home":
                newValue = 0;
                event.preventDefault();
                break;
            case "End":
                newValue = 100;
                event.preventDefault();
                break;
            case "PageUp":
                newValue = Math.min(100, value + 10);
                event.preventDefault();
                break;
            case "PageDown":
                newValue = Math.max(0, value - 10);
                event.preventDefault();
                break;
            default:
                return;
        }

        if (newValue !== value) {
            onChange(newValue);
        }
    }

    function handleDoubleClick() {
        if (disabled) return;
        if (value !== 0) {
            onChange(0);
        }
    }

    // Handle drag interaction on the slider track
    // Caches getBoundingClientRect on mousedown to prevent layout thrashing
    function handleMouseDown(event) {
        if (disabled) return;

        const rect = trackRef.current.getBoundingClientRect();
        let lastValue = xToValueWithRect(event.clientX, rect);
        onChange(lastValue);

        function handleMouseMove(moveEvent) {
            const newValue = xToValueWithRect(moveEvent.clientX, rect);
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
            <span className="effect-slider__label" id={`label-${label.replace(/\s+/g, '-')}`}>{label}</span>

            {/* Horizontal slider track */}
            <div
                ref={trackRef}
                onMouseDown={handleMouseDown}
                onKeyDown={handleKeyDown}
                onDoubleClick={handleDoubleClick}
                className="effect-slider__track"
                style={{ cursor: disabled ? "default" : "pointer" }}
                title={disabled ? "Power on to adjust" : "Double-click to reset"}
                role="slider"
                tabIndex={disabled ? -1 : 0}
                aria-labelledby={`label-${label.replace(/\s+/g, '-')}`}
                aria-valuemin={0}
                aria-valuemax={100}
                aria-valuenow={value}
                aria-disabled={disabled}
            >
                {/* Filled portion of the track */}
                <div
                    className="effect-slider__fill"
                    style={{
                        width: `${value}%`,
                        opacity: disabled ? 0.3 : 1,
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
            <span className="effect-slider__value" aria-hidden="true">{value}</span>
        </div>
    );
});

export default EffectSlider;
