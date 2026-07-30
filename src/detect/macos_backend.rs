#![allow(non_camel_case_types, dead_code)]

use crate::info::ModuleResult;
use std::ffi::CStr;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

// ── Framework FFI ──
#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOServiceGetMatchingServices(mainPort: libc::c_uint, matching: *mut libc::c_void, existing: *mut libc::c_uint) -> libc::c_int;
    fn IOServiceNameMatching(name: *const libc::c_char) -> *mut libc::c_void;
    fn IOIteratorNext(iterator: libc::c_uint) -> libc::c_uint;
    fn IORegistryEntryCreateCFProperty(entry: libc::c_uint, key: *mut libc::c_void, allocator: *mut libc::c_void, options: u32) -> *mut libc::c_void;
    fn IOObjectRelease(object: libc::c_uint) -> libc::c_int;
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFStringCreateWithCString(alloc: *mut libc::c_void, cStr: *const libc::c_char, encoding: u32) -> *mut libc::c_void;
    fn CFStringGetCString(theString: *mut libc::c_void, buffer: *mut libc::c_char, bufferSize: usize, encoding: u32) -> u8;
    fn CFGetTypeID(cf: *mut libc::c_void) -> usize;
    fn CFStringGetTypeID() -> usize;
    fn CFNumberGetTypeID() -> usize;
    fn CFNumberGetValue(number: *mut libc::c_void, theType: u32, valuePtr: *mut libc::c_void) -> u8;
    fn CFRelease(cf: *mut libc::c_void);
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGMainDisplayID() -> u32;
    fn CGDisplayCopyDisplayMode(display: u32) -> *mut libc::c_void;
    fn CGDisplayModeGetPixelWidth(mode: *mut libc::c_void) -> usize;
    fn CGDisplayModeGetPixelHeight(mode: *mut libc::c_void) -> usize;
}

extern "C" {
    fn proc_pidpath(pid: libc::c_int, buffer: *mut libc::c_void, buffersize: u32) -> libc::c_int;
}

const kCFStringEncodingUTF8: u32 = 0x8000100;

// ── Helpers ──

fn sysctl_str(name: &str) -> Option<String> {
    let cname = std::ffi::CString::new(name).ok()?;
    let mut size: libc::size_t = 0;
    unsafe {
        if libc::sysctlbyname(cname.as_ptr(), std::ptr::null_mut(), &mut size, std::ptr::null_mut(), 0) != 0 {
            return None;
        }
        let mut buf: Vec<u8> = vec![0u8; size];
        if libc::sysctlbyname(cname.as_ptr(), buf.as_mut_ptr() as *mut libc::c_void, &mut size, std::ptr::null_mut(), 0) != 0 {
            return None;
        }
        Some(CStr::from_ptr(buf.as_ptr() as *const libc::c_char).to_string_lossy().into_owned())
    }
}

fn sysctl_u64(name: &str) -> Option<u64> {
    let cname = std::ffi::CString::new(name).ok()?;
    let mut val: u64 = 0;
    let mut size = std::mem::size_of::<u64>() as libc::size_t;
    unsafe {
        if libc::sysctlbyname(cname.as_ptr(), &mut val as *mut _ as *mut libc::c_void, &mut size, std::ptr::null_mut(), 0) != 0 {
            return None;
        }
        Some(val)
    }
}

fn sysctl_i32(name: &str) -> Option<i32> {
    let cname = std::ffi::CString::new(name).ok()?;
    let mut val: i32 = 0;
    let mut size = std::mem::size_of::<i32>() as libc::size_t;
    unsafe {
        if libc::sysctlbyname(cname.as_ptr(), &mut val as *mut _ as *mut libc::c_void, &mut size, std::ptr::null_mut(), 0) != 0 {
            return None;
        }
        Some(val)
    }
}

fn cfstring(s: &str) -> *mut libc::c_void {
    let cs = std::ffi::CString::new(s).unwrap();
    unsafe { CFStringCreateWithCString(std::ptr::null_mut(), cs.as_ptr(), kCFStringEncodingUTF8) }
}

fn cf_to_string(cf: *mut libc::c_void) -> Option<String> {
    if cf.is_null() { return None; }
    unsafe {
        if CFGetTypeID(cf) != CFStringGetTypeID() { CFRelease(cf); return None; }
        let mut buf = [0i8; 1024];
        if CFStringGetCString(cf, buf.as_mut_ptr(), buf.len(), kCFStringEncodingUTF8) != 0 {
            let s = CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned();
            CFRelease(cf);
            Some(s)
        } else {
            CFRelease(cf);
            None
        }
    }
}

fn read_first_line(path: impl AsRef<Path>) -> Option<String> {
    let content = std::fs::read_to_string(path.as_ref()).ok()?;
    content.lines().next().map(|s| s.trim().to_string())
}

fn io_service_string(service_name: &str, key: &str) -> Option<String> {
    unsafe {
        let matching = IOServiceNameMatching(std::ffi::CString::new(service_name).unwrap().as_ptr());
        if matching.is_null() { return None; }
        let mut iterator: libc::c_uint = 0;
        if IOServiceGetMatchingServices(0, matching, &mut iterator) != 0 { return None; }
        let svc = IOIteratorNext(iterator);
        IOObjectRelease(iterator);
        if svc == 0 { return None; }
        let cfkey = cfstring(key);
        if cfkey.is_null() { IOObjectRelease(svc); return None; }
        let val = IORegistryEntryCreateCFProperty(svc, cfkey, std::ptr::null_mut(), 0);
        CFRelease(cfkey);
        IOObjectRelease(svc);
        cf_to_string(val)
    }
}

