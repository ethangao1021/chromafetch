use crate::info::ModuleResult;
use std::ffi::CStr;
use std::path::Path;

pub fn detect(name: &str) -> Vec<ModuleResult> {
    match name {
        "OS" => os(),
        "Host" => host(),
        "Kernel" => kernel(),
        "Architecture" => architecture(),
        "OSBuild" => os_build(),
        "Uptime" => uptime(),
        "Processes" => processes(),
        "LoadAvg" => load_avg(),
        "Packages" => packages(),
        "Shell" => shell(),
        "Display" => display(),
        "DE" | "DesktopEnvironment" => de(),
        "WM" | "WindowManager" => wm(),
        "WMTheme" => wm_theme(),
        "Theme" => theme(),
        "Icons" => icons(),
        "Font" => font(),
        "Cursor" => cursor(),
        "Terminal" => terminal(),
        "TerminalFont" => terminal_font(),
        "TerminalSize" => terminal_size(),
        "CPU" => cpu(),
        "CPUUsage" => Vec::new(),
        "CPUFrequency" => cpu_frequency(),
        "GPU" => gpu(),
        "GPUUsage" => Vec::new(),
        "Memory" => memory(),
        "Swap" => swap(),
        "Disk" => disk(),
        "PhysicalDisk" => physical_disk(),
        "DiskIO" => disk_io(),
        "PhysicalDiskIO" => physical_disk(),
        "LocalIp" | "HostIP" => local_ip(),
        "PublicIp" => public_ip(),
        "Battery" => battery(),
        "BatteryStatus" => battery_status(),
        "BatteryCycles" => battery_cycles(),
        "PowerAdapter" => power_adapter(),
        "Locale" => locale(),
        "Users" => users(),
        "Motherboard" => motherboard(),
        "Bios" => bios(),
        "Chassis" => chassis(),
        "Sound" => sound(),
        "Bluetooth" => bluetooth(),
        "Wifi" => wifi(),
        "NetworkIO" => network_io(),
        "Media" => media(),
        "Monitor" => monitor(),
        "Container" => container(),
        "Virtualization" => virtualization(),
        "Temperature" => temperature(),
        "Fans" => fans(),
        "PhysicalMemory" => physical_memory(),
        "Systemd" => systemd_units(),
        "InitSystem" => init_system(),
        "PackageManager" => package_manager(),
        "OpenGL" => opengl_version(),
        "Vulkan" => vulkan_version(),
        "GTK" => gtk_version(),
        "Qt" => qt_version(),
        "DiskUsage" => disk(),
        _ => vec![ModuleResult { key: name.to_string(), value: "unknown module".to_string() }],
    }
}

fn read_first_line(path: impl AsRef<Path>) -> Option<String> {
    let content = std::fs::read_to_string(path.as_ref()).ok()?;
    content.lines().next().map(|s| s.trim().to_string())
}

fn join_non_empty(items: &[Option<String>], sep: &str) -> String {
    items.iter().filter_map(|x| x.as_ref()).cloned().collect::<Vec<_>>().join(sep)
}

// ── OS ──

fn os() -> Vec<ModuleResult> {
    let content = std::fs::read_to_string("/etc/os-release").ok()
        .or_else(|| std::fs::read_to_string("/usr/lib/os-release").ok());

    let (name, version) = match content {
        Some(c) => {
            let mut name = None;
            let mut version = None;
            let mut pretty = None;
            for line in c.lines() {
                if let Some(val) = line.strip_prefix("PRETTY_NAME=") {
                    pretty = Some(val.trim_matches('"').to_string());
                } else if let Some(val) = line.strip_prefix("NAME=") {
                    name = Some(val.trim_matches('"').to_string());
                } else if let Some(val) = line.strip_prefix("VERSION_ID=") {
                    version = Some(val.trim_matches('"').to_string());
                }
            }
            if let Some(p) = pretty {
                (p, String::new())
            } else {
                (name.unwrap_or_else(|| "Linux".to_string()), version.unwrap_or_default())
            }
        }
        None => ("Linux".to_string(), String::new()),
    };

    let value = if version.is_empty() { name } else { format!("{name} {version}") };
    vec![ModuleResult { key: "OS".to_string(), value }]
}

fn os_build() -> Vec<ModuleResult> {
    let content = std::fs::read_to_string("/etc/os-release").ok()
        .or_else(|| std::fs::read_to_string("/usr/lib/os-release").ok());

    if let Some(c) = content {
        for line in c.lines() {
            if let Some(val) = line.strip_prefix("BUILD_ID=") {
                let build = val.trim_matches('"').to_string();
                if !build.is_empty() {
                    return vec![ModuleResult { key: "Build".to_string(), value: build }];
                }
            }
        }
    }
    Vec::new()
}

fn architecture() -> Vec<ModuleResult> {
    unsafe {
        let mut utsname = std::mem::MaybeUninit::<libc::utsname>::uninit();
        if libc::uname(utsname.as_mut_ptr()) == 0 {
            let uts = utsname.assume_init();
            let machine = CStr::from_ptr(uts.machine.as_ptr()).to_string_lossy().into_owned();
            let mapped = match machine.as_str() {
                "x86_64" => "x86_64",
                "aarch64" => "ARM64",
                "armv7l" => "ARM32",
                "i686" => "x86_32",
                "riscv64" => "RISC-V 64",
                _ => &machine,
            };
            vec![ModuleResult { key: "Architecture".to_string(), value: mapped.to_string() }]
        } else {
            Vec::new()
        }
    }
}

// ── Host ──

fn host() -> Vec<ModuleResult> {
    let product = read_first_line("/sys/devices/virtual/dmi/id/product_name");
    let version = read_first_line("/sys/devices/virtual/dmi/id/product_version");
    let value = join_non_empty(&[product, version], " ");
    if value.is_empty() {
        return Vec::new();
    }
    vec![ModuleResult { key: "Host".to_string(), value }]
}

fn motherboard() -> Vec<ModuleResult> {
    let vendor = read_first_line("/sys/devices/virtual/dmi/id/board_vendor");
    let name = read_first_line("/sys/devices/virtual/dmi/id/board_name");
    let version = read_first_line("/sys/devices/virtual/dmi/id/board_version");
    let value = join_non_empty(&[vendor, name, version], " ");
    if value.is_empty() {
        return Vec::new();
    }
    vec![ModuleResult { key: "Motherboard".to_string(), value }]
}

fn bios() -> Vec<ModuleResult> {
    let vendor = read_first_line("/sys/devices/virtual/dmi/id/bios_vendor");
    let version = read_first_line("/sys/devices/virtual/dmi/id/bios_version");
    let date = read_first_line("/sys/devices/virtual/dmi/id/bios_date");
    let value = join_non_empty(&[vendor, version, date], " ");
    if value.is_empty() {
        return Vec::new();
    }
    vec![ModuleResult { key: "BIOS".to_string(), value }]
}

