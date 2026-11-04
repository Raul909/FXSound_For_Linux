import React, { useState, useEffect, useCallback, useMemo, useRef } from "react";
import { call, subscribe } from "./tauri";

import { PRESETS, PRESET_EQ, PRESET_FX, EQ_BANDS, INITIAL_PRESET, APP_VERSION } from "./constants";
import EQBand from "./components/EQBand";
import EffectSlider from "./components/EffectSlider";
import Visualizer from "./components/Visualizer";
import SettingsPanel from "./components/SettingsPanel";
import { I18nContext, makeT, translateMessage } from "./i18n";

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
  const [uiInfo, setUiInfo] = useState(null);
  const [notice, setNotice] = useState(null);
  // Bumped by every settings change; a slower `get_ui_info` reply that was
  // started before a newer change is dropped instead of undoing it.
  const uiSeq = useRef(0);
  // False while the window is hidden in the tray or minimized.
  const [visible, setVisible] = useState(() => document.visibilityState !== "hidden");
  // The backend resolves "auto" from the system locale and reports the result.
  const language = uiInfo?.language ?? "en";
  const t = useMemo(() => makeT(language), [language]);
  useEffect(() => {
    document.documentElement.lang = language === "zh" ? "zh-CN" : language;
  }, [language]);

  // Window options, the rendering in use, and telling the backend the page
  // loaded (which confirms an automatic GPU choice works on this machine).
  useEffect(() => {
    call("get_ui_info").then(setUiInfo).catch(() => {});
    call("ui_ready").catch(() => {});
  }, []);

  // Power toggled from the tray menu, and window hidden/shown via the tray.
  useEffect(() => {
    const stopPower = subscribe("power-changed", (enabled) => setPowered(enabled));
    const stopWindow = subscribe("window-visibility", (shown) => setVisible(shown));
    const onVisibility = () => setVisible(document.visibilityState !== "hidden");
    document.addEventListener("visibilitychange", onVisibility);
    return () => {
      stopPower();
      stopWindow();
      document.removeEventListener("visibilitychange", onVisibility);
    };
  }, []);

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
  const updateUi = useCallback((ui) => {
    uiSeq.current += 1;
    setUiInfo((prev) => (prev ? { ...prev, ui } : prev));
    call("set_ui_settings", { ui })
      .then((r) => r && setUiInfo((prev) => (prev ? { ...prev, language: r.language, next_renderer: r.next_renderer } : prev)))
      .catch(console.error);
  }, []);

  const changeAutostart = useCallback((enabled) => {
    uiSeq.current += 1;
    call("set_autostart", { enabled })
      .then((now) => setUiInfo((prev) => (prev ? { ...prev, autostart: now } : prev)))
      .catch((err) => setNotice({ kind: "error", text: String(err) }));
  }, []);

  const exportConfig = useCallback(() => {
    call("export_settings")
      .then((saved) => saved && setNotice({ kind: "ok", key: "set.exported" }))
      .catch((err) => setNotice({ kind: "error", text: String(err) }));
  }, []);

  const importConfig = useCallback(() => {
    call("import_settings")
      .then((s) => {
        if (!s) return;
        const seq = ++uiSeq.current;
        const known = s.preset === "Custom" || PRESET_EQ[s.preset];
        setPowered(s.powered);
        setPreset(known ? s.preset : "Custom");
        setEq([...s.eq]);
        setFx({ ...PRESET_FX.Flat, ...s.effects });
        setUiInfo((prev) => (prev ? { ...prev, ui: s.ui } : prev));
        call("get_ui_info")
          .then((info) => uiSeq.current === seq && setUiInfo(info))
          .catch(() => {});
        setNotice({ kind: "ok", key: "set.imported" });
      })
      .catch((err) => setNotice({ kind: "error", text: String(err) }));
  }, []);

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
      id: "preset",
      label: t("header.preset"),
      value: preset,
      options: [
        ...PRESETS.map((name) => ({ value: name, label: t(`preset.${name}`) })),
        // Shown only once the user has moved a slider off a named preset.
        ...(preset === "Custom" ? [{ value: "Custom", label: t("preset.Custom") }] : []),
      ],
      onChange: applyPreset,
      // Like the sliders, presets only apply while processing is on.
      disabled: !powered,
    },
    {
      id: "output",
      label: t("header.output"),
      value: audio.output ?? "",
      options: audio.devices.length
        ? audio.devices.map((d) => ({ value: d.name, label: d.description }))
        : [{ value: "", label: audio.state === "starting" ? t("devices.detecting") : t("devices.none") }],
      onChange: changeDevice,
      // Audio still flows through FXSound when bypassed, so the device can
      // always be changed.
      disabled: !audio.devices.length,
    },
  ], [preset, powered, audio, applyPreset, changeDevice, t]);

  // Effect sliders with display labels and their keys in PRESET_FX
  const effectSliders = useMemo(
    () => ["fidelity", "ambiance", "dynamic", "surround", "bass"].map((key) => ({ label: t(`fx.${key}`), key })),
    [t],
  );

  // What the status bar reports: the device in use, or why there is none.
  const statusInfo = useMemo(() => {
    if (audio.state === "error") return translateMessage(t, audio.message) || t("info.unavailable");
    const name = audio.devices.find((d) => d.name === audio.output)?.description;
    if (!name) return audio.state === "starting" ? t("info.connecting") : t("devices.none");
    return `${name} · 48kHz`;
  }, [audio, t]);

  const statusLabel =
    audio.state === "error" ? t("status.offline")
      : audio.state === "starting" ? t("status.starting")
        : powered ? t("status.active") : t("status.bypassed");
  const statusMod =
    audio.state === "error" ? "error"
      : audio.state === "running" && powered ? "active" : null;

  // ---------- Render ----------

  return (
    <I18nContext.Provider value={t}>
    <div className="app-container">
      <div className="app-window">

        {/* ---- Header: Brand + Power Button + Dropdowns ---- */}
        <div className="header">
          <div className="header__left">
            <button
              className={`power-btn ${powered ? "power-btn--on" : "power-btn--off"}`}
              onClick={togglePower}
              aria-label={t("power.toggle")}
              title={powered ? t("power.turnOff") : t("power.turnOn")}
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
            {dropdowns.map(({ id, label, value: val, options, onChange, disabled }) => (
              <div key={id} className="dropdown">
                <div id={`dropdown-label-${id}`} className="dropdown__label">{label}</div>
                <div className="dropdown__wrapper">
                  <select
                    value={val}
                    onChange={(e) => onChange(e.target.value)}
                    disabled={disabled}
                    className="dropdown__select"
                    title={disabled && !powered ? t("power.needed") : undefined}
                    aria-labelledby={`dropdown-label-${id}`}
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
        </div>

        {/* ---- Visualizer ---- */}
        <Visualizer powered={powered} active={visible && (uiInfo?.ui.visualizer ?? true)} />

        {/* ---- Tab Buttons ---- */}
        <div className="tabs" role="tablist" aria-label={t("tabs.label")}>
          {[
            { id: "eq", label: t("tabs.eq") },
            { id: "fx", label: t("tabs.fx") },
            { id: "settings", label: t("tabs.settings") },
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
                  const order = ["eq", "fx", "settings"];
                  const step = e.key === "ArrowRight" ? 1 : order.length - 1;
                  const newTab = order[(order.indexOf(tab) + step) % order.length];
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
          className={`content ${tab === "settings" ? "content--settings" : ""}`}
          style={tab === "settings" ? undefined : {
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
                <span className="eq-footer__title">{t("eq.title")}</span>
                <span className="eq-footer__label">+12 dB</span>
              </div>
            </div>
          )}

          {/* Settings Tab: usable even with effects off */}
          {tab === "settings" && (
            <div id="panel-settings" role="tabpanel" aria-labelledby="tab-settings">
              <SettingsPanel
                info={uiInfo}
                onUi={updateUi}
                onAutostart={changeAutostart}
                onExport={exportConfig}
                onImport={importConfig}
                notice={notice}
              />
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
            {(PRESET_EQ[preset] || preset === "Custom" ? t(`preset.${preset}`) : preset).toUpperCase()}
            <span className="status-bar__version">v{APP_VERSION}</span>
          </span>
        </div>

      </div>
    </div>
    </I18nContext.Provider>
  );
}