// ── Dispatch ──

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
        "DE" => de(),
        "WM" => wm(),
        "WMTheme" => wm_theme(),
        "Theme" => theme(),
        "Icons" => icons(),
        "Font" => font(),
        "Cursor" => cursor(),
        "Terminal" => terminal_combined(),
        "CPU" => cpu_combined(),
        "GPU" => gpu_combined(),
        "Memory" => memory_combined(),
        "Disk" => disk(),
        "PhysicalDisk" => physical_disk_combined(),
        "LocalIp" => local_ip_combined(),
        "Network" => network_combined(),
        "Motherboard" => motherboard_combined(),
        "Sound" => sound(),
        "Monitor" => monitor(),
        "Battery" => battery_combined(),
        "Sensors" => sensors_combined(),
        "Users" => users(),
        "Locale" => locale(),
        "Media" => media(),
        "Virtualization" => virtualization_combined(),
        "InitSystem" => init_system(),
        "PackageManager" => package_manager(),
        "PhysicalMemory" => physical_memory(),
        "Libraries" => libraries_combined(),
        _ => vec![ModuleResult { key: name.to_string(), value: "unknown module".to_string() }],
    }
}

// ── Combined Modules ──

fn terminal_combined() -> Vec<ModuleResult> {
    let mut results = Vec::new();
    for r in terminal() {
        results.push(r);
    }
    let font = terminal_font();
    if !font.is_empty() {
        results.extend(font);
    }
    let size = terminal_size();
    if !size.is_empty() {
        results.extend(size);
    }
    let colorterm = std::env::var("COLORTERM").unwrap_or_default();
    if colorterm.contains("truecolor") || colorterm.contains("24bit") {
        results.push(ModuleResult { key: "Colors".to_string(), value: "truecolor".to_string() });
    } else {
        match std::env::var("TERM").unwrap_or_default().as_str() {
            t if t.contains("truecolor") || t.contains("24bit") => results.push(ModuleResult { key: "Colors".to_string(), value: "truecolor".to_string() }),
            t if t.contains("256color") => results.push(ModuleResult { key: "Colors".to_string(), value: "256color".to_string() }),
            _ => {}
        }
    }
    results
}

fn cpu_combined() -> Vec<ModuleResult> {
    let mut results = Vec::new();
    for r in cpu() {
        results.push(r);
    }
    let freq = cpu_frequency();
    if !freq.is_empty() {
        results.extend(freq);
    }
    results
}

fn gpu_combined() -> Vec<ModuleResult> {
    gpu()
}

fn memory_combined() -> Vec<ModuleResult> {
    let mut results = Vec::new();
    for r in memory() {
        results.push(r);
    }
    for r in swap() {
        results.push(r);
    }
    results
}

fn physical_disk_combined() -> Vec<ModuleResult> {
    physical_disk()
}

fn local_ip_combined() -> Vec<ModuleResult> {
    let mut results = Vec::new();
    for r in local_ip() {
        results.push(r);
    }
    let pub_ip = public_ip();
    if !pub_ip.is_empty() {
        results.extend(pub_ip);
    }
    results
}

fn network_combined() -> Vec<ModuleResult> {
    let mut results = Vec::new();
    for r in wifi() {
        results.push(r);
    }
    for r in bluetooth() {
        results.push(r);
    }
    for r in network_io() {
        results.push(r);
    }
    results
}

fn motherboard_combined() -> Vec<ModuleResult> {
    let mut results = Vec::new();
    for r in motherboard() {
        results.push(r);
    }
    for r in chassis() {
        results.push(r);
    }
    results
}

fn battery_combined() -> Vec<ModuleResult> {
    let mut results = Vec::new();
    for r in battery() {
        results.push(r);
    }
    for r in battery_status() {
        results.push(r);
    }
    for r in battery_cycles() {
        results.push(r);
    }
    results
}

fn sensors_combined() -> Vec<ModuleResult> {
    let mut results = Vec::new();
    for r in temperature() {
        results.push(r);
    }
    for r in fans() {
        results.push(r);
    }
    results
}

fn virtualization_combined() -> Vec<ModuleResult> {
    let mut results = Vec::new();
    for r in container() {
        results.push(r);
    }
    for r in virtualization() {
        results.push(r);
    }
    results
}

fn libraries_combined() -> Vec<ModuleResult> {
    let mut results = Vec::new();
    for r in opengl_version() {
        results.push(r);
    }
    for r in vulkan_version() {
        results.push(r);
    }
    for r in gtk_version() {
        results.push(r);
    }
    for r in qt_version() {
        results.push(r);
    }
    results
}

// ── OS ──

fn os() -> Vec<ModuleResult> {
    let version = sysctl_str("kern.osproductversion");
    let value = match version {
        Some(v) => format!("macOS {v}"),
        None => "macOS".to_string(),
    };
    vec![ModuleResult { key: "OS".to_string(), value }]
}