fn chassis() -> Vec<ModuleResult> {
    let chassis_type = read_first_line("/sys/devices/virtual/dmi/id/chassis_type");
    if let Some(ct) = chassis_type {
        let mapped = match ct.as_str() {
            "1" | "2" | "3" | "4" | "5" => "Desktop",
            "6" | "7" | "8" | "9" | "10" => "Laptop",
            "11" | "12" => "Server",
            "13" => "Tablet",
            "14" => "Portable",
            "21" => "Convertible",
            _ => &ct,
        };
        vec![ModuleResult { key: "Chassis".to_string(), value: mapped.to_string() }]
    } else {
        Vec::new()
    }
}

// ── Kernel ──

fn kernel() -> Vec<ModuleResult> {
    let release = unsafe {
        let mut utsname = std::mem::MaybeUninit::<libc::utsname>::uninit();
        if libc::uname(utsname.as_mut_ptr()) == 0 {
            let uts = utsname.assume_init();
            Some(CStr::from_ptr(uts.release.as_ptr()).to_string_lossy().into_owned())
        } else {
            read_first_line("/proc/sys/kernel/osrelease")
        }
    };
    if let Some(r) = release {
        vec![ModuleResult { key: "Kernel".to_string(), value: r }]
    } else {
        Vec::new()
    }
}

// ── Uptime ──

fn uptime() -> Vec<ModuleResult> {
    let content = match read_first_line("/proc/uptime") {
        Some(c) => c,
        None => return Vec::new(),
    };
    let seconds: f64 = match content.split_whitespace().next().and_then(|s| s.parse().ok()) {
        Some(s) => s,
        None => return Vec::new(),
    };
    let days = (seconds / 86400.0) as u64;
    let hours = ((seconds % 86400.0) / 3600.0) as u64;
    let mins = ((seconds % 3600.0) / 60.0) as u64;

    let value = if days > 0 {
        format!("{days}d {hours}h {mins}m")
    } else if hours > 0 {
        format!("{hours}h {mins}m")
    } else {
        format!("{mins}m")
    };
    vec![ModuleResult { key: "Uptime".to_string(), value }]
}

// ── Processes ──

fn processes() -> Vec<ModuleResult> {
    if let Ok(entries) = std::fs::read_dir("/proc") {
        let count = entries
            .flatten()
            .filter(|e| {
                e.file_name().to_string_lossy().as_ref().parse::<u32>().is_ok()
            })
            .count();
        if count > 0 {
            return vec![ModuleResult { key: "Processes".to_string(), value: count.to_string() }];
        }
    }
    Vec::new()
}

// ── LoadAvg ──

fn load_avg() -> Vec<ModuleResult> {
    let content = match read_first_line("/proc/loadavg") {
        Some(c) => c,
        None => return Vec::new(),
    };
    let parts: Vec<&str> = content.split_whitespace().collect();
    if parts.len() >= 3 {
        vec![ModuleResult { key: "Load Average".to_string(), value: format!("{} {} {}", parts[0], parts[1], parts[2]) }]
    } else {
        Vec::new()
    }
}

// ── Packages ──

fn packages() -> Vec<ModuleResult> {
    let mut counts: Vec<String> = Vec::new();

    if let Ok(content) = std::fs::read_to_string("/var/lib/dpkg/status") {
        let count = content.lines().filter(|l| l.starts_with("Package:")).count();
        if count > 0 {
            counts.push(format!("dpkg:{count}"));
        }
    }
    if let Ok(entries) = std::fs::read_dir("/var/lib/pacman/local") {
        let count = entries.count();
        if count > 0 {
            counts.push(format!("pacman:{count}"));
        }
    }
    if let Ok(entries) = std::fs::read_dir("/var/lib/flatpak/app") {
        let count = entries.count();
        if count > 0 {
            counts.push(format!("flatpak:{count}"));
        }
    }
    if let Ok(entries) = std::fs::read_dir("/var/lib/snapd/snaps") {
        let count = entries.count();
        if count > 0 {
            counts.push(format!("snap:{count}"));
        }
    }
    if let Ok(entries) = std::fs::read_dir("/var/db/pkg") {
        let count = entries.count();
        if count > 0 {
            counts.push(format!("ebuild:{count}"));
        }
    }
    if let Ok(entries) = std::fs::read_dir("/var/db/xbps/pkgdb") {
        let count = entries.count();
        if count > 0 {
            counts.push(format!("xbps:{count}"));
        }
    }
    if let Ok(content) = std::fs::read_to_string("/var/log/rpm") {
        let count = content.lines().filter(|l| l.contains(" install ")).count();
        if count > 0 {
            counts.push(format!("rpm:{count}"));
        }
    }
    if Path::new("/var/lib/rpm/Packages").exists() {
        if let Ok(c) = std::process::Command::new("rpm")
            .arg("-qa")
            .output()
        {
            if c.status.success() {
                let count = String::from_utf8_lossy(&c.stdout).lines().count();
                if count > 0 {
                    counts.push(format!("rpm:{count}"));
                }
            }
        }
    }

    if counts.is_empty() {
        return Vec::new();
    }
    vec![ModuleResult { key: "Packages".to_string(), value: counts.join(" / ") }]
}

fn package_manager() -> Vec<ModuleResult> {
    let mut managers: Vec<String> = Vec::new();
    for (path, name) in &[
        ("/var/lib/dpkg", "dpkg"),
        ("/var/lib/pacman", "pacman"),
        ("/var/lib/rpm", "rpm"),
        ("/var/db/pkg", "ebuild"),
        ("/var/db/xbps", "xbps"),
        ("/var/lib/flatpak", "flatpak"),
        ("/var/lib/snapd", "snap"),
        ("/usr/bin/nix", "nix"),
        ("/usr/bin/apk", "apk"),
        ("/usr/bin/opkg", "opkg"),
        ("/opt/homebrew", "homebrew"),
    ] {
        if Path::new(path).exists() {
            managers.push(name.to_string());
        }
    }
    if managers.is_empty() {
        Vec::new()
    } else {
        vec![ModuleResult { key: "Package Manager".to_string(), value: managers.join(", ") }]
    }
}

// ── Shell ──

fn shell() -> Vec<ModuleResult> {
    let shell = std::env::var("SHELL").ok()
        .and_then(|s| {
            let name = std::path::Path::new(&s).file_name()?.to_str()?.to_string();
            Some(name)
        })
        .unwrap_or_else(|| "unknown".to_string());
    vec![ModuleResult { key: "Shell".to_string(), value: shell }]
}

// ── Display ──

