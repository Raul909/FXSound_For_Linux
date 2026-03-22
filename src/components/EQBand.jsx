import { useRef, memo } from "react";

/**
 * A single vertical EQ band slider.
 *
 * Renders a draggable vertical slider for one frequency band of the equalizer.
 * The slider maps mouse position to a gain value between -12 dB and +12 dB.
 *
 * Props:
 *   freq     — frequency label to display (e.g. "32", "1K")
 *   value    — current gain in dB (-12 to +12)
 *   onChange — callback receiving the new gain value
 *   disabled — whether the slider is interactive
 */
const EQBand = memo(function EQBand({ freq, value, onChange, disabled }) {
    const trackRef = useRef(null);

    // Convert a mouse Y position to a gain value (-12 to +12 dB)
    function yToGain(mouseY) {
        const rect = trackRef.current.getBoundingClientRect();
        const ratio = 1 - (mouseY - rect.top) / rect.height; // 0 at bottom, 1 at top
        return Math.round(Math.max(-12, Math.min(12, ratio * 24 - 12)));
    }

    // Handle keyboard interaction for the slider
    function handleKeyDown(event) {
        if (disabled) return;

        let newValue = value;
        const step = event.shiftKey ? 2 : 1;

        switch (event.key) {
            case "ArrowLeft":
            case "ArrowDown":
                newValue = Math.max(-12, value - step);
                break;
            case "ArrowRight":
            case "ArrowUp":
                newValue = Math.min(12, value + step);
                break;
            case "Home":
                newValue = -12;
                break;
            case "End":
                newValue = 12;
                break;
            default:
                return;
        }

        event.preventDefault();
        if (newValue !== value) {
            onChange(newValue);
        }
    }

    // Handle drag interaction on the slider track
    function handleMouseDown(event) {
        if (disabled) return;

        let lastValue = yToGain(event.clientY);
        onChange(lastValue);

        function handleMouseMove(moveEvent) {
            const newValue = yToGain(moveEvent.clientY);
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

    // Calculate thumb position as a percentage (0% = -12dB bottom, 100% = +12dB top)
    const thumbPercent = ((value + 12) / 24) * 100;

    return (
        <div className="eq-band">
            {/* Current gain value label */}
            <span className="eq-band__value">
                {value > 0 ? `+${value}` : value}
            </span>

            {/* Vertical slider track */}
            <div
                ref={trackRef}
                onMouseDown={handleMouseDown}
                onKeyDown={handleKeyDown}
                className="eq-band__track"
                style={{ cursor: disabled ? "default" : "pointer" }}
                role="slider"
                tabIndex={disabled ? -1 : 0}
                aria-label={`${freq} EQ Band`}
                aria-valuenow={value}
                aria-valuemin={-12}
                aria-valuemax={12}
                aria-orientation="vertical"
            >
                {/* Center line marking 0 dB */}
                <div className="eq-band__center-line" />

                {/* Dashed vertical guide line */}
                <svg className="eq-band__guide" width="2" height="140">
                    <line
                        x1="1" y1="0" x2="1" y2="140"
                        stroke="#e33250" strokeWidth="1"
                        strokeDasharray="5,2" opacity="0.4"
                    />
                </svg>

                {/* Draggable thumb */}
                <div
                    className="eq-band__thumb"
                    style={{
                        top: `${100 - thumbPercent}%`,
                        opacity: disabled ? 0.3 : 1,
                    }}
                />
            </div>

            {/* Frequency label */}
            <span className="eq-band__freq">{freq}</span>
        </div>
    );
});

export default EQBand;
