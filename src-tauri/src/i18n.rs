//! Languages for the parts the backend draws itself: the tray menu and the
//! native file dialogs. The web page has its own tables (`src/i18n.js`); both
//! use the language resolved here, so they always agree.

/// Languages the interface is translated into.
pub const SUPPORTED: [&str; 4] = ["en", "pt", "es", "zh"];

/// Strings the backend shows.
pub struct Texts {
    pub show: &'static str,
    pub hide: &'static str,
    pub effects_on: &'static str,
    pub quit: &'static str,
    pub export_title: &'static str,
    pub import_title: &'static str,
    pub file_filter: &'static str,
}

const EN: Texts = Texts {
    show: "Show FXSound",
    hide: "Hide to tray",
    effects_on: "Effects on",
    quit: "Quit FXSound",
    export_title: "Export FXSound configuration",
    import_title: "Import FXSound configuration",
    file_filter: "FXSound configuration",
};

const PT: Texts = Texts {
    show: "Mostrar FXSound",
    hide: "Esconder na bandeja",
    effects_on: "Efeitos ligados",
    quit: "Sair do FXSound",
    export_title: "Exportar configuração do FXSound",
    import_title: "Importar configuração do FXSound",
    file_filter: "Configuração do FXSound",
};

const ES: Texts = Texts {
    show: "Mostrar FXSound",
    hide: "Ocultar en la bandeja",
    effects_on: "Efectos encendidos",
    quit: "Salir de FXSound",
    export_title: "Exportar configuración de FXSound",
    import_title: "Importar configuración de FXSound",
    file_filter: "Configuración de FXSound",
};

const ZH: Texts = Texts {
    show: "显示 FXSound",
    hide: "隐藏到托盘",
    effects_on: "开启音效",
    quit: "退出 FXSound",
    export_title: "导出 FXSound 配置",
    import_title: "导入 FXSound 配置",
    file_filter: "FXSound 配置",
};

pub fn texts(lang: &str) -> &'static Texts {
    match lang {
        "pt" => &PT,
        "es" => &ES,
        "zh" => &ZH,
        _ => &EN,
    }
}

/// The language to use for a saved preference ("auto" or a language code).
pub fn resolve(preference: &str) -> &'static str {
    if let Some(code) = SUPPORTED.iter().find(|c| **c == preference) {
        return code;
    }
    // POSIX precedence: LC_ALL, then LC_MESSAGES, then LANG.
    let locale = ["LC_ALL", "LC_MESSAGES", "LANG"]
        .iter()
        .filter_map(|v| std::env::var(v).ok())
        .find(|v| !v.is_empty());
    from_locale(locale.as_deref().unwrap_or(""))
}

/// `pt_BR.UTF-8` → `pt`, `zh_CN` → `zh`, anything unsupported → `en`.
fn from_locale(locale: &str) -> &'static str {
    let lang = locale
        .split(['_', '.', '@', '-'])
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();
    SUPPORTED
        .iter()
        .find(|c| **c == lang)
        .copied()
        .unwrap_or("en")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locales_map_to_supported_languages() {
        assert_eq!(from_locale("pt_BR.UTF-8"), "pt");
        assert_eq!(from_locale("es_MX.UTF-8"), "es");
        assert_eq!(from_locale("zh_CN.UTF-8"), "zh");
        assert_eq!(from_locale("zh_TW"), "zh");
        assert_eq!(from_locale("en_US.UTF-8"), "en");
        assert_eq!(from_locale("de_DE.UTF-8"), "en");
        assert_eq!(from_locale("C"), "en");
        assert_eq!(from_locale(""), "en");
    }

    #[test]
    fn an_explicit_language_wins_over_the_system() {
        for code in SUPPORTED {
            assert_eq!(resolve(code), code);
        }
    }

    #[test]
    fn every_language_has_every_tray_text() {
        for code in SUPPORTED {
            let t = texts(code);
            for s in [
                t.show,
                t.hide,
                t.effects_on,
                t.quit,
                t.export_title,
                t.import_title,
                t.file_filter,
            ] {
                assert!(!s.is_empty(), "{code} is missing a text");
            }
        }
    }
}