fn display() -> Vec<ModuleResult> {
    let drm = Path::new("/sys/class/drm");
    if !drm.is_dir() {
        return Vec::new();
    }

    let resolutions: Vec<String> = std::fs::read_dir(drm).ok()
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().contains('-'))
        .filter_map(|e| {
            let modes_path = e.path().join("modes");
            let modes = std::fs::read_to_string(modes_path).ok()?;
            let first = modes.lines().next()?.to_string();
            Some(first)
        })
        .collect();

    if resolutions.is_empty() {
        return Vec::new();
    }
    vec![ModuleResult { key: "Display".to_string(), value: resolutions.join(", ") }]
}

// ── DE ──

fn de() -> Vec<ModuleResult> {
    let de = std::env::var("XDG_CURRENT_DESKTOP").ok()
        .or_else(|| std::env::var("DESKTOP_SESSION").ok())
        .or_else(|| std::env::var("GDMSESSION").ok());
    if let Some(d) = de {
        vec![ModuleResult { key: "DE".to_string(), value: d }]
    } else {
        Vec::new()
    }
}

// ── WM ──

fn wm_processes() -> Vec<String> {
    let known_wms = [
        "i3", "sway", "bspwm", "dwm", "qtile", "xmonad", "herbstluftwm",
        "openbox", "fluxbox", "blackbox", "icewm", "jwm", "fvwm",
        "awesome", "ratpoison", "stumpwm", "spectrwm", "dwmstatus",
        "hyprland", "wayfire", "river", "hikari",
    ];

    let mut found: Vec<String> = Vec::new();
    if let Ok(procs) = std::fs::read_dir("/proc") {
        for entry in procs.flatten() {
            let _pid: u32 = match entry.file_name().to_string_lossy().parse() {
                Ok(n) if n > 0 => n,
                _ => continue,
            };
            let cmdline_path = entry.path().join("cmdline");
            let cmdline = match std::fs::read_to_string(&cmdline_path) {
                Ok(c) => c.replace('\0', " "),
                Err(_) => continue,
            };
            let lower = cmdline.to_lowercase();
            for wm in &known_wms {
                if lower.contains(wm) && !found.contains(&wm.to_string()) {
                    found.push(wm.to_string());
                }
            }
        }
    }
    found
}

fn wm() -> Vec<ModuleResult> {
    let from_env = std::env::var("XDG_SESSION_TYPE").ok()
        .and_then(|t| {
            if t == "wayland" || t == "x11" {
                std::env::var("XDG_CURRENT_DESKTOP").ok()
                    .or_else(|| std::env::var("DESKTOP_SESSION").ok())
            } else {
                None
            }
        });

    if let Some(w) = from_env {
        return vec![ModuleResult { key: "WM".to_string(), value: w }];
    }

    let procs = wm_processes();
    if !procs.is_empty() {
        return vec![ModuleResult { key: "WM".to_string(), value: procs.join(", ") }];
    }

    Vec::new()
}

// ── GTK Settings ──

fn read_gtk_setting(key: &str) -> Option<String> {
    let config_dirs = [
        std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| format!("{}/.config", home())),
        format!("/etc/gtk-3.0"),
    ];

    for dir in &config_dirs {
        let path = Path::new(dir).join("gtk-3.0/settings.ini");
        if let Ok(content) = std::fs::read_to_string(&path) {
            for line in content.lines() {
                let trimmed = line.trim();
                if let Some(val) = trimmed.strip_prefix(key) {
                    if val.starts_with('=') {
                        return Some(val[1..].trim().trim_matches('"').to_string());
                    }
                }
            }
        }
        let path2 = Path::new(dir).join("gtk-4.0/settings.ini");
        if let Ok(content) = std::fs::read_to_string(&path2) {
            for line in content.lines() {
                let trimmed = line.trim();
                if let Some(val) = trimmed.strip_prefix(key) {
                    if val.starts_with('=') {
                        return Some(val[1..].trim().trim_matches('"').to_string());
                    }
                }
            }
        }
    }
    None
}

fn home() -> String {
    std::env::var("HOME").unwrap_or_else(|_| "/root".to_string())
}

fn wm_theme() -> Vec<ModuleResult> {
    let theme = read_gtk_setting("gtk-theme-name");
    if let Some(t) = theme {
        vec![ModuleResult { key: "WM Theme".to_string(), value: t }]
    } else {
        Vec::new()
    }
}

fn theme() -> Vec<ModuleResult> {
    let theme = read_gtk_setting("gtk-theme-name");
    if let Some(t) = theme {
        vec![ModuleResult { key: "Theme".to_string(), value: t }]
    } else {
        Vec::new()
    }
}

fn icons() -> Vec<ModuleResult> {
    let icon = read_gtk_setting("gtk-icon-theme-name");
    if let Some(i) = icon {
        vec![ModuleResult { key: "Icons".to_string(), value: i }]
    } else {
        Vec::new()
    }
}

fn font() -> Vec<ModuleResult> {
    let font = read_gtk_setting("gtk-font-name");
    if let Some(f) = font {
        vec![ModuleResult { key: "Font".to_string(), value: f }]
    } else {
        Vec::new()
    }
}

fn cursor() -> Vec<ModuleResult> {
    let cursor = read_gtk_setting("gtk-cursor-theme-name");
    if let Some(c) = cursor {
        vec![ModuleResult { key: "Cursor".to_string(), value: c }]
    } else {
        Vec::new()
    }
}

// ── Terminal ──

fn get_ppid() -> Option<u32> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    for line in status.lines() {
        if let Some(val) = line.strip_prefix("PPid:") {
            return val.trim().parse().ok();
        } else if let Some(val) = line.strip_prefix("PPid:\t") {
            return val.trim().parse().ok();
        }
    }
    None
}

fn get_process_name(pid: u32) -> Option<String> {
    let cmdline = std::fs::read_to_string(format!("/proc/{pid}/cmdline")).ok()?;
    let first = cmdline.split('\0').next()?;
    let name = Path::new(first).file_name()?.to_str()?.to_string();
    Some(name)
}

