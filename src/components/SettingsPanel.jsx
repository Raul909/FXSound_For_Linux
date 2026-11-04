import React from "react";
import { LANGUAGES, translateMessage, useT } from "../i18n";

const RENDER_MODES = ["auto", "gpu", "software"];

function Toggle({ id, label, hint, tip, checked, disabled, onChange }) {
    return (
        <div className={`settings-row ${disabled ? "settings-row--disabled" : ""}`} title={tip}>
            <div className="settings-row__text">
                <label htmlFor={id} className="settings-row__label">{label}</label>
                {hint && <span className="settings-row__hint">{hint}</span>}
            </div>
            <button
                id={id}
                type="button"
                role="switch"
                aria-checked={checked}
                disabled={disabled}
                className={`switch ${checked ? "switch--on" : ""}`}
                onClick={() => onChange(!checked)}
            >
                <span className="switch__knob" />
            </button>
        </div>
    );
}

function Select({ id, value, options, onChange }) {
    return (
        <div className="dropdown__wrapper settings-row__select">
            <select id={id} className="dropdown__select" value={value} onChange={(e) => onChange(e.target.value)}>
                {options.map(({ value: v, label }) => (
                    <option key={v} value={v}>{label}</option>
                ))}
            </select>
            <svg aria-hidden="true" className="dropdown__arrow" width="10" height="6" viewBox="0 0 10 6">
                <path d="M0 0l5 6 5-6z" fill="#e63462" />
            </svg>
        </div>
    );
}

/**
 * Window, startup, language and rendering options, plus configuration
 * export/import.
 *
 * Props:
 *   info     — `get_ui_info` result: { ui, graphics, tray_available, autostart, language }
 *   onUi     — save a changed `ui` object
 *   onAutostart(enabled)
 *   onExport / onImport — open the file dialogs
 *   notice   — { kind: "ok" | "error", key?, text? } after an export/import, or null
 */
export default function SettingsPanel({ info, onUi, onAutostart, onExport, onImport, notice }) {
    const t = useT();
    if (!info) {
        return <div className="settings-panel settings-panel--loading">{t("set.loading")}</div>;
    }
    const { ui, graphics, tray_available: tray, autostart, language } = info;
    const set = (key) => (value) => onUi({ ...ui, [key]: value });
    const restartNeeded =
        (ui.render_mode === "gpu" && graphics.renderer !== "gpu") ||
        (ui.render_mode === "software" && graphics.renderer !== "software") ||
        (ui.render_mode === "auto" && graphics.reason === "user-choice");
    const current = LANGUAGES.find((l) => l.code === language)?.name ?? "English";
    const noticeText = notice ? (notice.key ? t(notice.key) : translateMessage(t, notice.text)) : "";

    return (
        <div className="settings-panel">
            <Toggle
                id="set-visualizer"
                label={t("set.visualizer")}
                tip={t("set.visualizerHint")}
                checked={ui.visualizer}
                onChange={set("visualizer")}
            />
            <Toggle
                id="set-tray"
                label={t("set.tray")}
                tip={t("set.trayHint")}
                hint={tray ? undefined : t("set.trayMissing")}
                checked={tray && ui.close_to_tray}
                disabled={!tray}
                onChange={set("close_to_tray")}
            />
            <Toggle id="set-autostart" label={t("set.autostart")} checked={autostart} onChange={onAutostart} />
            <Toggle
                id="set-hidden"
                label={t("set.hidden")}
                checked={tray && ui.start_hidden}
                disabled={!tray || !autostart}
                onChange={set("start_hidden")}
            />

            <div className="settings-row">
                <div className="settings-row__text">
                    <label htmlFor="set-language" className="settings-row__label">{t("set.language")}</label>
                </div>
                <Select
                    id="set-language"
                    value={ui.language}
                    onChange={set("language")}
                    options={[
                        { value: "auto", label: t("set.languageAuto", { name: current }) },
                        ...LANGUAGES.map(({ code, name }) => ({ value: code, label: name })),
                    ]}
                />
            </div>

            <div className="settings-row">
                <div className="settings-row__text">
                    <label htmlFor="set-render" className="settings-row__label">{t("set.rendering")}</label>
                    <span className="settings-row__hint">
                        {t("set.renderingNow", {
                            renderer: t(`renderer.${graphics.renderer}`),
                            reason: t(`reason.${graphics.reason}`),
                        })}
                        {restartNeeded && t("set.restart")}
                    </span>
                </div>
                <Select
                    id="set-render"
                    value={ui.render_mode}
                    onChange={set("render_mode")}
                    options={RENDER_MODES.map((m) => ({ value: m, label: t(`mode.${m}`) }))}
                />
            </div>

            <div className="settings-actions" title={t("set.exportHint")}>
                <button type="button" className="settings-btn" onClick={onExport}>{t("set.export")}</button>
                <button type="button" className="settings-btn" onClick={onImport}>{t("set.import")}</button>
            </div>
            <p className={`settings-notice ${notice ? `settings-notice--${notice.kind}` : ""}`} role="status">
                {noticeText}
            </p>
        </div>
    );
}