fn os_build() -> Vec<ModuleResult> {
    let build = sysctl_str("kern.osversion");
    if let Some(b) = build {
        vec![ModuleResult { key: "Build".to_string(), value: b }]
    } else {
        Vec::new()
    }
}

fn architecture() -> Vec<ModuleResult> {
    let mach = sysctl_str("hw.machine");
    if let Some(m) = mach {
        let mapped = match m.as_str() {
            "x86_64" => "x86_64",
            "arm64" => "ARM64",
            _ => &m,
        };
        vec![ModuleResult { key: "Architecture".to_string(), value: mapped.to_string() }]
    } else {
        Vec::new()
    }
}

// ── Host ──

fn host() -> Vec<ModuleResult> {
    let model = sysctl_str("hw.model");
    if let Some(m) = model {
        vec![ModuleResult { key: "Host".to_string(), value: m }]
    } else {
        Vec::new()
    }
}

fn motherboard() -> Vec<ModuleResult> {
    let board = io_service_string("AppleACPIPlatformExpert", "board-id");
    if let Some(b) = board {
        vec![ModuleResult { key: "Motherboard".to_string(), value: b }]
    } else {
        Vec::new()
    }
}

fn chassis() -> Vec<ModuleResult> {
    let model = sysctl_str("hw.model").unwrap_or_default();
    let lower = model.to_lowercase();
    if lower.contains("book") || lower.contains("macbook") {
        vec![ModuleResult { key: "Chassis".to_string(), value: "Laptop".to_string() }]
    } else if lower.contains("mini") || lower.contains("macmini") {
        vec![ModuleResult { key: "Chassis".to_string(), value: "Desktop".to_string() }]
    } else if lower.contains("pro") || lower.contains("macpro") || lower.contains("studio") {
        vec![ModuleResult { key: "Chassis".to_string(), value: "Desktop".to_string() }]
    } else if lower.contains("imac") {
        vec![ModuleResult { key: "Chassis".to_string(), value: "All-in-One".to_string() }]
    } else {
        vec![ModuleResult { key: "Chassis".to_string(), value: model }]
    }
}

// ── Kernel ──

fn kernel() -> Vec<ModuleResult> {
    let release = sysctl_str("kern.osrelease");
    match release {
        Some(r) => vec![ModuleResult { key: "Kernel".to_string(), value: r }],
        None => Vec::new(),
    }
}

// ── Uptime ──

fn uptime() -> Vec<ModuleResult> {
    let name = std::ffi::CString::new("kern.boottime").ok()?;
    let mut boottime: libc::timeval = unsafe { std::mem::zeroed() };
    let mut size = std::mem::size_of::<libc::timeval>() as libc::size_t;
    unsafe {
        if libc::sysctlbyname(name.as_ptr(), &mut boottime as *mut _ as *mut libc::c_void, &mut size, std::ptr::null_mut(), 0) != 0 {
            return Vec::new();
        }
    }
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?;
    let boot_secs = boottime.tv_sec as u64;
    let now_secs = now.as_secs();
    let secs = now_secs.saturating_sub(boot_secs);
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;
    let value = if days > 0 { format!("{days}d {hours}h {mins}m") }
                else if hours > 0 { format!("{hours}h {mins}m") }
                else { format!("{mins}m") };
    vec![ModuleResult { key: "Uptime".to_string(), value }]
}

// ── Processes ──

fn processes() -> Vec<ModuleResult> {
    let nprocs = sysctl_i32("kern.nprocs");
    if let Some(n) = nprocs {
        vec![ModuleResult { key: "Processes".to_string(), value: n.to_string() }]
    } else {
        Vec::new()
    }
}

// ── LoadAvg ──

fn load_avg() -> Vec<ModuleResult> {
    let mut loadavg: [f64; 3] = [0.0; 3];
    unsafe {
        libc::getloadavg(&mut loadavg as *mut f64, 3);
    }
    if loadavg[0] > 0.0 || loadavg[1] > 0.0 || loadavg[2] > 0.0 {
        vec![ModuleResult { key: "Load Average".to_string(), value: format!("{:.2} {:.2} {:.2}", loadavg[0], loadavg[1], loadavg[2]) }]
    } else {
        Vec::new()
    }
}

// ── Packages ──

fn packages() -> Vec<ModuleResult> {
    let mut counts: Vec<String> = Vec::new();
    for p in &["/usr/local/Cellar", "/opt/homebrew/Cellar"] {
        if let Ok(entries) = std::fs::read_dir(p) {
            let count = entries.count();
            if count > 0 { counts.push(format!("brew:{count}")); break; }
        }
    }
    if let Ok(entries) = std::fs::read_dir("/opt/local/var/macports/software") {
        let count = entries.count();
        if count > 0 { counts.push(format!("ports:{count}")); }
    }
    if Path::new("/usr/bin/apk").exists() {
        if let Ok(entries) = std::fs::read_dir("/etc/apk/cache") {
            let count = entries.count();
            if count > 0 { counts.push(format!("apk:{count}")); }
        }
    }
    if counts.is_empty() { return Vec::new(); }
    vec![ModuleResult { key: "Packages".to_string(), value: counts.join(" / ") }]
}

