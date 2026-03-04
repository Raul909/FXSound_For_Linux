/**
 * Audio preset and configuration constants for FXSound.
 *
 * PRESETS       — ordered list of preset names shown in the dropdown
 * PRESET_EQ    — 10-band EQ gain values (dB) for each preset
 * PRESET_FX    — effect intensity (0–100) for each preset
 * EQ_BANDS     — frequency labels for the 10 EQ sliders
 * DEVICES      — available output devices
 */

// Preset names displayed in the dropdown selector
export const PRESETS = [
    "Flat",
    "Music",
    "Movies",
    "Gaming",
    "Podcast",
    "Bass Boost",
    "Vocal Boost",
    "Deep Bass",
    "Treble Boost",
    "Night Mode",
];

// EQ gain values for each preset (10 bands, values in dB from -12 to +12)
// Band order: 32Hz, 64Hz, 125Hz, 250Hz, 500Hz, 1kHz, 2kHz, 4kHz, 8kHz, 16kHz
export const PRESET_EQ = {
    "Flat": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    "Music": [3, 2, 1, 0, -1, 0, 2, 3, 3, 2],
    "Movies": [4, 3, 2, 0, -1, 0, 1, 2, 3, 4],
    "Gaming": [5, 4, 2, 1, 0, 0, 1, 2, 4, 5],
    "Podcast": [-1, 0, 2, 4, 4, 3, 2, 1, 0, -1],
    "Bass Boost": [8, 7, 5, 3, 0, -1, -1, -1, -2, -2],
    "Vocal Boost": [-2, -1, 0, 3, 5, 5, 3, 1, 0, -1],
    "Deep Bass": [10, 8, 6, 2, 0, -1, -2, -2, -3, -3],
    "Treble Boost": [-3, -2, -1, 0, 1, 2, 4, 6, 7, 8],
    "Night Mode": [2, 2, 1, 0, -2, -2, -1, 0, 1, 2],
};

// Effect values for each preset (0–100 intensity scale)
export const PRESET_FX = {
    "Flat": { fidelity: 0, ambiance: 0, dynamic: 0, surround: 0, bass: 0 },
    "Music": { fidelity: 65, ambiance: 40, dynamic: 50, surround: 30, bass: 45 },
    "Movies": { fidelity: 70, ambiance: 75, dynamic: 60, surround: 80, bass: 55 },
    "Gaming": { fidelity: 60, ambiance: 50, dynamic: 70, surround: 90, bass: 60 },
    "Podcast": { fidelity: 80, ambiance: 20, dynamic: 55, surround: 10, bass: 20 },
    "Bass Boost": { fidelity: 50, ambiance: 30, dynamic: 65, surround: 20, bass: 90 },
    "Vocal Boost": { fidelity: 85, ambiance: 35, dynamic: 50, surround: 25, bass: 15 },
    "Deep Bass": { fidelity: 45, ambiance: 25, dynamic: 70, surround: 15, bass: 95 },
    "Treble Boost": { fidelity: 75, ambiance: 45, dynamic: 45, surround: 35, bass: 10 },
    "Night Mode": { fidelity: 55, ambiance: 60, dynamic: 35, surround: 40, bass: 30 },
};

// Frequency labels for the 10-band EQ (displayed below each slider)
export const EQ_BANDS = ["32", "64", "125", "250", "500", "1K", "2K", "4K", "8K", "16K"];

// Available audio output devices
export const DEVICES = [
    "Built-in Speakers",
    "Headphones (3.5mm)",
    "USB Audio Device",
    "HDMI Audio",
    "Bluetooth Headset",
];
