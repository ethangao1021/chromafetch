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
        "Editor" => {
            let editor = std::env::var("EDITOR")
                .or_else(|_| std::env::var("VISUAL"))
                .unwrap_or_else(|_| String::new());
            if editor.is_empty() {
                Vec::new()
            } else {
                let name = std::path::Path::new(&editor)
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or(&editor);
                vec![ModuleResult { key: "Editor".to_string(), value: name.to_string() }]
            }
        }
        "DateTime" => {
            let mut results = Vec::new();
            let dt = datetime_now();
            if !dt.is_empty() {
                results.push(ModuleResult { key: "Date".to_string(), value: dt });
            }
            let tz = timezone_name();
            if !tz.is_empty() {
                results.push(ModuleResult { key: "Timezone".to_string(), value: tz });
            }
            results
        }
        _ => backend::detect(name),
    }
}

fn datetime_now() -> String {
    unsafe {
        let mut tv: libc::time_t = 0;
        libc::time(&mut tv);
        #[cfg(unix)]
        let tm = {
            let tm_ptr = libc::localtime(&tv);
            if tm_ptr.is_null() {
                return String::new();
            }
            *tm_ptr
        };
        #[cfg(windows)]
        let tm = {
            let mut tm: libc::tm = std::mem::zeroed();
            if libc::localtime_s(&mut tm, &tv) != 0 {
                return String::new();
            }
            tm
        };
        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            tm.tm_year + 1900,
            tm.tm_mon + 1,
            tm.tm_mday,
            tm.tm_hour,
            tm.tm_min,
            tm.tm_sec,
        )
    }
}

fn timezone_name() -> String {
    if let Ok(tz) = std::env::var("TZ") {
        if !tz.is_empty() {
            return tz;
        }
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(link) = std::fs::read_link("/etc/localtime") {
            if let Some(path) = link.to_str() {
                if let Some(name) = path.rsplit("/zoneinfo/").next() {
                    return name.to_string();
                }
                if let Some(name) = path.rsplit('/').next() {
                    return name.to_string();
                }
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Ok(link) = std::fs::read_link("/etc/localtime") {
            if let Some(path) = link.to_str() {
                if let Some(name) = path.rsplit("/zoneinfo/").next() {
                    return name.to_string();
                }
            }
        }
    }
    #[cfg(target_os = "windows")]
    {
        let tz = crate::detect::windows_backend::windows_timezone();
        if !tz.is_empty() {
            return tz;
        }
    }
    String::new()
}
