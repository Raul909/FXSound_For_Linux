// Tauri Command Reference for FXSound Linux
// Import in React components: import { invoke } from "@tauri-apps/api/core";

// Set EQ band gain (-12 to +12 dB)
await invoke('set_eq_band', { 
  band: 0,      // 0-9 (32Hz to 16kHz)
  gain: 5.0     // -12.0 to +12.0
});

// Set effect value (0-100)
await invoke('set_effect', { 
  effect: 'fidelity',  // 'fidelity', 'ambiance', 'dynamic', 'surround', 'bass'
  value: 65.0          // 0.0 to 100.0
});

// Toggle power on/off
await invoke('set_power', { 
  enabled: true  // true or false
});

// Get available audio devices
const devices = await invoke('get_audio_devices');
// Returns: ["Built-in Speakers", "Headphones (3.5mm)", ...]

// Error handling
try {
  await invoke('set_eq_band', { band: 0, gain: 5.0 });
} catch (error) {
  console.error('Failed to set EQ:', error);
}