fn terminal() -> Vec<ModuleResult> {
    let ppid = match get_ppid() {
        Some(p) => p,
        None => return Vec::new(),
    };
    let mut current = ppid;
    let known_terms = [
        "gnome-terminal", "konsole", "xfce4-terminal", "lxterminal",
        "alacritty", "kitty", "wezterm", "foot", "st", "urxvt",
        "xterm", "rxvt", "termite", "tilix", "terminator",
        "windows-terminal", "wt", "tabby", "hyper", "iterm2",
        "terminal-apple", "tmux", "screen", "ghostty", "rio",
        "contour", "warp", "blackbox",
    ];

    let term_env = std::env::var("TERM").ok();
    if let Some(t) = &term_env {
        if t != "screen" && t != "tmux" && t != "dumb" && t != "vt100" && t != "xterm" {
            return vec![ModuleResult { key: "Terminal".to_string(), value: t.clone() }];
        }
    }

    for _ in 0..10 {
        if current <= 1 {
            break;
        }
        let name = get_process_name(current);
        if let Some(ref n) = name {
            let lower = n.to_lowercase();
            for term in &known_terms {
                if lower.contains(term) {
                    return vec![ModuleResult { key: "Terminal".to_string(), value: n.clone() }];
                }
            }
            if lower == "bash" || lower == "zsh" || lower == "fish" || lower == "sh" || lower == "dash" {
            } else if lower == "tmux" || lower == "screen" {
            }
        }
        let status = match std::fs::read_to_string(format!("/proc/{current}/status")).ok() {
            Some(s) => s,
            None => break,
        };
        let mut found_ppid = None;
        for line in status.lines() {
            if let Some(val) = line.strip_prefix("PPid:") {
                found_ppid = val.trim().parse().ok();
                break;
            }
        }
        match found_ppid {
            Some(p) => current = p,
            None => break,
        }
    }

    if let Some(t) = term_env {
        if t != "dumb" {
            return vec![ModuleResult { key: "Terminal".to_string(), value: t }];
        }
    }

    Vec::new()
}

fn terminal_size() -> Vec<ModuleResult> {
    let cols = std::env::var("COLUMNS").ok();
    let lines = std::env::var("LINES").ok();
    match (cols, lines) {
        (Some(c), Some(l)) => {
            vec![ModuleResult { key: "Terminal Size".to_string(), value: format!("{c}x{l}") }]
        }
        _ => {
            unsafe {
                let mut ws: libc::winsize = std::mem::zeroed();
                if libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut ws) == 0 && ws.ws_col > 0 {
                    vec![ModuleResult { key: "Terminal Size".to_string(), value: format!("{}x{}", ws.ws_col, ws.ws_row) }]
                } else {
                    Vec::new()
                }
            }
        }
    }
}

fn terminal_font() -> Vec<ModuleResult> {
    let term = terminal();
    if term.is_empty() {
        return Vec::new();
    }
    let name = &term[0].value;

    let font = match name.as_str() {
        "alacritty" => {
            let path = format!("{}/.config/alacritty/alacritty.toml", home());
            std::fs::read_to_string(&path).ok().and_then(|c| {
                c.lines()
                    .find(|l| l.contains("font") || l.contains("family"))
                    .map(|l| l.split('=').nth(1).unwrap_or(l).trim().trim_matches('"').to_string())
            })
        }
        "kitty" => read_gtk_setting("font-name").or_else(|| {
            let path = format!("{}/.config/kitty/kitty.conf", home());
            std::fs::read_to_string(&path).ok().and_then(|c| {
                c.lines()
                    .find(|l| l.starts_with("font_family") || l.starts_with("font"))
                    .map(|l| l.split_whitespace().skip(1).collect::<Vec<_>>().join(" "))
            })
        }),
        "gnome-terminal" | "konsole" | "xfce4-terminal" => {
            read_gtk_setting("monospace-font-name")
                .or_else(|| read_gtk_setting("gtk-monospace-font-name"))
        }
        _ => None,
    };

    if let Some(f) = font {
        vec![ModuleResult { key: "Terminal Font".to_string(), value: f }]
    } else {
        Vec::new()
    }
}

// ── CPU ──

fn cpu() -> Vec<ModuleResult> {
    let content = match std::fs::read_to_string("/proc/cpuinfo").ok() {
        Some(c) => c,
        None => return Vec::new(),
    };
    let mut model_name = None;
    let mut core_count = 0;

    for line in content.lines() {
        if let Some(val) = line.strip_prefix("model name") {
            if model_name.is_none() {
                let parts: Vec<&str> = val.splitn(2, ':').collect();
                if parts.len() == 2 {
                    model_name = Some(parts[1].trim().to_string());
                }
            }
        } else if let Some(_val) = line.strip_prefix("processor") {
            core_count += 1;
        }
    }

    if core_count == 0 {
        core_count = 1;
    }

    let value = if let Some(model) = model_name {
        if core_count > 1 {
            format!("{model} ({core_count})")
        } else {
            model
        }
    } else {
        format!("{core_count} cores")
    };

    vec![ModuleResult { key: "CPU".to_string(), value }]
}

fn cpu_frequency() -> Vec<ModuleResult> {
    let freq_paths = [
        "/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq",
        "/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_cur_freq",
    ];
    for p in &freq_paths {
        if let Ok(freq_str) = std::fs::read_to_string(p) {
            if let Ok(freq_khz) = freq_str.trim().parse::<f64>() {
                let ghz = freq_khz / 1_000_000.0;
                return vec![ModuleResult { key: "CPU Freq".to_string(), value: format!("{:.2} GHz", ghz) }];
            }
        }
    }
    if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if let Some(val) = line.strip_prefix("cpu MHz") {
                let parts: Vec<&str> = val.splitn(2, ':').collect();
                if parts.len() == 2 {
                    if let Ok(mhz) = parts[1].trim().parse::<f64>() {
                        return vec![ModuleResult { key: "CPU Freq".to_string(), value: format!("{:.2} GHz", mhz / 1000.0) }];
                    }
                }
            }
        }
    }
    Vec::new()
}

// ── GPU ──

fn gpu() -> Vec<ModuleResult> {
    let pci_path = Path::new("/sys/bus/pci/devices");
    if !pci_path.is_dir() {
        return Vec::new();
    }

    let mut gpus: Vec<String> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(pci_path) {
        for entry in entries.flatten() {
            let class_path = entry.path().join("class");
            let class = match read_first_line(&class_path) {
                Some(c) => c,
                None => continue,
            };
            if class.starts_with("0x0300") || class.starts_with("0x0302") || class.starts_with("0x0380") {
                let vendor = read_first_line(entry.path().join("vendor")).unwrap_or_default();
                let device = read_first_line(entry.path().join("device")).unwrap_or_default();
                let rev = read_first_line(entry.path().join("revision")).unwrap_or_default();
                gpus.push(format!("{vendor}:{device} rev {rev}"));
            }
        }
    }

    if gpus.is_empty() {
        return Vec::new();
    }
    vec![ModuleResult { key: "GPU".to_string(), value: gpus.join(" / ") }]
}

// ── Memory / Swap ──