fn package_manager() -> Vec<ModuleResult> {
    let mut managers: Vec<String> = Vec::new();
    if Path::new("/usr/local/Cellar").exists() || Path::new("/opt/homebrew/Cellar").exists() {
        managers.push("homebrew".to_string());
    }
    if Path::new("/opt/local/var/macports").exists() {
        managers.push("macports".to_string());
    }
    if Path::new("/usr/bin/apk").exists() {
        managers.push("apk".to_string());
    }
    if Path::new("/usr/bin/nix").exists() {
        managers.push("nix".to_string());
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
        .and_then(|s| Path::new(&s).file_name()?.to_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "unknown".to_string());
    vec![ModuleResult { key: "Shell".to_string(), value: shell }]
}

// ── Display ──

fn display() -> Vec<ModuleResult> {
    unsafe {
        let main_id = CGMainDisplayID();
        if main_id == 0 { return Vec::new(); }
        let mode = CGDisplayCopyDisplayMode(main_id);
        if mode.is_null() { return Vec::new(); }
        let w = CGDisplayModeGetPixelWidth(mode);
        let h = CGDisplayModeGetPixelHeight(mode);
        CFRelease(mode);
        vec![ModuleResult { key: "Display".to_string(), value: format!("{w}x{h}") }]
    }
}

// ── DE / WM ──

fn de() -> Vec<ModuleResult> {
    vec![ModuleResult { key: "DE".to_string(), value: "Aqua".to_string() }]
}

fn wm() -> Vec<ModuleResult> {
    vec![ModuleResult { key: "WM".to_string(), value: "Aqua".to_string() }]
}

fn wm_theme() -> Vec<ModuleResult> { Vec::new() }
fn theme() -> Vec<ModuleResult> { Vec::new() }
fn icons() -> Vec<ModuleResult> { Vec::new() }
fn font() -> Vec<ModuleResult> { Vec::new() }
fn cursor() -> Vec<ModuleResult> { Vec::new() }

// ── Terminal ──

fn terminal() -> Vec<ModuleResult> {
    let ppid = unsafe { libc::getppid() };
    let known_terms = ["Terminal", "iTerm2", "Alacritty", "kitty", "WezTerm", "Hyper", "Tabby", "Warp", "Ghostty", "Rio", "Contour"];

    let mut current = ppid;
    for _ in 0..10 {
        if current <= 1 { break; }
        let mut buf = [0i8; 4096];
        let len = unsafe { proc_pidpath(current, buf.as_mut_ptr() as *mut libc::c_void, buf.len() as u32) };
        if len <= 0 { break; }
        let path = unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned() };
        let name = Path::new(&path).file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
        let lower = name.to_lowercase();
        for term in &known_terms {
            if lower.contains(&term.to_lowercase()) {
                return vec![ModuleResult { key: "Terminal".to_string(), value: name }];
            }
        }
        if lower == "bash" || lower == "zsh" || lower == "fish" || lower == "sh" || lower == "dash" {
        } else if lower == "login" || lower == "loginwindow" || lower == "launchd" {
            break;
        }
        break;
    }
    if let Ok(term) = std::env::var("TERM") {
        if term != "dumb" {
            return vec![ModuleResult { key: "Terminal".to_string(), value: term }];
        }
    }
    Vec::new()
}

