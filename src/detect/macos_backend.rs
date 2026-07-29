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

// ── Dispatch ──

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

// ── Module Implementations ──

fn os() -> Vec<ModuleResult> {
    let version = sysctl_str("kern.osproductversion");
    let value = match version {
        Some(v) => format!("macOS {v}"),
        None => "macOS".to_string(),
    };
    vec![ModuleResult { key: "OS".to_string(), value }]
}

fn host() -> Vec<ModuleResult> {
    let model = sysctl_str("hw.model");
    if let Some(m) = model {
        vec![ModuleResult { key: "Host".to_string(), value: m }]
    } else {
        Vec::new()
    }
}

fn kernel() -> Vec<ModuleResult> {
    let release = sysctl_str("kern.osrelease");
    match release {
        Some(r) => vec![ModuleResult { key: "Kernel".to_string(), value: r }],
        None => Vec::new(),
    }
}

fn uptime() -> Vec<ModuleResult> {
    // Get boot time via sysctl kern.boottime (struct timeval)
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
    if counts.is_empty() { return Vec::new(); }
    vec![ModuleResult { key: "Packages".to_string(), value: counts.join(" / ") }]
}

fn shell() -> Vec<ModuleResult> {
    let shell = std::env::var("SHELL").ok()
        .and_then(|s| Path::new(&s).file_name()?.to_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "unknown".to_string());
    vec![ModuleResult { key: "Shell".to_string(), value: shell }]
}

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

fn terminal() -> Vec<ModuleResult> {
    let ppid = unsafe { libc::getppid() };
    let known_terms = ["Terminal", "iTerm2", "Alacritty", "kitty", "WezTerm", "Hyper", "Tabby", "Warp"];

    // Walk process parent chain via proc_pidpath
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
            // shell, walk up
        } else if lower == "login" || lower == "loginwindow" || lower == "launchd" {
            break;
        }
        break; // simplified
    }
    if let Ok(term) = std::env::var("TERM") {
        if term != "dumb" {
            return vec![ModuleResult { key: "Terminal".to_string(), value: term }];
        }
    }
    Vec::new()
}

fn terminal_font() -> Vec<ModuleResult> { Vec::new() }

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

fn gpu() -> Vec<ModuleResult> {
    unsafe {
        let gpu_names = ["AGXAccelerator", "AMDRadeonAccelerator", "IntelAccelerator",
                         "AMDRadeonX5000", "AppleGraphicsControl", "AMDRadeonX6000"];
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
        gpus.dedup();
        if gpus.is_empty() { return Vec::new(); }
        vec![ModuleResult { key: "GPU".to_string(), value: gpus.join(" / ") }]
    }
}

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

fn power_adapter() -> Vec<ModuleResult> {
    Vec::new()
}

fn locale() -> Vec<ModuleResult> {
    let locale = std::env::var("LANG").ok().or_else(|| std::env::var("LC_ALL").ok());
    match locale {
        Some(l) => vec![ModuleResult { key: "Locale".to_string(), value: l }],
        None => Vec::new(),
    }
}