fn parse_meminfo() -> Option<(String, String)> {
    let content = std::fs::read_to_string("/proc/meminfo").ok()?;
    let mut total_kb = 0u64;
    let mut avail_kb = 0u64;
    let mut swap_total_kb = 0u64;
    let mut swap_free_kb = 0u64;

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }
        let val: u64 = parts[1].parse().unwrap_or(0);
        if line.starts_with("MemTotal:") {
            total_kb = val;
        } else if line.starts_with("MemAvailable:") {
            avail_kb = val;
        } else if line.starts_with("SwapTotal:") {
            swap_total_kb = val;
        } else if line.starts_with("SwapFree:") {
            swap_free_kb = val;
        }
    }

    fn fmt_kb(kb: u64) -> String {
        if kb >= 1_048_576 {
            format!("{:.1} GiB", kb as f64 / 1_048_576.0)
        } else if kb >= 1024 {
            format!("{:.1} MiB", kb as f64 / 1024.0)
        } else {
            format!("{kb} KiB")
        }
    }

    let mem = if avail_kb > 0 && total_kb > 0 {
        let used_kb = total_kb.saturating_sub(avail_kb);
        format!("{} / {}", fmt_kb(used_kb), fmt_kb(total_kb))
    } else if total_kb > 0 {
        fmt_kb(total_kb)
    } else {
        String::new()
    };

    let swap = if swap_total_kb > 0 {
        let used_kb = swap_total_kb.saturating_sub(swap_free_kb);
        format!("{} / {}", fmt_kb(used_kb), fmt_kb(swap_total_kb))
    } else {
        String::new()
    };

    Some((mem, swap))
}

fn memory() -> Vec<ModuleResult> {
    if let Some((mem, _)) = parse_meminfo() {
        if !mem.is_empty() {
            return vec![ModuleResult { key: "Memory".to_string(), value: mem }];
        }
    }
    Vec::new()
}

fn swap() -> Vec<ModuleResult> {
    if let Some((_, swap)) = parse_meminfo() {
        if !swap.is_empty() {
            return vec![ModuleResult { key: "Swap".to_string(), value: swap }];
        }
    }
    Vec::new()
}

// ── Disk ──

fn disk() -> Vec<ModuleResult> {
    let cpath = match std::ffi::CString::new("/") {
        Ok(p) => p,
        Err(_) => return Vec::new(),
    };
    unsafe {
        let mut stat: libc::statvfs = std::mem::zeroed();
        if libc::statvfs(cpath.as_ptr(), &mut stat) != 0 {
            return Vec::new();
        }
        let block_size = stat.f_frsize as u64;
        let total = stat.f_blocks as u64 * block_size;
        let free = stat.f_bfree as u64 * block_size;
        let used = total.saturating_sub(free);

        fn fmt_bytes(b: u64) -> String {
            if b >= 1_073_741_824 {
                format!("{:.1} GiB", b as f64 / 1_073_741_824.0)
            } else if b >= 1_048_576 {
                format!("{:.1} MiB", b as f64 / 1_048_576.0)
            } else {
                format!("{b} B")
            }
        }

        vec![ModuleResult { key: "Disk".to_string(), value: format!("{} / {}", fmt_bytes(used), fmt_bytes(total)) }]
    }
}

fn physical_disk() -> Vec<ModuleResult> {
    let block = Path::new("/sys/block");
    if !block.is_dir() {
        return Vec::new();
    }

    let mut disks: Vec<String> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(block) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with("sd") || name_str.starts_with("nvme") || name_str.starts_with("mmcblk") || name_str.starts_with("vd") {
                let model_path = entry.path().join("device").join("model");
                if let Ok(model) = std::fs::read_to_string(&model_path) {
                    disks.push(format!("{}: {}", name_str, model.trim()));
                } else {
                    disks.push(name_str.to_string());
                }
            }
        }
    }

    if disks.is_empty() {
        Vec::new()
    } else {
        vec![ModuleResult { key: "Physical Disk".to_string(), value: disks.join(" / ") }]
    }
}

fn disk_io() -> Vec<ModuleResult> {
    let content = match read_first_line("/proc/diskstats") {
        Some(c) => c,
        None => return Vec::new(),
    };

    let mut total_read = 0u64;
    let mut total_write = 0u64;

    for line in content.lines().take(60) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 10 {
            if let Ok(r) = parts[5].parse::<u64>() {
                total_read += r;
            }
            if let Ok(w) = parts[9].parse::<u64>() {
                total_write += w;
            }
        }
    }

    fn fmt_sectors(s: u64) -> String {
        let bytes = s * 512;
        if bytes >= 1_073_741_824 {
            format!("{:.1} GiB", bytes as f64 / 1_073_741_824.0)
        } else if bytes >= 1_048_576 {
            format!("{:.1} MiB", bytes as f64 / 1_048_576.0)
        } else {
            format!("{bytes} B")
        }
    }

    vec![
        ModuleResult { key: "Disk Read".to_string(), value: fmt_sectors(total_read) },
        ModuleResult { key: "Disk Write".to_string(), value: fmt_sectors(total_write) },
    ]
}

// ── Local IP ──

fn local_ip() -> Vec<ModuleResult> {
    unsafe {
        let mut ifap: *mut libc::ifaddrs = std::ptr::null_mut();
        if libc::getifaddrs(&mut ifap) != 0 {
            return Vec::new();
        }

        let mut ips: Vec<String> = Vec::new();
        let mut curr = ifap;
        while !curr.is_null() {
            let ifa = &*curr;
            if !ifa.ifa_addr.is_null() {
                let family = (*ifa.ifa_addr).sa_family as i32;
                if family == libc::AF_INET {
                    let sin = &*(ifa.ifa_addr as *const libc::sockaddr_in);
                    let bytes = sin.sin_addr.s_addr.to_ne_bytes();
                    if bytes[0] != 127 {
                        ips.push(format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3]));
                    }
                } else if family == libc::AF_INET6 {
                    let sin6 = &*(ifa.ifa_addr as *const libc::sockaddr_in6);
                    let bytes = sin6.sin6_addr.s6_addr;
                    let is_loopback = bytes[..15].iter().all(|&b| b == 0) && bytes[15] == 1;
                    let is_link_local = bytes[0] == 0xfe && bytes[1] == 0x80;
                    if !is_loopback && !is_link_local {
                        let segments: Vec<String> = bytes.chunks(2).map(|c| {
                            format!("{:02x}{:02x}", c[0], c[1])
                        }).collect();
                        let ipv6 = segments.join(":");
                        ips.push(ipv6);
                    }
                }
            }
            curr = ifa.ifa_next;
        }
        libc::freeifaddrs(ifap);

        if ips.is_empty() {
            return Vec::new();
        }
        vec![ModuleResult { key: "Local IP".to_string(), value: ips.join(" / ") }]
    }
}

// ── Public IP ──

fn public_ip() -> Vec<ModuleResult> {
    let providers = ["https://api.ipify.org", "https://ident.me", "https://icanhazip.com"];
    for url in &providers {
        if let Ok(resp) = ureq::get(*url).call() {
            if let Ok(body) = resp.into_body().read_to_string() {
                let ip = body.trim().to_string();
                if !ip.is_empty() {
                    return vec![ModuleResult { key: "Public IP".to_string(), value: ip }];
                }
            }
        }
    }
    Vec::new()
}

// ── Battery ──

