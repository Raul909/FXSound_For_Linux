import React, { useState, useEffect, useCallback, useMemo } from "react";
import { call, subscribe } from "./tauri";

import { PRESETS, PRESET_EQ, PRESET_FX, EQ_BANDS, INITIAL_PRESET, APP_VERSION } from "./constants";
import EQBand from "./components/EQBand";
import EffectSlider from "./components/EffectSlider";
import Visualizer from "./components/Visualizer";

// Helper wrappers to give stable references via useCallback at the child level
// Wrapped in React.memo to prevent unnecessary re-renders when other App state changes
const EQBandWrapper = React.memo(function EQBandWrapper({ freq, index, value, updateEQBand, disabled }) {
  const handleChange = useCallback((val) => updateEQBand(index, val), [index, updateEQBand]);
  return <EQBand freq={freq} value={value} onChange={handleChange} disabled={disabled} />;
});

const EffectSliderWrapper = React.memo(function EffectSliderWrapper({ label, effectKey, value, updateEffect, disabled }) {
  const handleChange = useCallback((val) => updateEffect(effectKey, val), [effectKey, updateEffect]);
  return <EffectSlider label={label} value={value} onChange={handleChange} disabled={disabled} />;
});

/**
 * Root application component for FXSound.
 *
 * Manages the global state (power, preset, EQ values, effects, output device)
 * and renders the full UI: title bar, controls, visualizer, tabs, and content.
 */
