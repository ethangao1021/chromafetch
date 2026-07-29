use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub display: DisplaySection,

    #[serde(default)]
    pub modules: ModulesSection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplaySection {
    #[serde(default)]
    pub flag: Option<String>,

    #[serde(default = "default_logo")]
    pub logo: String,

    #[serde(default)]
    pub lightness: Option<String>,

    #[serde(default = "default_separator")]
    pub separator: String,

    #[serde(default = "default_color_key")]
    pub color_key: String,

    #[serde(default = "default_color_value")]
    pub color_value: String,
}

fn default_logo() -> String {
    "auto".to_string()
}
fn default_separator() -> String {
    " -> ".to_string()
}
fn default_color_key() -> String {
    "cyan".to_string()
}
fn default_color_value() -> String {
    "reset".to_string()
}

impl Default for DisplaySection {
    fn default() -> Self {
        Self {
            flag: None,
            logo: default_logo(),
            lightness: None,
            separator: default_separator(),
            color_key: default_color_key(),
            color_value: default_color_value(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModulesSection {
    #[serde(default = "default_order")]
    pub order: Vec<String>,

    #[serde(default)]
    pub disabled: Vec<String>,
}

fn default_order() -> Vec<String> {
    crate::info::DEFAULT_MODULE_ORDER.iter().map(|s| s.to_string()).collect()
}

impl Default for ModulesSection {
    fn default() -> Self {
        Self {
            order: default_order(),
            disabled: Vec::new(),
        }
    }
}

impl Config {
    pub fn load(path: Option<&str>) -> Self {
        let config_path = path.map(PathBuf::from).unwrap_or_else(default_config_path);

        if let Ok(content) = std::fs::read_to_string(&config_path) {
            toml::from_str(&content).unwrap_or_else(|e| {
                eprintln!("warn: failed to parse config: {e}, using defaults");
                Config::default()
            })
        } else {
            Config::default()
        }
    }

    pub fn save(&self, path: Option<&str>) -> Result<(), String> {
        let config_path = path.map(PathBuf::from).unwrap_or_else(default_config_path);

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("failed to create config dir: {e}"))?;
        }

        let content = toml::to_string_pretty(self).map_err(|e| format!("failed to serialize config: {e}"))?;
        std::fs::write(&config_path, content).map_err(|e| format!("failed to write config: {e}"))?;
        Ok(())
    }

    pub fn apply_flag(&mut self, flag: &str) {
        self.display.flag = Some(flag.to_string());
    }

    pub fn apply_theme(&mut self, url_or_path: &str) -> Result<(), String> {
        let content = if url_or_path.starts_with("http://") || url_or_path.starts_with("https://") {
            eprintln!("note: downloading theme from {url_or_path}");
            let resp = ureq::get(url_or_path)
                .call()
                .map_err(|e| format!("failed to download theme: {e}"))?;
            resp.into_body()
                .read_to_string()
                .map_err(|e| format!("failed to read theme response: {e}"))?
        } else {
            std::fs::read_to_string(url_or_path)
                .map_err(|e| format!("failed to read theme file: {e}"))?
        };

        let theme: Config = toml::from_str(&content).map_err(|e| format!("invalid theme config: {e}"))?;

        if let Some(flag) = theme.display.flag {
            self.display.flag = Some(flag);
        }
        if theme.display.logo != default_logo() {
            self.display.logo = theme.display.logo;
        }
        if let Some(lightness) = theme.display.lightness {
            self.display.lightness = Some(lightness);
        }
        if theme.display.separator != default_separator() {
            self.display.separator = theme.display.separator;
        }
        if !theme.modules.disabled.is_empty() {
            self.modules.disabled = theme.modules.disabled;
        }

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display: DisplaySection::default(),
            modules: ModulesSection::default(),
        }
    }
}

fn default_config_path() -> PathBuf {
    let base = if cfg!(target_os = "windows") {
        std::env::var("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."))
    } else {
        std::env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                PathBuf::from(home).join(".config")
            })
    };
    base.join("chromafetch").join("config.toml")
}