fn terminal_size() -> Vec<ModuleResult> {
    if let (Ok(cols), Ok(lines)) = (std::env::var("COLUMNS"), std::env::var("LINES")) {
        vec![ModuleResult { key: "Terminal Size".to_string(), value: format!("{cols}x{lines}") }]
    } else {
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

fn terminal_font() -> Vec<ModuleResult> { Vec::new() }

// ── CPU ──

fn cpu() -> Vec<ModuleResult> {
    let brand = sysctl_str("machdep.cpu.brand_string");
    let cores = sysctl_i32("hw.ncpu");
    let value = match (brand, cores) {
        (Some(b), Some(c)) if c > 1 => format!("{b} ({c})"),
        (Some(b), _) => b,
        (None, Some(c)) => format!("{c} cores"),
        _ => return Vec::new(),
    };
    vec![ModuleResult { key: "CPU".to_string(), value }]
}

fn cpu_frequency() -> Vec<ModuleResult> {
    let freq = sysctl_u64("hw.cpufrequency");
    if let Some(f) = freq {
        let ghz = f as f64 / 1_000_000_000.0;
        vec![ModuleResult { key: "CPU Freq".to_string(), value: format!("{:.2} GHz", ghz) }]
    } else {
        // Try cpu frequency max
        let freq_max = sysctl_u64("hw.cpufrequency_max");
        if let Some(f) = freq_max {
            let ghz = f as f64 / 1_000_000_000.0;
            vec![ModuleResult { key: "CPU Freq".to_string(), value: format!("{:.2} GHz (max)", ghz) }]
        } else {
            Vec::new()
        }
    }
}

// ── GPU ──

fn gpu() -> Vec<ModuleResult> {
    unsafe {
        let gpu_names = ["AGXAccelerator", "AMDRadeonAccelerator", "IntelAccelerator",
                         "AMDRadeonX5000", "AppleGraphicsControl", "AMDRadeonX6000",
                         "AMDRadeonX6000F"];
        let mut gpus: Vec<String> = Vec::new();
        for gname in &gpu_names {
            let matching = IOServiceNameMatching(std::ffi::CString::new(*gname).unwrap().as_ptr());
            if matching.is_null() { continue; }
            let mut iterator: libc::c_uint = 0;
            if IOServiceGetMatchingServices(0, matching, &mut iterator) != 0 { continue; }
            let svc = IOIteratorNext(iterator);
            if svc != 0 {
                let key = cfstring("model");
                if !key.is_null() {
                    let val = IORegistryEntryCreateCFProperty(svc, key, std::ptr::null_mut(), 0);
                    if !val.is_null() {
                        if let Some(s) = cf_to_string(val) {
                            gpus.push(s.trim_matches('\0').trim().to_string());
                        }
                    }
                    CFRelease(key);
                }
                IOObjectRelease(svc);
            }
            IOObjectRelease(iterator);
        }
        if gpus.is_empty() {
            // fallback: try AppleGraphicsControl
            let matching = IOServiceNameMatching(std::ffi::CString::new("AppleGraphicsControl").unwrap().as_ptr());
            if !matching.is_null() {
                let mut iterator: libc::c_uint = 0;
                if IOServiceGetMatchingServices(0, matching, &mut iterator) == 0 {
                    let svc = IOIteratorNext(iterator);
                    if svc != 0 {
                        let key = cfstring("IOPropertyMatch");
                        if !key.is_null() {
                            let val = IORegistryEntryCreateCFProperty(svc, key, std::ptr::null_mut(), 0);
                            cf_to_string(val);
                            CFRelease(key);
                        }
                        IOObjectRelease(svc);
                    }
                    IOObjectRelease(iterator);
                }
            }
        }
        gpus.dedup();
        if gpus.is_empty() { return Vec::new(); }
        vec![ModuleResult { key: "GPU".to_string(), value: gpus.join(" / ") }]
    }
}

// ── Memory / Swap ──

fn memory() -> Vec<ModuleResult> {
    let total = sysctl_u64("hw.memsize");
    let total_bytes = match total {
        Some(t) => t,
        None => return Vec::new(),
    };
    let total_gib = total_bytes as f64 / 1_073_741_824.0;
    vec![ModuleResult { key: "Memory".to_string(), value: format!("{total_gib:.1} GiB") }]
}

fn swap() -> Vec<ModuleResult> {
    let swap_str = sysctl_str("vm.swapusage");
    if let Some(s) = swap_str {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let total = parts.get(2).unwrap_or(&"");
        let used = parts.get(5).unwrap_or(&"");
        vec![ModuleResult { key: "Swap".to_string(), value: format!("{used} / {total}") }]
    } else {
        Vec::new()
    }
}

// ── Disk ──

fn disk() -> Vec<ModuleResult> {
    let cpath = match std::ffi::CString::new("/") {
        Ok(p) => p,
        Err(_) => return Vec::new(),
    };
    unsafe {
        let mut stat: libc::statfs = std::mem::zeroed();
        if libc::statfs(cpath.as_ptr(), &mut stat) != 0 { return Vec::new(); }
        let block_size = stat.f_bsize as u64;
        let total = stat.f_blocks as u64 * block_size;
        let free = stat.f_bfree as u64 * block_size;
        let used = total.saturating_sub(free);
        fn fmt_bytes(b: u64) -> String {
            if b >= 1_073_741_824 { format!("{:.1} GiB", b as f64 / 1_073_741_824.0) }
            else if b >= 1_048_576 { format!("{:.1} MiB", b as f64 / 1_048_576.0) }
            else { format!("{b} B") }
        }
        vec![ModuleResult { key: "Disk".to_string(), value: format!("{} / {}", fmt_bytes(used), fmt_bytes(total)) }]
    }
}

fn physical_disk() -> Vec<ModuleResult> {
    let media_path = Path::new("/sys/block");
    if media_path.is_dir() {
        let mut disks: Vec<String> = Vec::new();
        if let Ok(entries) = std::fs::read_dir(media_path) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("disk") || name.starts_with("nvme") {
                    disks.push(name);
                }
            }
        }
        if !disks.is_empty() {
            return vec![ModuleResult { key: "Physical Disk".to_string(), value: disks.join(" / ") }];
        }
    }
    // Try diskutil
    if let Ok(output) = std::process::Command::new("diskutil")
        .args(["list", "-plist", "physical"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let count = stdout.matches("DeviceIdentifier").count();
        if count > 0 {
            return vec![ModuleResult { key: "Physical Disk".to_string(), value: format!("{count} disk(s)") }];
        }
    }
    Vec::new()
}

// ── Local IP ──

fn local_ip() -> Vec<ModuleResult> {
    unsafe {
        let mut ifap: *mut libc::ifaddrs = std::ptr::null_mut();
        if libc::getifaddrs(&mut ifap) != 0 { return Vec::new(); }
        let mut ips: Vec<String> = Vec::new();
        let mut curr = ifap;
        while !curr.is_null() {
            let ifa = &*curr;
            if !ifa.ifa_addr.is_null() && (*ifa.ifa_addr).sa_family as i32 == libc::AF_INET {
                let sin = &*(ifa.ifa_addr as *const libc::sockaddr_in);
                let bytes = sin.sin_addr.s_addr.to_ne_bytes();
                if bytes[0] != 127 {
                    ips.push(format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3]));
                }
            }
            curr = ifa.ifa_next;
        }
        libc::freeifaddrs(ifap);
        if ips.is_empty() { return Vec::new(); }
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
    unsafe {
        let matching = IOServiceNameMatching(std::ffi::CString::new("AppleSmartBattery").unwrap().as_ptr());
        if matching.is_null() { return Vec::new(); }
        let mut iterator: libc::c_uint = 0;
        if IOServiceGetMatchingServices(0, matching, &mut iterator) != 0 { return Vec::new(); }
        let service = IOIteratorNext(iterator);
        IOObjectRelease(iterator);
        if service == 0 { return Vec::new(); }

        let keys = ["Capacity", "CurrentCapacity", "TimeRemaining", "BatteryStatus"];
        let cf_keys: Vec<_> = keys.iter().map(|k| cfstring(k)).collect();
        let mut vals: Vec<i64> = Vec::new();

        for &ck in &cf_keys {
            if !ck.is_null() {
                let val = IORegistryEntryCreateCFProperty(service, ck, std::ptr::null_mut(), 0);
                if !val.is_null() {
                    if CFGetTypeID(val) == CFNumberGetTypeID() {
                        let mut n: i64 = 0;
                        CFNumberGetValue(val, 15, &mut n as *mut _ as *mut libc::c_void);
                        vals.push(n);
                    }
                    CFRelease(val);
                }
            }
        }
        for ck in cf_keys { if !ck.is_null() { CFRelease(ck); } }
        IOObjectRelease(service);

        if vals.len() >= 2 {
            let status = if vals.get(3) == Some(&1) { "Charging" } else { "Discharging" };
            vec![ModuleResult { key: "Battery".to_string(), value: format!("{}% ({})", vals[0], status) }]
        } else {
            Vec::new()
        }
    }
}