export default function App() {
  const [powered, setPowered] = useState(true);
  const [preset, setPreset] = useState(INITIAL_PRESET);
  const [eq, setEq] = useState([...PRESET_EQ[INITIAL_PRESET]]);
  const [fx, setFx] = useState({ ...PRESET_FX[INITIAL_PRESET] });
  // Live routing state from the backend: { state, message, devices, output }.
  // Devices are real sinks ({ name, description }) — never invented ones.
  const [audio, setAudio] = useState({ state: "starting", message: null, devices: [], output: null });
  const [tab, setTab] = useState("eq");

  // Restore the last session. The backend has already applied these values to
  // the engine before any audio flowed; this only brings the controls in line.
  // On first launch there is nothing saved, so apply the default preset.
  useEffect(() => {
    let cancelled = false;
    call("get_settings")
      .then((saved) => {
        if (cancelled) return;
        if (saved) {
          const known = saved.preset === "Custom" || PRESET_EQ[saved.preset];
          setPowered(saved.powered);
          setPreset(known ? saved.preset : "Custom");
          setEq([...saved.eq]);
          setFx({ ...PRESET_FX.Flat, ...saved.effects });
        } else {
          call("apply_preset_state", {
            eqBands: PRESET_EQ[INITIAL_PRESET],
            effects: PRESET_FX[INITIAL_PRESET],
            preset: INITIAL_PRESET,
          }).catch(console.error);
        }
      })
      .catch((err) => console.warn("Could not load settings:", err));
    return () => {
      cancelled = true;
    };
  }, []);

  // Follow the audio routing: output devices appear and disappear as they are
  // plugged in, and the backend reports problems instead of hanging.
  useEffect(() => {
    let cancelled = false;
    call("get_audio_status")
      .then((status) => {
        if (!cancelled && status) setAudio(status);
      })
      .catch(() => {});
    const stop = subscribe("audio-status", (status) => setAudio(status));
    return () => {
      cancelled = true;
      stop();
    };
  }, []);

  // ---------- Preset & Power Logic ----------

  /**
   * Apply a named preset — updates EQ bands, effects, and sends
   * each value to the Rust backend.
   */
  const applyPreset = useCallback((name) => {
    if (!PRESET_EQ[name]) return;

    setPreset(name);
    setEq([...PRESET_EQ[name]]);
    setFx({ ...PRESET_FX[name] });

    // Send all EQ band values and effect values to backend in one batch
    call("apply_preset_state", {
      eqBands: PRESET_EQ[name],
      effects: PRESET_FX[name],
      preset: name,
    }).catch(console.error);
  }, []);

  /**
   * Toggle processing. Off is a bypass: audio keeps playing, unprocessed.
   * Sent from the click itself rather than from an effect on `powered`, which
   * fired on mount and would overwrite a saved "off" with the initial "on".
   */
  const togglePower = useCallback(() => {
    const next = !powered;
    setPowered(next);
    call("set_power", { enabled: next }).catch(console.error);
  }, [powered]);

  /**
   * Update a single EQ band — called when the user drags an EQ slider.
   * Marks the preset as "Custom" since it no longer matches any named preset.
   */
  const updateEQBand = useCallback((index, value) => {
    setEq((prev) => {
      const newEq = [...prev];
      newEq[index] = value;
      return newEq;
    });
    setPreset("Custom");
    call("set_eq_band", { band: index, gain: value }).catch(console.error);
  }, []);

  /**
   * Update a single effect — called when the user drags an effect slider.
   * Marks the preset as "Custom".
   */
  const updateEffect = useCallback((key, value) => {
    setFx((prev) => ({ ...prev, [key]: value }));
    setPreset("Custom");
    call("set_effect", { effect: key, value }).catch(console.error);
  }, []);

  /**
   * Play to a different output device. The backend moves FXSound's output
   * there and remembers the choice for next time.
   */
  const changeDevice = useCallback((sinkName) => {
    setAudio((prev) => ({ ...prev, output: sinkName }));
    call("set_output_device", { sink: sinkName || null }).catch(console.error);
  }, []);

  // ---------- Dropdown Data ----------

  // Configuration for the two dropdown selectors (preset + output device).
  // Options are {value, label} pairs: sinks are selected by their PulseAudio
  // name but displayed by their human-readable description.
  const dropdowns = useMemo(() => [
    {
      label: "PRESET",
      value: preset,
      options: [
        ...PRESETS.map((name) => ({ value: name, label: name })),
        // Shown only once the user has moved a slider off a named preset.
        ...(preset === "Custom" ? [{ value: "Custom", label: "Custom" }] : []),
      ],
      onChange: applyPreset,
      // Like the sliders, presets only apply while processing is on.
      disabled: !powered,
    },
    {
      label: "OUTPUT DEVICE",
      value: audio.output ?? "",
      options: audio.devices.length
        ? audio.devices.map((d) => ({ value: d.name, label: d.description }))
        : [{ value: "", label: audio.state === "starting" ? "Detecting…" : "No output device" }],
      onChange: changeDevice,
      // Audio still flows through FXSound when bypassed, so the device can
      // always be changed.
      disabled: !audio.devices.length,
    },
  ], [preset, powered, audio, applyPreset, changeDevice]);

  // Effect sliders with display labels and their keys in PRESET_FX
  const effectSliders = useMemo(() => [
    { label: "Fidelity", key: "fidelity" },
    { label: "Ambiance", key: "ambiance" },
    { label: "Dynamic Boost", key: "dynamic" },
    { label: "3D Surround", key: "surround" },
    { label: "HyperBass", key: "bass" },
  ], []);

  // What the status bar reports: the device in use, or why there is none.
  const statusInfo = useMemo(() => {
    if (audio.state === "error") return audio.message || "Audio unavailable";
    const name = audio.devices.find((d) => d.name === audio.output)?.description;
    if (!name) return audio.state === "starting" ? "Connecting…" : "No output device";
    return `${name} · 48kHz`;
  }, [audio]);

  const statusLabel =
    audio.state === "error" ? "OFFLINE"
      : audio.state === "starting" ? "STARTING"
        : powered ? "ACTIVE" : "BYPASSED";
  const statusMod =
    audio.state === "error" ? "error"
      : audio.state === "running" && powered ? "active" : null;

  // ---------- Window Controls ----------

  const handleMinimize = useCallback(() => {
    call("minimize_window").catch(() => {
      import("@tauri-apps/api/window")
        .then(({ getCurrentWindow }) => getCurrentWindow().minimize())
        .catch((err) => console.warn("Could not minimize window:", err));
    });
  }, []);

  const handleDragStart = useCallback((e) => {
    if (e.buttons === 1 && !e.target.closest("button")) {
      call("start_dragging").catch(() => {
        import("@tauri-apps/api/window")
          .then(({ getCurrentWindow }) => getCurrentWindow().startDragging())
          .catch((err) => console.warn("Could not start dragging:", err));
      });
    }
  }, []);

  const handleClose = useCallback(() => {
    call("close_window").catch(() => {
      import("@tauri-apps/api/window")
        .then(({ getCurrentWindow }) => getCurrentWindow().close())
        .catch((err) => console.warn("Could not close window:", err));
    });
  }, []);

  // ---------- Preset Import / Export ----------

  const fileInputRef = React.useRef(null);

  const handleExportPreset = useCallback(() => {
    const name = preset === "Custom" ? "My Custom Preset" : preset;
    call("export_preset", { name })
      .then((savedPath) => {
        if (savedPath) {
          console.log("Preset exported successfully to:", savedPath);
        }
      })
      .catch((err) => console.error("Export failed:", err));
  }, [preset]);

  const handleImportPreset = useCallback(() => {
    fileInputRef.current?.click();
  }, []);

  const onFileSelected = useCallback((e) => {
    const file = e.target.files?.[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = (ev) => {
      const json = ev.target?.result;
      if (typeof json !== "string") return;
      call("import_preset", { json })
        .then((result) => {
          setEq([...result.eq]);
          setFx({ ...result.effects });
          setPreset("Custom");
        })
        .catch((err) => console.error("Import failed:", err));
    };
    reader.readAsText(file);
    // Reset so the same file can be re-imported
    e.target.value = "";
  }, []);

  // ---------- Render ----------

  return (
    <div className="app-container">
      <div className="app-window">

        {/* ---- Window Title Bar (Drag region + Minimize / Close controls) ---- */}
        <div className="titlebar" data-tauri-drag-region onMouseDown={handleDragStart}>
          <div className="titlebar__drag" data-tauri-drag-region onMouseDown={handleDragStart}>
            <span className="titlebar__brand" data-tauri-drag-region>
              <span className="titlebar__brand-fx">FX</span>SOUND
            </span>
          </div>
          <div className="titlebar__controls">
            <button
              className="titlebar__btn titlebar__btn--minimize"
              onClick={handleMinimize}
              aria-label="Minimize Window"
              title="Minimize"
            >
              <svg aria-hidden="true" width="10" height="10" viewBox="0 0 10 10">
                <rect x="1" y="4.5" width="8" height="1.2" rx="0.6" fill="currentColor" />
              </svg>
            </button>
            <button
              className="titlebar__btn titlebar__btn--close"
              onClick={handleClose}
              aria-label="Close Window"
              title="Close"
            >
              <svg aria-hidden="true" width="10" height="10" viewBox="0 0 10 10">
                <path
                  d="M1.5 1.5L8.5 8.5M8.5 1.5L1.5 8.5"
                  stroke="currentColor"
                  strokeWidth="1.2"
                  strokeLinecap="round"
                />
              </svg>
            </button>
          </div>
        </div>

        {/* ---- Header: Brand + Power Button + Dropdowns ---- */}
        <div className="header">
          <div className="header__left">
            <button
              className={`power-btn ${powered ? "power-btn--on" : "power-btn--off"}`}
              onClick={togglePower}
              aria-label="Toggle Power"
              title={powered ? "Turn Power Off" : "Turn Power On"}
              aria-pressed={powered}
            >
              <svg aria-hidden="true" width="22" height="22" viewBox="0 0 24 24" fill={powered ? "#fff" : "#555"}>
                <path d="M13 3h-2v10h2V3zm4.83 2.17l-1.42 1.42C17.99 7.86 19 9.81 19 12c0 3.87-3.13 7-7 7s-7-3.13-7-7c0-2.19 1.01-4.14 2.58-5.42L6.17 5.17C4.23 6.82 3 9.26 3 12c0 4.97 4.03 9 9 9s9-4.03 9-9c0-2.74-1.23-5.18-3.17-6.83z" />
              </svg>
            </button>
            <span className="header__brand">
              <span className="header__brand-fx">FX</span>SOUND
            </span>
          </div>

          <div className="header__dropdowns">
            {dropdowns.map(({ label, value: val, options, onChange, disabled }) => (
              <div key={label} className="dropdown">
                <div className="dropdown__header">
                  <div id={`dropdown-label-${label.replace(/\s+/g, "-")}`} className="dropdown__label">{label}</div>
                  {label === "PRESET" && (
                    <div className="preset-actions">
                      <button
                        className="preset-actions__btn"
                        onClick={handleImportPreset}
                        title="Import Preset (Load from File)"
                        aria-label="Import Preset from File"
                      >
                        <svg aria-hidden="true" width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
                          <path d="M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z"/>
                        </svg>
                      </button>
                      <button
                        className="preset-actions__btn"
                        onClick={handleExportPreset}
                        title="Export Preset (Save to File)"
                        aria-label="Export Current Preset to File"
                      >
                        <svg aria-hidden="true" width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
                          <path d="M9 16h6v-6h4l-7-7-7 7h4zm-4 2h14v2H5z"/>
                        </svg>
                      </button>
                    </div>
                  )}
                </div>
                <div className="dropdown__wrapper">
                  <select
                    value={val}
                    onChange={(e) => onChange(e.target.value)}
                    disabled={disabled}
                    className="dropdown__select"
                    title={disabled && !powered ? "Power on to adjust" : undefined}
                    aria-labelledby={`dropdown-label-${label.replace(/\s+/g, "-")}`}
                  >
                    {options.map(({ value: optValue, label: optLabel }) => (
                      <option key={optValue} value={optValue}>{optLabel}</option>
                    ))}
                  </select>
                  <svg aria-hidden="true" className="dropdown__arrow" width="10" height="6" viewBox="0 0 10 6">
                    <path d="M0 0l5 6 5-6z" fill={disabled ? "#555" : "#e63462"} />
                  </svg>
                </div>
              </div>
            ))}
          </div>

          {/* Hidden file input for preset import */}
          <input
            ref={fileInputRef}
            type="file"
            accept=".fxpreset,.json"
            style={{ display: "none" }}
            onChange={onFileSelected}
          />
        </div>

        {/* ---- Visualizer ---- */}
        <Visualizer powered={powered} />

        {/* ---- Tab Buttons ---- */}
        <div className="tabs" role="tablist" aria-label="Settings pages">
          {[
            { id: "eq", label: "EQUALIZER" },
            { id: "fx", label: "EFFECTS" },
          ].map(({ id, label }) => (
            <button
              key={id}
              id={`tab-${id}`}
              role="tab"
              aria-selected={tab === id}
              aria-controls={`panel-${id}`}
              tabIndex={tab === id ? 0 : -1}
              onKeyDown={(e) => {
                if (e.key === "ArrowRight" || e.key === "ArrowLeft") {
                  e.preventDefault();
                  const newTab = tab === "eq" ? "fx" : "eq";
                  setTab(newTab);
                  document.getElementById(`tab-${newTab}`)?.focus();
                }
              }}
              className={`tabs__btn ${tab === id ? "tabs__btn--active" : ""}`}
              onClick={() => setTab(id)}
            >
              {label}
            </button>
          ))}
        </div>

        {/* ---- Tab Content ---- */}
        <div
          className="content"
          style={{
            opacity: powered ? 1 : 0.4,
            pointerEvents: powered ? "auto" : "none",
          }}
        >
          {/* Equalizer Tab */}
          {tab === "eq" && (
            <div id="panel-eq" role="tabpanel" aria-labelledby="tab-eq">
              <div className="eq-panel">
                {EQ_BANDS.map((freq, index) => (
                  <EQBandWrapper
                    key={freq}
                    freq={freq}
                    index={index}
                    value={eq[index]}
                    updateEQBand={updateEQBand}
                    disabled={!powered}
                  />
                ))}
              </div>
              <div className="eq-footer">
                <span className="eq-footer__label">-12 dB</span>
                <span className="eq-footer__title">10-Band Parametric EQ</span>
                <span className="eq-footer__label">+12 dB</span>
              </div>
            </div>
          )}

          {/* Effects Tab */}
          {tab === "fx" && (
            <div id="panel-fx" role="tabpanel" aria-labelledby="tab-fx" className="fx-panel">
              {effectSliders.map(({ label, key }) => (
                <EffectSliderWrapper
                  key={key}
                  label={label}
                  effectKey={key}
                  value={fx[key]}
                  updateEffect={updateEffect}
                  disabled={!powered}
                />
              ))}
            </div>
          )}
        </div>

        {/* ---- Status Bar ---- */}
        <div className="status-bar" role="status">
          <div className="status-bar__indicator">
            <div className={`status-bar__dot ${statusMod ? `status-bar__dot--${statusMod}` : ""}`} />
            <span className={`status-bar__text ${statusMod ? `status-bar__text--${statusMod}` : ""}`}>
              {statusLabel}
            </span>
          </div>
          <span className="status-bar__info" title={statusInfo}>{statusInfo}</span>
          <span className="status-bar__preset">
            {preset.toUpperCase()}
            <span className="status-bar__version">v{APP_VERSION}</span>
          </span>
        </div>

      </div>
    </div>
  );
}
