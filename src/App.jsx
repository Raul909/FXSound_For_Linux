import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

import { PRESETS, PRESET_EQ, PRESET_FX, EQ_BANDS, DEVICES } from "./constants";
import EQBand from "./components/EQBand";
import EffectSlider from "./components/EffectSlider";
import Visualizer from "./components/Visualizer";

/**
 * Root application component for FXSound.
 *
 * Manages the global state (power, preset, EQ values, effects, output device)
 * and renders the full UI: title bar, controls, visualizer, tabs, and content.
 */
export default function App() {
  const [powered, setPowered] = useState(true);
  const [preset, setPreset] = useState("Music");
  const [eq, setEq] = useState([...PRESET_EQ["Music"]]);
  const [fx, setFx] = useState({ ...PRESET_FX["Music"] });
  const [device, setDevice] = useState(DEVICES[0]);
  const [tab, setTab] = useState("eq");

  // ---------- Preset & Power Logic ----------

  /**
   * Apply a named preset — updates EQ bands, effects, and sends
   * each value to the Rust backend.
   */
  function applyPreset(name) {
    if (!PRESET_EQ[name]) return;

    setPreset(name);
    setEq([...PRESET_EQ[name]]);
    setFx({ ...PRESET_FX[name] });

    // Send all EQ band values to backend
    PRESET_EQ[name].forEach((gain, index) => {
      invoke("set_eq_band", { band: index, gain }).catch(console.error);
    });

    // Send all effect values to backend
    Object.entries(PRESET_FX[name]).forEach(([key, value]) => {
      invoke("set_effect", { effect: key, value }).catch(console.error);
    });
  }

  // Sync power state to the Rust backend whenever it changes
  useEffect(() => {
    invoke("set_power", { enabled: powered }).catch(console.error);
  }, [powered]);

  /**
   * Update a single EQ band — called when the user drags an EQ slider.
   * Marks the preset as "Custom" since it no longer matches any named preset.
   */
  function updateEQBand(index, value) {
    const newEq = [...eq];
    newEq[index] = value;
    setEq(newEq);
    setPreset("Custom");
    invoke("set_eq_band", { band: index, gain: value }).catch(console.error);
  }

  /**
   * Update a single effect — called when the user drags an effect slider.
   * Marks the preset as "Custom".
   */
  function updateEffect(key, value) {
    setFx((prev) => ({ ...prev, [key]: value }));
    setPreset("Custom");
    invoke("set_effect", { effect: key, value }).catch(console.error);
  }

  // ---------- Dropdown Data ----------

  // Configuration for the two dropdown selectors (preset + output device)
  const dropdowns = [
    {
      label: "PRESET",
      value: preset,
      options: PRESETS,
      onChange: applyPreset,
      showCustom: true,
    },
    {
      label: "OUTPUT DEVICE",
      value: device,
      options: DEVICES,
      onChange: setDevice,
      showCustom: false,
    },
  ];

  // Effect sliders with display labels and their keys in PRESET_FX
  const effectSliders = [
    { label: "Fidelity", key: "fidelity" },
    { label: "Ambiance", key: "ambiance" },
    { label: "Dynamic Boost", key: "dynamic" },
    { label: "3D Surround", key: "surround" },
    { label: "HyperBass", key: "bass" },
  ];

  // ---------- Render ----------

  return (
    <div className="app-container">
      <div className="app-window">

        {/* ---- Title Bar ---- */}
        <div className="title-bar">
          <div className="title-bar__dots">
            <div className="title-bar__dot title-bar__dot--red" />
            <div className="title-bar__dot title-bar__dot--yellow" />
            <div className="title-bar__dot title-bar__dot--green" />
          </div>
          <span className="title-bar__title">FXSOUND</span>
          <div className="title-bar__controls">
            <button className="title-bar__btn">⚙</button>
            <button className="title-bar__btn">−</button>
          </div>
        </div>

        {/* ---- Header: Power Button + Dropdowns ---- */}
        <div className="header">
          <button
            className={`power-btn ${powered ? "power-btn--on" : "power-btn--off"}`}
            onClick={() => setPowered((prev) => !prev)}
          >
            <svg width="24" height="24" viewBox="0 0 24 24" fill={powered ? "#fff" : "#666"}>
              <path d="M13 3h-2v10h2V3zm4.83 2.17l-1.42 1.42C17.99 7.86 19 9.81 19 12c0 3.87-3.13 7-7 7s-7-3.13-7-7c0-2.19 1.01-4.14 2.58-5.42L6.17 5.17C4.23 6.82 3 9.26 3 12c0 4.97 4.03 9 9 9s9-4.03 9-9c0-2.74-1.23-5.18-3.17-6.83z" />
            </svg>
          </button>

          <div className="header__dropdowns">
            {dropdowns.map(({ label, value: val, options, onChange, showCustom }) => (
              <div key={label} className="dropdown">
                <div className="dropdown__label">{label}</div>
                <div className="dropdown__wrapper">
                  <select
                    value={val}
                    onChange={(e) => onChange(e.target.value)}
                    disabled={!powered}
                    className="dropdown__select"
                    style={{ color: powered ? "#b1b1b1" : "#666" }}
                  >
                    {options.map((opt) => (
                      <option key={opt}>{opt}</option>
                    ))}
                    {showCustom && <option value="Custom">Custom</option>}
                  </select>
                  <svg className="dropdown__arrow" width="10" height="6" viewBox="0 0 10 6">
                    <path d="M0 0l5 6 5-6z" fill={powered ? "#e63462" : "#666"} />
                  </svg>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* ---- Visualizer ---- */}
        <Visualizer powered={powered} />

        {/* ---- Tab Buttons ---- */}
        <div className="tabs">
          {[
            { id: "eq", label: "EQUALIZER" },
            { id: "fx", label: "EFFECTS" },
          ].map(({ id, label }) => (
            <button
              key={id}
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
            <>
              <div className="eq-panel">
                {EQ_BANDS.map((freq, index) => (
                  <EQBand
                    key={freq}
                    freq={freq}
                    value={eq[index]}
                    onChange={(val) => updateEQBand(index, val)}
                    disabled={!powered}
                  />
                ))}
              </div>
              <div className="eq-footer">
                <span className="eq-footer__label">-12 dB</span>
                <span className="eq-footer__title">10-Band Parametric EQ</span>
                <span className="eq-footer__label">+12 dB</span>
              </div>
            </>
          )}

          {/* Effects Tab */}
          {tab === "fx" && (
            <div className="fx-panel">
              {effectSliders.map(({ label, key }) => (
                <EffectSlider
                  key={key}
                  label={label}
                  value={fx[key]}
                  onChange={(val) => updateEffect(key, val)}
                  disabled={!powered}
                />
              ))}
            </div>
          )}
        </div>

        {/* ---- Status Bar ---- */}
        <div className="status-bar">
          <div className="status-bar__indicator">
            <div className={`status-bar__dot ${powered ? "status-bar__dot--active" : ""}`} />
            <span className={`status-bar__text ${powered ? "status-bar__text--active" : ""}`}>
              {powered ? "ACTIVE" : "BYPASSED"}
            </span>
          </div>
          <span className="status-bar__info">PulseAudio · 48kHz</span>
          <span className="status-bar__preset">{preset.toUpperCase()}</span>
        </div>

      </div>
    </div>
  );
}