fn battery() -> Vec<ModuleResult> {
    let bat_path = Path::new("/sys/class/power_supply");
    if !bat_path.is_dir() {
        return Vec::new();
    }

    let mut results: Vec<ModuleResult> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(bat_path) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if !name_str.starts_with("BAT") {
                continue;
            }

            let capacity = read_first_line(entry.path().join("capacity"));
            let status = read_first_line(entry.path().join("status"));

            let value = match (&capacity, &status) {
                (Some(cap), Some(st)) => format!("{}% ({st})", cap),
                (Some(cap), None) => format!("{}%", cap),
                (None, Some(st)) => format!("({st})"),
                (None, None) => continue,
            };

            results.push(ModuleResult { key: name_str.to_string(), value });
        }
    }

    if results.is_empty() {
        return Vec::new();
    }

    if results.len() == 1 {
        results[0].key = "Battery".to_string();
    }
    results
}

fn battery_status() -> Vec<ModuleResult> {
    let bat_path = Path::new("/sys/class/power_supply");
    if !bat_path.is_dir() {
        return Vec::new();
    }

    let mut health_info: Vec<String> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(bat_path) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if !name_str.starts_with("BAT") {
                continue;
            }

            let energy_full = read_first_line(entry.path().join("energy_full"))
                .and_then(|s| s.parse::<f64>().ok());
            let energy_full_design = read_first_line(entry.path().join("energy_full_design"))
                .and_then(|s| s.parse::<f64>().ok());
            let voltage = read_first_line(entry.path().join("voltage_now"))
                .and_then(|s| s.parse::<f64>().ok());
            let technology = read_first_line(entry.path().join("technology"));

            if let (Some(full), Some(design)) = (energy_full, energy_full_design) {
                if design > 0.0 {
                    let health_pct = (full / design) * 100.0;
                    let tech = technology.unwrap_or_default();
                    health_info.push(format!("{}: {:.0}% health", name_str, health_pct));
                    if !tech.is_empty() {
                        health_info.push(tech);
                    }
                }
            }
            if let Some(v) = voltage {
                health_info.push(format!("{:.3} V", v / 1_000_000.0));
            }
        }
    }

    if health_info.is_empty() {
        Vec::new()
    } else {
        vec![ModuleResult { key: "Battery Status".to_string(), value: health_info.join(" / ") }]
    }
}

fn battery_cycles() -> Vec<ModuleResult> {
    let bat_path = Path::new("/sys/class/power_supply");
    if !bat_path.is_dir() {
        return Vec::new();
    }

    let mut cycles: Vec<String> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(bat_path) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if !name_str.starts_with("BAT") {
                continue;
            }
            if let Ok(c) = std::fs::read_to_string(entry.path().join("cycle_count")) {
                let count = c.trim();
                if let Ok(n) = count.parse::<u32>() {
                    cycles.push(format!("{}", n));
                }
            }
        }
    }

    if cycles.is_empty() {
        Vec::new()
    } else {
        vec![ModuleResult { key: "Battery Cycles".to_string(), value: cycles.join(" / ") }]
    }
}

fn power_adapter() -> Vec<ModuleResult> {
    let bat_path = Path::new("/sys/class/power_supply");
    if !bat_path.is_dir() {
        return Vec::new();
    }

    if let Ok(entries) = std::fs::read_dir(bat_path) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with("AC") || name_str.starts_with("ADP") {
                let online = read_first_line(entry.path().join("online"));
                if let Some(val) = online {
                    let status = if val == "1" { "Plugged In" } else { "Unplugged" };
                    return vec![ModuleResult { key: "Power Adapter".to_string(), value: status.to_string() }];
                }
            }
        }
    }
    Vec::new()
}

// ── Locale ──

fn locale() -> Vec<ModuleResult> {
    let locale = std::env::var("LANG").ok().or_else(|| {
        std::env::var("LC_ALL").ok()
    });

    if let Some(l) = locale {
        vec![ModuleResult { key: "Locale".to_string(), value: l }]
    } else {
        Vec::new()
    }
}

// ── Users ──

fn users() -> Vec<ModuleResult> {
    let content = match std::fs::read_to_string("/etc/passwd").ok() {
        Some(c) => c,
        None => return Vec::new(),
    };
    let users: Vec<&str> = content
        .lines()
        .filter(|l| {
            let parts: Vec<&str> = l.split(':').collect();
            parts.len() >= 7
                && parts[2] != "0"
                && parts[6] != "/usr/sbin/nologin"
                && parts[6] != "/sbin/nologin"
                && parts[6] != "/bin/false"
        })
        .map(|l| l.split(':').next().unwrap_or(""))
        .filter(|u| !u.is_empty())
        .collect();

    if users.is_empty() {
        Vec::new()
    } else {
        vec![ModuleResult { key: "Users".to_string(), value: format!("{} ({})", users.len(), users.join(", ")) }]
    }
}

// ── Sound ──

fn sound() -> Vec<ModuleResult> {
    let cards_path = Path::new("/proc/asound/cards");
    if !cards_path.exists() {
        return Vec::new();
    }

    let content = match std::fs::read_to_string(cards_path).ok() {
        Some(c) => c,
        None => return Vec::new(),
    };
    let mut cards: Vec<String> = Vec::new();
    for line in content.lines() {
        if line.contains('-') {
            let parts: Vec<&str> = line.splitn(2, " - ").collect();
            if parts.len() == 2 {
                cards.push(parts[1].trim().to_string());
            }
        }
    }

    if cards.is_empty() {
        Vec::new()
    } else {
        vec![ModuleResult { key: "Sound".to_string(), value: cards.join(" / ") }]
    }
}

// ── Bluetooth ──

fn bluetooth() -> Vec<ModuleResult> {
    let bt_path = Path::new("/sys/class/bluetooth");
    if bt_path.is_dir() {
        if let Ok(entries) = std::fs::read_dir(bt_path) {
            let count = entries.count();
            if count > 0 {
                return vec![ModuleResult { key: "Bluetooth".to_string(), value: format!("{count} adapter(s)") }];
            }
        }
    }
    Vec::new()
}

// ── Wifi ──

fn wifi() -> Vec<ModuleResult> {
    let wireless_path = Path::new("/proc/net/wireless");
    if wireless_path.exists() {
        if let Ok(content) = std::fs::read_to_string(wireless_path) {
            let interfaces: Vec<&str> = content.lines()
                .skip(2)
                .filter_map(|l| l.split_whitespace().next())
                .map(|s| s.trim_end_matches(':'))
                .collect();
            if !interfaces.is_empty() {
                return vec![ModuleResult { key: "WiFi".to_string(), value: interfaces.join(", ") }];
            }
        }
    }
    if let Ok(entries) = std::fs::read_dir("/sys/class/net") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("wl") {
                if let Ok(operstate) = std::fs::read_to_string(entry.path().join("operstate")) {
                    return vec![ModuleResult { key: "WiFi".to_string(), value: format!("{name} ({})", operstate.trim()) }];
                }
                return vec![ModuleResult { key: "WiFi".to_string(), value: name }];
            }
        }
    }
    Vec::new()
}