fn battery_status() -> Vec<ModuleResult> {
    unsafe {
        let matching = IOServiceNameMatching(std::ffi::CString::new("AppleSmartBattery").unwrap().as_ptr());
        if matching.is_null() { return Vec::new(); }
        let mut iterator: libc::c_uint = 0;
        if IOServiceGetMatchingServices(0, matching, &mut iterator) != 0 { return Vec::new(); }
        let service = IOIteratorNext(iterator);
        IOObjectRelease(iterator);
        if service == 0 { return Vec::new(); }

        let keys = ["AppleRawMaxCapacity", "AppleRawDesignCapacity", "Temperature", "Manufacturer"];
        let cf_keys: Vec<_> = keys.iter().map(|k| cfstring(k)).collect();
        let mut health_info: Vec<String> = Vec::new();

        for &ck in &cf_keys {
            if !ck.is_null() {
                let val = IORegistryEntryCreateCFProperty(service, ck, std::ptr::null_mut(), 0);
                if !val.is_null() {
                    if CFGetTypeID(val) == CFNumberGetTypeID() {
                        let mut n: i64 = 0;
                        CFNumberGetValue(val, 15, &mut n as *mut _ as *mut libc::c_void);
                        let key_str = cfstring(keys[&cf_keys as *const _ as usize % keys.len()]);
                        // just collect for now
                        drop(key_str);
                    } else if CFGetTypeID(val) == CFStringGetTypeID() {
                        if let Some(s) = cf_to_string(val) {
                            health_info.push(s);
                        }
                        continue;
                    }
                    CFRelease(val);
                }
            }
        }
        for ck in cf_keys { if !ck.is_null() { CFRelease(ck); } }
        IOObjectRelease(service);

        if health_info.is_empty() {
            Vec::new()
        } else {
            vec![ModuleResult { key: "Battery Status".to_string(), value: health_info.join(" / ") }]
        }
    }
}

fn battery_cycles() -> Vec<ModuleResult> {
    unsafe {
        let matching = IOServiceNameMatching(std::ffi::CString::new("AppleSmartBattery").unwrap().as_ptr());
        if matching.is_null() { return Vec::new(); }
        let mut iterator: libc::c_uint = 0;
        if IOServiceGetMatchingServices(0, matching, &mut iterator) != 0 { return Vec::new(); }
        let service = IOIteratorNext(iterator);
        IOObjectRelease(iterator);
        if service == 0 { return Vec::new(); }

        let key = cfstring("CycleCount");
        let val = IORegistryEntryCreateCFProperty(service, key, std::ptr::null_mut(), 0);
        CFRelease(key);
        IOObjectRelease(service);

        if !val.is_null() && CFGetTypeID(val) == CFNumberGetTypeID() {
            let mut n: i64 = 0;
            CFNumberGetValue(val, 15, &mut n as *mut _ as *mut libc::c_void);
            CFRelease(val);
            vec![ModuleResult { key: "Battery Cycles".to_string(), value: n.to_string() }]
        } else {
            if !val.is_null() { CFRelease(val); }
            Vec::new()
        }
    }
}

fn power_adapter() -> Vec<ModuleResult> { Vec::new() }

// ── Locale ──

