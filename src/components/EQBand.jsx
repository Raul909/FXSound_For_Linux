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
    // Uses a cached rect to avoid layout thrashing during drag
    function yToGainWithRect(mouseY, rect) {
        const ratio = 1 - (mouseY - rect.top) / rect.height; // 0 at bottom, 1 at top
        return Math.round(Math.max(-12, Math.min(12, ratio * 24 - 12)));
    }

    // Handle keyboard interaction for accessibility
    function handleKeyDown(event) {
        if (disabled) return;

        let newValue = value;
        switch (event.key) {
            case "ArrowUp":
            case "ArrowRight":
                newValue = Math.min(12, value + 1);
                event.preventDefault();
                break;
            case "ArrowDown":
            case "ArrowLeft":
                newValue = Math.max(-12, value - 1);
                event.preventDefault();
                break;
            case "Home":
                newValue = -12;
                event.preventDefault();
                break;
            case "End":
                newValue = 12;
                event.preventDefault();
                break;
            case "PageUp":
                newValue = Math.min(12, value + 3);
                event.preventDefault();
                break;
            case "PageDown":
                newValue = Math.max(-12, value - 3);
                event.preventDefault();
                break;
            default:
                return;
        }

        if (newValue !== value) {
            onChange(newValue);
        }
    }

    // Handle drag interaction on the slider track
    // Caches getBoundingClientRect on mousedown to prevent layout thrashing
    function handleMouseDown(event) {
        if (disabled) return;

        const rect = trackRef.current.getBoundingClientRect();
        let lastValue = yToGainWithRect(event.clientY, rect);
        onChange(lastValue);

        let rafId = null;
        let latestClientY = event.clientY;

        function handleMouseMove(moveEvent) {
            latestClientY = moveEvent.clientY;
            if (!rafId) {
                rafId = requestAnimationFrame(() => {
                    rafId = null;
                    const newValue = yToGainWithRect(latestClientY, rect);
                    if (newValue !== lastValue) {
                        lastValue = newValue;
                        onChange(newValue);
                    }
                });
            }
        }

        function handleMouseUp() {
            if (rafId) {
                cancelAnimationFrame(rafId);
                rafId = null;
                const newValue = yToGainWithRect(latestClientY, rect);
                if (newValue !== lastValue) {
                    lastValue = newValue;
                    onChange(newValue);
                }
            }
            window.removeEventListener("mousemove", handleMouseMove);
            window.removeEventListener("mouseup", handleMouseUp);
        }

        window.addEventListener("mousemove", handleMouseMove);
        window.addEventListener("mouseup", handleMouseUp);
    }

    // Calculate thumb position as a percentage (0% = -12dB bottom, 100% = +12dB top)
    const thumbPercent = ((value + 12) / 24) * 100;

    // Calculate the filled region from center (50%) to the thumb position
    const fillTop = value >= 0 ? (100 - thumbPercent) : 50;
    const fillHeight = Math.abs(thumbPercent - 50);

    return (
        <div className="eq-band">
            {/* Current gain value label */}
            <span className="eq-band__value" aria-hidden="true">
                {value > 0 ? `+${value}` : value}
            </span>

            {/* Vertical slider track */}
            <div
                ref={trackRef}
                onMouseDown={handleMouseDown}
                onDoubleClick={() => !disabled && onChange(0)}
                onKeyDown={handleKeyDown}
                className="eq-band__track"
                style={{ cursor: disabled ? "default" : "pointer" }}
                title={disabled ? "Power on to adjust" : "Double-click to reset to 0 dB"}
                role="slider"
                tabIndex={disabled ? -1 : 0}
                aria-label={`${freq} Hz equalizer band`}
                aria-valuemin={-12}
                aria-valuemax={12}
                aria-valuenow={value}
                aria-valuetext={`${value > 0 ? '+' : ''}${value} decibels`}
                aria-disabled={disabled}
                aria-orientation="vertical"
            >
                {/* Center line marking 0 dB */}
                <div className="eq-band__center-line" />

                {/* Filled region from center to thumb (shows boost/cut) */}
                <div
                    className="eq-band__fill"
                    style={{
                        top: `${fillTop}%`,
                        height: `${fillHeight}%`,
                        opacity: disabled ? 0.15 : 1,
                    }}
                />

                {/* Dashed vertical guide line */}
                <svg className="eq-band__guide" width="2" height="160">
                    <line
                        x1="1" y1="0" x2="1" y2="160"
                        stroke="#e33250" strokeWidth="1"
                        strokeDasharray="5,2" opacity="0.25"
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
            <span className="eq-band__freq" aria-hidden="true">{freq}</span>
        </div>
    );
});

export default EQBand;
