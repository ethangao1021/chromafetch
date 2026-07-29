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
    "Terminal", "TerminalFont", "TerminalSize", "TerminalColorSupport",
    "CPU", "CPUUsage", "CPUFrequency", "GPU", "GPUUsage",
    "Memory", "Swap", "Disk", "PhysicalDisk", "DiskIO",
    "Display", "DE", "DesktopEnvironment", "WM", "WindowManager", "WMTheme",
    "Theme", "Icons", "Font", "Cursor",
    "LocalIp", "HostIP", "PublicIp", "Wifi", "Bluetooth", "NetworkIO",
    "Motherboard", "Bios", "Chassis",
    "Sound", "Monitor",
    "Battery", "BatteryStatus", "BatteryCycles", "PowerAdapter",
    "Temperature", "Fans",
    "Users", "DateTime", "Timezone", "Locale", "Editor",
    "Media", "Container", "Virtualization",
    "InitSystem", "Systemd", "PackageManager", "PhysicalMemory",
    "OpenGL", "Vulkan", "GTK", "Qt",
    "DiskUsage", "PhysicalDiskIO",
    "Break", "Colors",
];
