use crate::info::ModuleResult;
use std::ffi::CStr;
use std::path::Path;

pub fn detect(name: &str) -> Vec<ModuleResult> {
    match name {
        "OS" => os(),
        "Host" => host(),
        "Kernel" => kernel(),
        "Uptime" => uptime(),
        "Packages" => packages(),
        "Shell" => shell(),
        "Display" => display(),
        "DE" => de(),
        "WM" => wm(),
        "WMTheme" => wm_theme(),
        "Theme" => theme(),
        "Icons" => icons(),
        "Font" => font(),
        "Cursor" => cursor(),
        "Terminal" => terminal(),
        "TerminalFont" => terminal_font(),
        "CPU" => cpu(),
        "GPU" => gpu(),
        "Memory" => memory(),
        "Swap" => swap(),
        "Disk" => disk(),
        "LocalIp" => local_ip(),
        "Battery" => battery(),
        "PowerAdapter" => power_adapter(),
        "Locale" => locale(),
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

fn host() -> Vec<ModuleResult> {
    let product = read_first_line("/sys/devices/virtual/dmi/id/product_name");
    let version = read_first_line("/sys/devices/virtual/dmi/id/product_version");
    let value = join_non_empty(&[product, version], " ");
    if value.is_empty() {
        return Vec::new();
    }
    vec![ModuleResult { key: "Host".to_string(), value }]
}

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

    if counts.is_empty() {
        return Vec::new();
    }
    vec![ModuleResult { key: "Packages".to_string(), value: counts.join(" / ") }]
}

fn shell() -> Vec<ModuleResult> {
    let shell = std::env::var("SHELL").ok()
        .and_then(|s| {
            let name = std::path::Path::new(&s).file_name()?.to_str()?.to_string();
            Some(name)
        })
        .unwrap_or_else(|| "unknown".to_string());
    vec![ModuleResult { key: "Shell".to_string(), value: shell }]
}

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
        "terminal-apple", "tmux", "screen",
    ];

    // First try $TERM as a fallback (useful in WSL/containers)
    let term_env = std::env::var("TERM").ok();
    if let Some(t) = &term_env {
        if t != "screen" && t != "tmux" && t != "dumb" && t != "vt100" && t != "xterm" {
            return vec![ModuleResult { key: "Terminal".to_string(), value: t.clone() }];
        }
    }

    for _ in 0..10 {
        if current <= 1 {
            // Reached PID 1 (init/systemd), stop
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
                // This is the shell, not the terminal. Continue up.
            } else if lower == "tmux" || lower == "screen" {
                // Multiplexer, continue up
            }
        }
        // Move to parent
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

    // Fallback to $TERM if available
    if let Some(t) = term_env {
        if t != "dumb" {
            return vec![ModuleResult { key: "Terminal".to_string(), value: t }];
        }
    }

    Vec::new()
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
                    // Skip link-local (fe80::) and loopback (::1)
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
            let _tech = read_first_line(entry.path().join("technology"));

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

    // If only one battery, use "Battery" as key
    if results.len() == 1 {
        results[0].key = "Battery".to_string();
    }
    results
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
