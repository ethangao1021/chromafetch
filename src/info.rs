use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ModuleResult {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub modules: HashMap<String, Vec<ModuleResult>>,
}

impl SystemInfo {
    pub fn new() -> Self {
        Self { modules: HashMap::new() }
    }

    pub fn insert(&mut self, name: &str, results: Vec<ModuleResult>) {
        self.modules.insert(name.to_string(), results);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogoMode {
    Auto,
    Ascii,
    NoneColor,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DisplayConfig {
    pub flag: Option<String>,
    pub logo: LogoMode,
    pub lightness: Option<String>,
    pub separator: String,
    pub color_key: String,
    pub color_value: String,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            flag: None,
            logo: LogoMode::Auto,
            lightness: None,
            separator: " -> ".to_string(),
            color_key: "cyan".to_string(),
            color_value: "reset".to_string(),
        }
    }
}

pub const DEFAULT_MODULE_ORDER: &[&str] = &[
    "Title", "Separator",
    "OS", "Host", "Kernel", "Architecture", "OSBuild", "Uptime",
    "Processes", "LoadAvg", "Packages", "Shell",
    "Terminal",
    "CPU", "GPU",
    "Memory", "Disk", "PhysicalDisk",
    "Display", "DE", "WM", "WMTheme",
    "Theme", "Icons", "Font", "Cursor",
    "LocalIp", "Network",
    "Motherboard",
    "Sound", "Monitor",
    "Battery",
    "Sensors",
    "Users", "DateTime", "Locale", "Editor",
    "Media", "Virtualization",
    "InitSystem", "Systemd", "PackageManager", "PhysicalMemory",
    "Libraries",
    "Break", "Colors",
];
