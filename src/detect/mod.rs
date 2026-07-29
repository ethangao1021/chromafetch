use crate::info::{ModuleResult, SystemInfo};

#[cfg(target_os = "linux")]
mod linux_backend;
#[cfg(target_os = "linux")]
use linux_backend as backend;

#[cfg(target_os = "macos")]
mod macos_backend;
#[cfg(target_os = "macos")]
use macos_backend as backend;

#[cfg(target_os = "windows")]
mod windows_backend;
#[cfg(target_os = "windows")]
use windows_backend as backend;

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
mod unsupported_backend {
    use crate::info::ModuleResult;
    pub fn detect(_name: &str) -> Vec<ModuleResult> {
        vec![ModuleResult { key: _name.to_string(), value: "unavailable on this platform".to_string() }]
    }
}
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
use unsupported_backend as backend;

pub fn run_detection(enabled: &[String], disabled: &[String]) -> SystemInfo {
    let mut info = SystemInfo::new();

    let results: Vec<(String, Vec<ModuleResult>)> = enabled
        .iter()
        .filter(|m| !disabled.contains(m))
        .map(|name| {
            let result = detect_module(name);
            (name.clone(), result)
        })
        .collect();

    for (name, result) in results {
        if !result.is_empty() {
            info.insert(&name, result);
        }
    }

    info
}

fn detect_module(name: &str) -> Vec<ModuleResult> {
    match name {
        "Title" => {
            let user = whoami::username();
            let host = whoami::fallible::hostname().unwrap_or_else(|_| "unknown".into());
            vec![ModuleResult { key: String::new(), value: format!("{user}@{host}") }]
        }
        "Separator" => {
            vec![ModuleResult { key: String::new(), value: "─".repeat(40) }]
        }
        "Break" => {
            vec![ModuleResult { key: String::new(), value: String::new() }]
        }
        "Colors" => {
            vec![ModuleResult { key: String::new(), value: "■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■".to_string() }]
        }
        _ => backend::detect(name),
    }
}