// ── Network IO ──

fn network_io() -> Vec<ModuleResult> {
    let content = match std::fs::read_to_string("/proc/net/dev").ok() {
        Some(c) => c,
        None => return Vec::new(),
    };
    let mut total_rx: u64 = 0;
    let mut total_tx: u64 = 0;

    for line in content.lines().skip(2) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 10 {
            if let Ok(rx) = parts[1].parse::<u64>() {
                total_rx += rx;
            }
            if let Ok(tx) = parts[9].parse::<u64>() {
                total_tx += tx;
            }
        }
    }

    fn fmt_bytes(b: u64) -> String {
        if b >= 1_073_741_824 {
            format!("{:.1} GiB", b as f64 / 1_073_741_824.0)
        } else if b >= 1_048_576 {
            format!("{:.1} MiB", b as f64 / 1_048_576.0)
        } else {
            format!("{b} B")
        }
    }

    vec![
        ModuleResult { key: "Network RX".to_string(), value: fmt_bytes(total_rx) },
        ModuleResult { key: "Network TX".to_string(), value: fmt_bytes(total_tx) },
    ]
}

// ── Media ──

fn media() -> Vec<ModuleResult> {
    let players = ["spotify", "mpd", "vlc", "firefox", "chromium", "chrome", "brave", "rhythmbox", "audacious", "cmus", "mpv"];
    let mut found: Vec<String> = Vec::new();

    if let Ok(procs) = std::fs::read_dir("/proc") {
        for entry in procs.flatten() {
            let cmdline_path = entry.path().join("cmdline");
            let cmdline = match std::fs::read_to_string(&cmdline_path) {
                Ok(c) => c.replace('\0', " ").to_lowercase(),
                Err(_) => continue,
            };
            for player in &players {
                if cmdline.contains(player) && !found.contains(&player.to_string()) {
                    found.push(player.to_string());
                }
            }
        }
    }

    if found.is_empty() {
        Vec::new()
    } else {
        vec![ModuleResult { key: "Media".to_string(), value: found.join(", ") }]
    }
}

// ── Monitor ──

fn monitor() -> Vec<ModuleResult> {
    let drm_path = Path::new("/sys/class/drm");
    if !drm_path.is_dir() {
        return Vec::new();
    }

    let mut monitors: Vec<String> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(drm_path) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.contains('-') || name.contains("card") {
                continue;
            }
            let edid_path = entry.path().join("edid");
            if edid_path.exists() {
                if let Ok(edid) = std::fs::read(&edid_path) {
                    if edid.len() >= 128 {
                        let manufacturer = String::from_utf8_lossy(&edid[0x08..0x0C]);
                        let product = u16::from_be_bytes([edid[0x0C], edid[0x0D]]);
                        monitors.push(format!("{manufacturer} {product}"));
                    }
                }
            } else {
                let status_path = entry.path().join("status");
                if let Ok(status) = std::fs::read_to_string(status_path) {
                    if status.trim() == "connected" {
                        let mode_path = entry.path().join("modes");
                        if let Ok(modes) = std::fs::read_to_string(mode_path) {
                            if let Some(first) = modes.lines().next() {
                                monitors.push(first.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    if monitors.is_empty() {
        Vec::new()
    } else {
        vec![ModuleResult { key: "Monitor".to_string(), value: monitors.join(", ") }]
    }
}

// ── Container ──

fn container() -> Vec<ModuleResult> {
    if Path::new("/.dockerenv").exists() {
        return vec![ModuleResult { key: "Container".to_string(), value: "Docker".to_string() }];
    }
    if let Ok(content) = std::fs::read_to_string("/proc/1/cgroup") {
        if content.contains("docker") {
            return vec![ModuleResult { key: "Container".to_string(), value: "Docker".to_string() }];
        }
        if content.contains("podman") {
            return vec![ModuleResult { key: "Container".to_string(), value: "Podman".to_string() }];
        }
        if content.contains("lxc") {
            return vec![ModuleResult { key: "Container".to_string(), value: "LXC".to_string() }];
        }
    }
    Vec::new()
}

// ── Virtualization ──

fn virtualization() -> Vec<ModuleResult> {
    if let Some(bios_vendor) = read_first_line("/sys/devices/virtual/dmi/id/bios_vendor") {
        let vendor = bios_vendor.to_lowercase();
        if vendor.contains("vmware") {
            return vec![ModuleResult { key: "Virtualization".to_string(), value: "VMware".to_string() }];
        }
        if vendor.contains("virtualbox") || vendor.contains("innotek") {
            return vec![ModuleResult { key: "Virtualization".to_string(), value: "VirtualBox".to_string() }];
        }
        if vendor.contains("qemu") || vendor.contains("bochs") {
            return vec![ModuleResult { key: "Virtualization".to_string(), value: "QEMU".to_string() }];
        }
        if vendor.contains("microsoft") {
            return vec![ModuleResult { key: "Virtualization".to_string(), value: "Hyper-V".to_string() }];
        }
    }
    if let Some(sys_vendor) = read_first_line("/sys/devices/virtual/dmi/id/sys_vendor") {
        let vendor = sys_vendor.to_lowercase();
        if vendor.contains("kvm") || vendor.contains("red hat") {
            return vec![ModuleResult { key: "Virtualization".to_string(), value: "KVM".to_string() }];
        }
    }
    if Path::new("/dev/kvm").exists() {
        return vec![ModuleResult { key: "Virtualization".to_string(), value: "KVM (guest)".to_string() }];
    }
    // Check CPU flags for hypervisor
    if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if let Some(val) = line.strip_prefix("flags") {
                if val.to_lowercase().contains("hypervisor") {
                    return vec![ModuleResult { key: "Virtualization".to_string(), value: "Unknown VM".to_string() }];
                }
                break;
            }
        }
    }
    Vec::new()
}

// ── Temperature ──

fn temperature() -> Vec<ModuleResult> {
    let thermal = Path::new("/sys/class/thermal");
    if !thermal.is_dir() {
        return Vec::new();
    }

    let mut temps: Vec<String> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(thermal) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("thermal_zone") {
                let temp_path = entry.path().join("temp");
                if let Ok(temp_str) = std::fs::read_to_string(&temp_path) {
                    if let Ok(temp_mc) = temp_str.trim().parse::<f64>() {
                        let temp_c = temp_mc / 1000.0;
                        let label = read_first_line(entry.path().join("type")).unwrap_or_default();
                        if temp_c > 0.0 && temp_c < 120.0 {
                            temps.push(format!("{}: {:.0}°C", label, temp_c));
                        }
                    }
                }
            }
        }
    }

    if temps.is_empty() {
        Vec::new()
    } else {
        vec![ModuleResult { key: "Temperature".to_string(), value: temps.join(" / ") }]
    }
}

// ── Fans ──

fn fans() -> Vec<ModuleResult> {
    let hwmon = Path::new("/sys/class/hwmon");
    if !hwmon.is_dir() {
        return Vec::new();
    }

    let mut fan_speeds: Vec<String> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(hwmon) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Ok(dir_entries) = std::fs::read_dir(&path) {
                for de in dir_entries.flatten() {
                    let fname = de.file_name().to_string_lossy().to_string();
                    if fname.starts_with("fan") && fname.ends_with("_input") {
                        if let Ok(speed) = std::fs::read_to_string(de.path()) {
                            let label_path = de.path().with_file_name(
                                fname.replace("_input", "_label")
                            );
                            let label = std::fs::read_to_string(label_path).unwrap_or_default();
                            let label = label.trim().to_string();
                            let name = if label.is_empty() { fname.clone() } else { label };
                            fan_speeds.push(format!("{}: {} RPM", name, speed.trim()));
                        }
                    }
                }
            }
        }
    }

    if fan_speeds.is_empty() {
        Vec::new()
    } else {
        vec![ModuleResult { key: "Fans".to_string(), value: fan_speeds.join(" / ") }]
    }
}