fn locale() -> Vec<ModuleResult> {
    let locale = std::env::var("LANG").ok().or_else(|| std::env::var("LC_ALL").ok());
    match locale {
        Some(l) => vec![ModuleResult { key: "Locale".to_string(), value: l }],
        None => Vec::new(),
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
    let devices = io_service_string("IOAudioEngine", "IOAudioDeviceName");
    if let Some(d) = devices {
        vec![ModuleResult { key: "Sound".to_string(), value: d }]
    } else {
        Vec::new()
    }
}

// ── Bluetooth ──

fn bluetooth() -> Vec<ModuleResult> {
    let bt = io_service_string("IOBluetoothHCIController", "IOBluetoothDeviceName");
    if let Some(b) = bt {
        vec![ModuleResult { key: "Bluetooth".to_string(), value: b }]
    } else {
        Vec::new()
    }
}

// ── WiFi ──

fn wifi() -> Vec<ModuleResult> {
    if let Ok(output) = std::process::Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport")
        .arg("-I")
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("SSID") {
                let ssid = line.split(':').nth(1).unwrap_or("").trim().to_string();
                if !ssid.is_empty() {
                    return vec![ModuleResult { key: "WiFi".to_string(), value: format!("Connected to {ssid}") }];
                }
            }
        }
        // If airport fails, just say WiFi is available
        return vec![ModuleResult { key: "WiFi".to_string(), value: "Available".to_string() }];
    }
    // Check for WiFi hardware
    let hardware = io_service_string("IO80211Interface", "IOInterfaceName");
    if let Some(h) = hardware {
        vec![ModuleResult { key: "WiFi".to_string(), value: h }]
    } else {
        Vec::new()
    }
}

// ── Network IO ──

fn network_io() -> Vec<ModuleResult> {
    unsafe {
        let mut ifap: *mut libc::ifaddrs = std::ptr::null_mut();
        if libc::getifaddrs(&mut ifap) != 0 { return Vec::new(); }

        // We can't easily get IO stats from ifaddrs; use sysctl
        let mut mib: [i32; 6] = [
            libc::CTL_NET, libc::PF_ROUTE, 0, 0, libc::NET_RT_IFLIST2, 0
        ];
        let mut size: libc::size_t = 0;
        if libc::sysctl(mib.as_mut_ptr(), 6, std::ptr::null_mut(), &mut size, std::ptr::null_mut(), 0) != 0 {
            libc::freeifaddrs(ifap);
            return Vec::new();
        }
        let mut buf = vec![0u8; size];
        if libc::sysctl(mib.as_mut_ptr(), 6, buf.as_mut_ptr() as *mut libc::c_void, &mut size, std::ptr::null_mut(), 0) != 0 {
            libc::freeifaddrs(ifap);
            return Vec::new();
        }
        libc::freeifaddrs(ifap);

        // Parse the data for network IO
        let mut total_rx: u64 = 0;
        let mut total_tx: u64 = 0;
        let mut pos = 0;
        while pos < size as usize {
            let msghdr = &*(buf[pos..].as_ptr() as *const libc::if_msghdr);
            if msghdr.ifm_type == libc::RTM_IFINFO2 {
                let ifmsg = &*(buf[pos..].as_ptr() as *const libc::if_msghdr2);
                total_rx += ifmsg.ifm_data.ifi_ibytes;
                total_tx += ifmsg.ifm_data.ifi_obytes;
            }
            pos += msghdr.ifm_msglen as usize;
        }

        fn fmt_bytes(b: u64) -> String {
            if b >= 1_073_741_824 { format!("{:.1} GiB", b as f64 / 1_073_741_824.0) }
            else if b >= 1_048_576 { format!("{:.1} MiB", b as f64 / 1_048_576.0) }
            else { format!("{b} B") }
        }

        vec![
            ModuleResult { key: "Network RX".to_string(), value: fmt_bytes(total_rx) },
            ModuleResult { key: "Network TX".to_string(), value: fmt_bytes(total_tx) },
        ]
    }
}

// ── Media ──