// ── Physical Memory ──

fn physical_memory() -> Vec<ModuleResult> {
    let mem_path = Path::new("/sys/devices/system/memory");
    if !mem_path.is_dir() {
        return Vec::new();
    }

    if let Ok(entries) = std::fs::read_dir(mem_path) {
        let count = entries
            .filter(|e| e.as_ref().is_ok_and(|e| e.file_name().to_string_lossy().starts_with("memory")))
            .count();
        if count > 0 {
            let total_gb = (count as f64 * 128.0) / 1024.0; // each memory block is 128MB on x86
            return vec![ModuleResult { key: "Physical Memory".to_string(), value: format!("{} blocks ({:.1} GiB)", count, total_gb) }];
        }
    }
    // Alternative: try dmidecode
    if let Ok(output) = std::process::Command::new("dmidecode")
        .arg("-t")
        .arg("memory")
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let count = stdout.matches("Memory Device").count();
        if count > 0 {
            return vec![ModuleResult { key: "Physical Memory".to_string(), value: format!("{} slots", count) }];
        }
    }
    Vec::new()
}

// ── Systemd ──

fn systemd_units() -> Vec<ModuleResult> {
    if Path::new("/run/systemd/system").exists() {
        if let Ok(entries) = std::fs::read_dir("/run/systemd/system") {
            let count = entries.count();
            return vec![ModuleResult { key: "Systemd".to_string(), value: format!("{count} units") }];
        }
    }
    Vec::new()
}

// ── Init System ──

fn init_system() -> Vec<ModuleResult> {
    if let Some(comm) = read_first_line("/proc/1/comm") {
        return vec![ModuleResult { key: "Init".to_string(), value: comm.trim().to_string() }];
    }
    Vec::new()
}

// ── OpenGL Version ──

fn opengl_version() -> Vec<ModuleResult> {
    // Try glxinfo first
    if let Ok(output) = std::process::Command::new("glxinfo")
        .arg("-B")
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("OpenGL version") || line.contains("OpenGL core profile version") {
                let version = line.split(':').nth(1).unwrap_or(line).trim().to_string();
                return vec![ModuleResult { key: "OpenGL".to_string(), value: version }];
            }
        }
    }
    // Check for libraries
    for lib in &["/usr/lib/libGL.so", "/usr/lib/x86_64-linux-gnu/libGL.so", "/usr/lib/aarch64-linux-gnu/libGL.so"] {
        if Path::new(lib).exists() {
            return vec![ModuleResult { key: "OpenGL".to_string(), value: "Present".to_string() }];
        }
    }
    Vec::new()
}

// ── Vulkan Version ──

fn vulkan_version() -> Vec<ModuleResult> {
    if let Ok(output) = std::process::Command::new("vulkaninfo")
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("Vulkan API version") || line.contains("apiVersion") {
                let version = line.split(':').nth(1).unwrap_or(line).trim().to_string();
                return vec![ModuleResult { key: "Vulkan".to_string(), value: version }];
            }
        }
        return vec![ModuleResult { key: "Vulkan".to_string(), value: "Present".to_string() }];
    }
    for lib in &["/usr/lib/libvulkan.so", "/usr/lib/x86_64-linux-gnu/libvulkan.so"] {
        if Path::new(lib).exists() || Path::new(&format!("{lib}.1")).exists() {
            return vec![ModuleResult { key: "Vulkan".to_string(), value: "Present".to_string() }];
        }
    }
    Vec::new()
}

// ── GTK Version ──

fn gtk_version() -> Vec<ModuleResult> {
    let mut versions: Vec<String> = Vec::new();
    for (ver, path) in &[("4.0", "/usr/lib/x86_64-linux-gnu/libgtk-4.so"), ("3", "/usr/lib/x86_64-linux-gnu/libgtk-3.so")] {
        if Path::new(path).exists() || Path::new(&format!("{path}.0")).exists() {
            versions.push(format!("{ver}"));
        }
    }
    // Check pkg-config
    for ver in &["4.0", "3"] {
        if let Ok(output) = std::process::Command::new("pkg-config")
            .args(["--modversion", &format!("gtk+-{ver}")])
            .output()
        {
            if output.status.success() {
                let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !versions.contains(&v) {
                    versions.push(v);
                }
            }
        }
    }
    if versions.is_empty() {
        Vec::new()
    } else {
        vec![ModuleResult { key: "GTK".to_string(), value: versions.join(", ") }]
    }
}

// ── Qt Version ──

fn qt_version() -> Vec<ModuleResult> {
    if let Ok(output) = std::process::Command::new("qmake")
        .arg("-query")
        .arg("QT_VERSION")
        .output()
    {
        if output.status.success() {
            let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !v.is_empty() {
                return vec![ModuleResult { key: "Qt".to_string(), value: v }];
            }
        }
    }
    if let Ok(output) = std::process::Command::new("qmake6")
        .arg("-query")
        .arg("QT_VERSION")
        .output()
    {
        if output.status.success() {
            let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !v.is_empty() {
                return vec![ModuleResult { key: "Qt".to_string(), value: v }];
            }
        }
    }
    for lib_path in &["/usr/lib/x86_64-linux-gnu/libQt5Core.so", "/usr/lib/x86_64-linux-gnu/libQt6Core.so"] {
        if Path::new(lib_path).exists() {
            let ver = if lib_path.contains("Qt5") { "5 (present)" } else { "6 (present)" };
            return vec![ModuleResult { key: "Qt".to_string(), value: ver.to_string() }];
        }
    }
    Vec::new()
}