fn media() -> Vec<ModuleResult> {
    let players = ["Spotify", "Music", "iTunes", "VLC", "IINA", "mpv"];
    let mut found: Vec<String> = Vec::new();

    if let Ok(output) = std::process::Command::new("pgrep").args(["-l"]).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let lower = line.to_lowercase();
            for player in &players {
                if lower.contains(&player.to_lowercase()) && !found.contains(&player.to_string()) {
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
    let edid_path = Path::new("/var/log/system.log");
    if edid_path.exists() {
        // Try to get display info via IOKit
        if let Ok(output) = std::process::Command::new("system_profiler")
            .args(["SPDisplaysDataType"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut monitors: Vec<String> = Vec::new();
            for line in stdout.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("Resolution:") || trimmed.starts_with("Display Type:") {
                    let val = trimmed.split(':').nth(1).unwrap_or("").trim().to_string();
                    if !val.is_empty() {
                        monitors.push(val);
                    }
                }
                if trimmed.starts_with("Vendor:") {
                    let val = trimmed.split(':').nth(1).unwrap_or("").trim().to_string();
                    if !val.is_empty() {
                        monitors.push(val);
                    }
                }
            }
            if !monitors.is_empty() {
                return vec![ModuleResult { key: "Monitor".to_string(), value: monitors.join(", ") }];
            }
        }
    }
    Vec::new()
}

// ── Container ──

fn container() -> Vec<ModuleResult> {
    if Path::new("/.dockerenv").exists() {
        return vec![ModuleResult { key: "Container".to_string(), value: "Docker".to_string() }];
    }
    if Path::new("/proc/1/cgroup").exists() {
        if let Ok(content) = std::fs::read_to_string("/proc/1/cgroup") {
            if content.contains("docker") {
                return vec![ModuleResult { key: "Container".to_string(), value: "Docker".to_string() }];
            }
        }
    }
    Vec::new()
}

// ── Virtualization ──

fn virtualization() -> Vec<ModuleResult> {
    // Check for Hypervisor framework
    let hv = sysctl_i32("kern.hv_vmm_present");
    if hv == Some(1) {
        return vec![ModuleResult { key: "Virtualization".to_string(), value: "Apple Hypervisor".to_string() }];
    }
    // Check for VM in CPU features
    if let Some(features) = sysctl_str("machdep.cpu.features") {
        let lower = features.to_lowercase();
        if lower.contains("hypervisor") {
            return vec![ModuleResult { key: "Virtualization".to_string(), value: "VM (guest)".to_string() }];
        }
    }
    Vec::new()
}

// ── Temperature ──

fn temperature() -> Vec<ModuleResult> {
    unsafe {
        let matching = IOServiceNameMatching(std::ffi::CString::new("AppleSMC").unwrap().as_ptr());
        if matching.is_null() { return Vec::new(); }
        let mut iterator: libc::c_uint = 0;
        if IOServiceGetMatchingServices(0, matching, &mut iterator) != 0 { return Vec::new(); }
        let svc = IOIteratorNext(iterator);
        IOObjectRelease(iterator);
        if svc == 0 { return Vec::new(); }
        IOObjectRelease(svc);
    }

    if let Ok(output) = std::process::Command::new("osx-cpu-temp").output() {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !stdout.is_empty() {
            return vec![ModuleResult { key: "Temperature".to_string(), value: stdout }];
        }
    }
    // Try reading from SMC keys via pmset
    if let Ok(output) = std::process::Command::new("pmset").args(["-g", "therm"]).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("CPU") || line.contains("cpu") || line.contains("degree") {
                let val = line.trim().to_string();
                if !val.is_empty() {
                    return vec![ModuleResult { key: "Temperature".to_string(), value: val }];
                }
            }
        }
    }
    Vec::new()
}

// ── Fans ──

fn fans() -> Vec<ModuleResult> {
    if let Ok(output) = std::process::Command::new("osx-cpu-temp").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("fan") || stdout.contains("Fan") {
            return vec![ModuleResult { key: "Fans".to_string(), value: stdout.trim().to_string() }];
        }
    }
    Vec::new()
}

// ── Physical Memory ──

fn physical_memory() -> Vec<ModuleResult> {
    if let Ok(output) = std::process::Command::new("system_profiler")
        .args(["SPMemoryDataType"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let count = stdout.matches("Memory Slot").count();
        if count > 0 {
            return vec![ModuleResult { key: "Physical Memory".to_string(), value: format!("{} slots", count) }];
        }
    }
    Vec::new()
}

// ── InitSystem ──

fn init_system() -> Vec<ModuleResult> {
    vec![ModuleResult { key: "Init".to_string(), value: "launchd".to_string() }]
}

// ── OpenGL ──

fn opengl_version() -> Vec<ModuleResult> {
    if let Ok(output) = std::process::Command::new("system_profiler")
        .args(["SPDisplaysDataType"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("OpenGL") || line.contains("Metal") {
                let val = line.split(':').nth(1).unwrap_or(line).trim().to_string();
                if !val.is_empty() {
                    return vec![ModuleResult { key: "OpenGL".to_string(), value: val }];
                }
            }
        }
        // Even if no version string, macOS always has OpenGL
        return vec![ModuleResult { key: "OpenGL".to_string(), value: "Present".to_string() }];
    }
    // Default since macOS always has OpenGL
    vec![ModuleResult { key: "OpenGL".to_string(), value: "Present".to_string() }]
}

// ── Vulkan ──

fn vulkan_version() -> Vec<ModuleResult> {
    if let Ok(output) = std::process::Command::new("vulkaninfo").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("Vulkan API version") || line.contains("apiVersion") {
                let version = line.split(':').nth(1).unwrap_or(line).trim().to_string();
                return vec![ModuleResult { key: "Vulkan".to_string(), value: version }];
            }
        }
        return vec![ModuleResult { key: "Vulkan".to_string(), value: "Present".to_string() }];
    }
    // MoltenVK might be installed
    if Path::new("/usr/local/lib/libMoltenVK.dylib").exists() || Path::new("/opt/homebrew/lib/libMoltenVK.dylib").exists() {
        return vec![ModuleResult { key: "Vulkan".to_string(), value: "MoltenVK".to_string() }];
    }
    Vec::new()
}

// ── GTK ──

fn gtk_version() -> Vec<ModuleResult> {
    if let Ok(output) = std::process::Command::new("pkg-config")
        .args(["--modversion", "gtk+-3.0"])
        .output()
    {
        if output.status.success() {
            let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !v.is_empty() {
                return vec![ModuleResult { key: "GTK".to_string(), value: v }];
            }
        }
    }
    if let Ok(output) = std::process::Command::new("pkg-config")
        .args(["--modversion", "gtk+-4.0"])
        .output()
    {
        if output.status.success() {
            let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !v.is_empty() {
                return vec![ModuleResult { key: "GTK".to_string(), value: v }];
            }
        }
    }
    Vec::new()
}

// ── Qt ──

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
    Vec::new()
}
