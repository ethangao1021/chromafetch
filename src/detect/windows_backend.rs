#![allow(non_camel_case_types, non_snake_case, dead_code)]

use crate::info::ModuleResult;
use std::path::Path;

// ── Win32 API Types ──

#[repr(C)]
struct RTL_OSVERSIONINFOW {
    dwOSVersionInfoSize: u32,
    dwMajorVersion: u32,
    dwMinorVersion: u32,
    dwBuildNumber: u32,
    dwPlatformId: u32,
    szCSDVersion: [u16; 128],
}

#[repr(C)]
struct MEMORYSTATUSEX {
    dwLength: u32,
    dwMemoryLoad: u32,
    ullTotalPhys: u64,
    ullAvailPhys: u64,
    ullTotalPageFile: u64,
    ullAvailPageFile: u64,
    ullTotalVirtual: u64,
    ullAvailVirtual: u64,
    ullAvailExtendedVirtual: u64,
}

#[repr(C)]
struct SYSTEM_INFO {
    wProcessorArchitecture: u16,
    wReserved: u16,
    dwPageSize: u32,
    lpMinimumApplicationAddress: *mut std::ffi::c_void,
    lpMaximumApplicationAddress: *mut std::ffi::c_void,
    dwActiveProcessorMask: usize,
    dwNumberOfProcessors: u32,
    dwProcessorType: u32,
    dwAllocationGranularity: u32,
    wProcessorLevel: u16,
    wProcessorRevision: u16,
}

#[repr(C)]
struct SYSTEM_POWER_STATUS {
    ACLineStatus: u8,
    BatteryFlag: u8,
    BatteryLifePercent: u8,
    Reserved1: u8,
    BatteryLifeTime: u32,
    BatteryFullLifeTime: u32,
}

#[repr(C)]
struct DISPLAY_DEVICEW {
    cb: u32,
    DeviceName: [u16; 32],
    DeviceString: [u16; 128],
    StateFlags: u32,
    DeviceID: [u16; 128],
    DeviceKey: [u16; 128],
}

// ── FFI Declarations ──

#[link(name = "kernel32")]
unsafe extern "system" {
    fn RtlGetVersion(lpVersionInformation: *mut RTL_OSVERSIONINFOW) -> i32;
    fn GetTickCount64() -> u64;
    fn GlobalMemoryStatusEx(lpBuffer: *mut MEMORYSTATUSEX) -> i32;
    fn GetDiskFreeSpaceExW(
        lpDirectoryName: *const u16,
        lpFreeBytesAvailable: *mut u64,
        lpTotalNumberOfBytes: *mut u64,
        lpTotalNumberOfFreeBytes: *mut u64,
    ) -> i32;
    fn GetSystemInfo(lpSystemInfo: *mut SYSTEM_INFO);
    fn GetComputerNameExW(NameType: u32, lpBuffer: *mut u16, lpnSize: *mut u32) -> i32;
    fn GetUserDefaultLocaleName(lpLocaleName: *mut u16, cchLocaleName: i32) -> i32;
    fn GetSystemPowerStatus(lpSystemPowerStatus: *mut SYSTEM_POWER_STATUS) -> i32;
    fn GetConsoleWindow() -> *mut std::ffi::c_void;
    fn GetWindowModuleFileNameW(hWnd: *mut std::ffi::c_void, lpszFileName: *mut u16, cchFileNameMax: u32) -> u32;
    fn GetModuleFileNameExW(hProcess: *mut std::ffi::c_void, hModule: *mut std::ffi::c_void, lpFilename: *mut u16, nSize: u32) -> u32;
    fn OpenProcess(dwDesiredAccess: u32, bInheritHandle: i32, dwProcessId: u32) -> *mut std::ffi::c_void;
    fn CloseHandle(hObject: *mut std::ffi::c_void) -> i32;
    fn GetCurrentProcessId() -> u32;
    fn GetStdHandle(nStdHandle: u32) -> *mut std::ffi::c_void;
    fn GetConsoleScreenBufferInfoEx(hConsoleOutput: *mut std::ffi::c_void, lpConsoleScreenBufferInfoEx: *mut std::ffi::c_void) -> i32;
    fn GetConsoleFontSize(hConsoleOutput: *mut std::ffi::c_void, nFont: u32) -> std::ffi::c_long;
    fn RegOpenKeyExW(hKey: *mut std::ffi::c_void, lpSubKey: *const u16, ulOptions: u32, samDesired: u32, phkResult: *mut *mut std::ffi::c_void) -> i32;
    fn RegQueryValueExW(hKey: *mut std::ffi::c_void, lpValueName: *const u16, lpReserved: *mut std::ffi::c_void, lpType: *mut u32, lpData: *mut u8, lpcbData: *mut u32) -> i32;
    fn RegCloseKey(hKey: *mut std::ffi::c_void) -> i32;
    fn CreateToolhelp32Snapshot(dwFlags: u32, th32ProcessID: u32) -> *mut std::ffi::c_void;
    fn Process32FirstW(hSnapshot: *mut std::ffi::c_void, lppe: *mut PROCESSENTRY32W) -> i32;
    fn Process32NextW(hSnapshot: *mut std::ffi::c_void, lppe: *mut PROCESSENTRY32W) -> i32;
}

#[link(name = "user32")]
unsafe extern "system" {
    fn EnumDisplayDevicesW(lpDevice: *const u16, iDevNum: u32, lpDisplayDevice: *mut DISPLAY_DEVICEW, dwFlags: u32) -> i32;
    fn EnumDisplaySettingsW(lpszDeviceName: *const u16, iModeNum: u32, lpDevMode: *mut std::ffi::c_void) -> i32;
}

#[link(name = "iphlpapi")]
unsafe extern "system" {
    fn GetAdaptersAddresses(
        Family: u32,
        Flags: u32,
        Reserved: *mut std::ffi::c_void,
        AdapterAddresses: *mut IP_ADAPTER_ADDRESSES_LH,
        SizePointer: *mut u32,
    ) -> u32;
}

// ── Win32 Extra Types ──

#[repr(C)]
struct PROCESSENTRY32W {
    dwSize: u32,
    cntUsage: u32,
    th32ProcessID: u32,
    th32DefaultHeapID: usize,
    th32ModuleID: u32,
    cntThreads: u32,
    th32ParentProcessID: u32,
    pcPriClassBase: i32,
    dwFlags: u32,
    szExeFile: [u16; 260],
}

#[repr(C)]
struct IP_ADAPTER_ADDRESSES_LH {
    Length: u32,
    IfIndex: u32,
    Next: *mut IP_ADAPTER_ADDRESSES_LH,
    AdapterName: *mut u8,
    FirstUnicastAddress: *mut IP_ADAPTER_UNICAST_ADDRESS_LH,
    // ... many more fields, we only need FirstUnicastAddress
}

#[repr(C)]
struct IP_ADAPTER_UNICAST_ADDRESS_LH {
    Length: u32,
    Flags: u32,
    Next: *mut IP_ADAPTER_UNICAST_ADDRESS_LH,
    Address: SOCKET_ADDRESS,
    // ...
}

#[repr(C)]
struct SOCKET_ADDRESS {
    lpSockaddr: *mut std::ffi::c_void,
    iSockaddrLength: i32,
}

#[repr(C)]
struct SOCKADDR_IN {
    sin_family: u16,
    sin_port: u16,
    sin_addr: IN_ADDR,
    sin_zero: [u8; 8],
}

#[repr(C)]
struct IN_ADDR {
    S_un: [u8; 4],
}

// ── Constants ──

const ERROR_SUCCESS: i32 = 0;
const NO_ERROR: u32 = 0;
const STATUS_SUCCESS: i32 = 0;
const TH32CS_SNAPPROCESS: u32 = 0x00000002;
const PROCESS_QUERY_INFORMATION: u32 = 0x0400;
const PROCESS_VM_READ: u32 = 0x0010;
const KEY_READ: u32 = 0x20019;
const REG_SZ: u32 = 1;
const REG_DWORD: u32 = 4;
const HKEY_LOCAL_MACHINE: *mut std::ffi::c_void = 0x80000002 as *mut _;
const HKEY_CURRENT_USER: *mut std::ffi::c_void = 0x80000001 as *mut _;
const AF_INET: u32 = 2;
const GAA_FLAG_SKIP_ANYCAST: u32 = 2;
const GAA_FLAG_SKIP_MULTICAST: u32 = 4;
const GAA_FLAG_SKIP_DNS_SERVER: u32 = 8;
const ERROR_BUFFER_OVERFLOW: u32 = 111;

// ── Helpers ──

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

fn from_wide(wide: &[u16]) -> String {
    let end = wide.iter().position(|&c| c == 0).unwrap_or(wide.len());
    String::from_utf16_lossy(&wide[..end])
}

fn read_registry_string(hkey: *mut std::ffi::c_void, subkey: &str, value: &str) -> Option<String> {
    let sk = to_wide(subkey);
    let vn = to_wide(value);
    unsafe {
        let mut hk: *mut std::ffi::c_void = std::ptr::null_mut();
        if RegOpenKeyExW(hkey, sk.as_ptr(), 0, KEY_READ, &mut hk) != ERROR_SUCCESS {
            return None;
        }
        let mut typ: u32 = 0;
        let mut size: u32 = 0;
        if RegQueryValueExW(hk, vn.as_ptr(), std::ptr::null_mut(), &mut typ, std::ptr::null_mut(), &mut size) != ERROR_SUCCESS {
            RegCloseKey(hk);
            return None;
        }
        let mut buf = vec![0u8; size as usize];
        if RegQueryValueExW(hk, vn.as_ptr(), std::ptr::null_mut(), &mut typ, buf.as_mut_ptr(), &mut size) != ERROR_SUCCESS {
            RegCloseKey(hk);
            return None;
        }
        RegCloseKey(hk);
        if typ == REG_SZ {
            let wide = std::slice::from_raw_parts(buf.as_ptr() as *const u16, size as usize / 2);
            Some(from_wide(wide))
        } else {
            None
        }
    }
}

fn read_registry_dword(hkey: *mut std::ffi::c_void, subkey: &str, value: &str) -> Option<u32> {
    let sk = to_wide(subkey);
    let vn = to_wide(value);
    unsafe {
        let mut hk: *mut std::ffi::c_void = std::ptr::null_mut();
        if RegOpenKeyExW(hkey, sk.as_ptr(), 0, KEY_READ, &mut hk) != ERROR_SUCCESS {
            return None;
        }
        let mut typ: u32 = 0;
        let mut size: u32 = 4;
        let mut data: u32 = 0;
        let rc = RegQueryValueExW(hk, vn.as_ptr(), std::ptr::null_mut(), &mut typ, &mut data as *mut u32 as *mut u8, &mut size);
        RegCloseKey(hk);
        if rc == ERROR_SUCCESS && typ == REG_DWORD { Some(data) } else { None }
    }
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
    unsafe {
        let mut ver: RTL_OSVERSIONINFOW = std::mem::zeroed();
        ver.dwOSVersionInfoSize = std::mem::size_of::<RTL_OSVERSIONINFOW>() as u32;
        if RtlGetVersion(&mut ver) != STATUS_SUCCESS {
            return Vec::new();
        }
        let edition = if ver.dwMajorVersion >= 10 {
            if ver.dwBuildNumber >= 22000 { "Windows 11" } else { "Windows 10" }
        } else if ver.dwMajorVersion == 6 && ver.dwMinorVersion == 3 {
            "Windows 8.1"
        } else if ver.dwMajorVersion == 6 && ver.dwMinorVersion == 2 {
            "Windows 8"
        } else if ver.dwMajorVersion == 6 && ver.dwMinorVersion == 1 {
            "Windows 7"
        } else {
            "Windows"
        };
        vec![ModuleResult { key: "OS".to_string(), value: format!("{edition} (build {})", ver.dwBuildNumber) }]
    }
}

fn host() -> Vec<ModuleResult> {
    unsafe {
        let mut buf = [0u16; 256];
        let mut size = buf.len() as u32;
        // Try ComputerNameNetBIOS (0) first, then ComputerNamePhysicalDnsHostname (2)
        if GetComputerNameExW(0, buf.as_mut_ptr(), &mut size) == 0 &&
           GetComputerNameExW(2, buf.as_mut_ptr(), &mut size) == 0 {
            return Vec::new();
        }
        let name = from_wide(&buf[..size as usize]);
        if name.is_empty() { return Vec::new(); }
        vec![ModuleResult { key: "Host".to_string(), value: name }]
    }
}

fn kernel() -> Vec<ModuleResult> {
    unsafe {
        let mut ver: RTL_OSVERSIONINFOW = std::mem::zeroed();
        ver.dwOSVersionInfoSize = std::mem::size_of::<RTL_OSVERSIONINFOW>() as u32;
        if RtlGetVersion(&mut ver) != STATUS_SUCCESS {
            return Vec::new();
        }
        vec![ModuleResult { key: "Kernel".to_string(), value: format!("{}.{}.{}", ver.dwMajorVersion, ver.dwMinorVersion, ver.dwBuildNumber) }]
    }
}

fn uptime() -> Vec<ModuleResult> {
    let ms = unsafe { GetTickCount64() };
    let secs = ms / 1000;
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

    // Chocolatey
    let choco = std::env::var("ChocolateyInstall").unwrap_or_else(|_| r"C:\ProgramData\chocolatey".to_string());
    let choco_lib = Path::new(&choco).join("lib");
    if let Ok(e) = std::fs::read_dir(&choco_lib) {
        let count = e.count();
        if count > 0 { counts.push(format!("choco:{count}")); }
    }

    // Scoop
    if let Ok(home) = std::env::var("USERPROFILE") {
        let scoop_apps = Path::new(&home).join("scoop").join("apps");
        if let Ok(e) = std::fs::read_dir(&scoop_apps) {
            let count = e.filter(|e| e.as_ref().is_ok_and(|e| e.file_type().is_ok_and(|t| t.is_dir()))).count();
            if count > 0 { counts.push(format!("scoop:{count}")); }
        }
    }

    if counts.is_empty() {
        // Fallback: check if winget is available
        if Path::new(r"C:\Users").exists() {
            // Windows package management isn't directory-based, just show total program count roughly
            if let Ok(e) = std::fs::read_dir(r"C:\Program Files") {
                let count = e.count();
                if count > 0 { counts.push(format!("progs:{count}")); }
            }
        }
    }

    if counts.is_empty() { return Vec::new(); }
    vec![ModuleResult { key: "Packages".to_string(), value: counts.join(" / ") }]
}

fn shell() -> Vec<ModuleResult> {
    let shell = std::env::var("ComSpec").ok()
        .and_then(|s| Path::new(&s).file_name()?.to_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "cmd".to_string());
    vec![ModuleResult { key: "Shell".to_string(), value: shell }]
}

fn display() -> Vec<ModuleResult> {
    unsafe {
        let mut dd: DISPLAY_DEVICEW = std::mem::zeroed();
        dd.cb = std::mem::size_of::<DISPLAY_DEVICEW>() as u32;

        let mut resolutions: Vec<String> = Vec::new();
        let mut dev_num = 0u32;
        while EnumDisplayDevicesW(std::ptr::null(), dev_num, &mut dd, 0) != 0 {
            if dd.StateFlags & 0x00000001 != 0 { // DISPLAY_DEVICE_ATTACHED_TO_DESKTOP
                let name = from_wide(&dd.DeviceName);
                // Get current settings for this display
                let mut dev_mode = [0u8; 220]; // DEVMODEW is ~220 bytes, we just need dmPelsWidth/Height
                if EnumDisplaySettingsW(to_wide(&name).as_ptr(), 0xffffffff, &mut dev_mode as *mut _ as *mut std::ffi::c_void) != 0 {
                    // Structure fields at specific offsets (x86_64)
                    // dmPelsWidth at offset 104 (0x68), dmPelsHeight at offset 108 (0x6C)
                    let w = u32::from_ne_bytes(dev_mode[104..108].try_into().unwrap_or([0; 4]));
                    let h = u32::from_ne_bytes(dev_mode[108..112].try_into().unwrap_or([0; 4]));
                    if w > 0 && h > 0 {
                        resolutions.push(format!("{w}x{h}"));
                    }
                }
            }
            dev_num += 1;
        }
        if resolutions.is_empty() { return Vec::new(); }
        vec![ModuleResult { key: "Display".to_string(), value: resolutions.join(", ") }]
    }
}

fn de() -> Vec<ModuleResult> {
    vec![ModuleResult { key: "DE".to_string(), value: "DWM".to_string() }]
}

fn wm() -> Vec<ModuleResult> {
    vec![ModuleResult { key: "WM".to_string(), value: "DWM".to_string() }]
}

fn wm_theme() -> Vec<ModuleResult> {
    let theme_path = read_registry_string(HKEY_CURRENT_USER,
        r"Software\Microsoft\Windows\CurrentVersion\Themes", "CurrentTheme");
    if let Some(p) = theme_path {
        if let Some(name) = Path::new(&p).file_stem().and_then(|s| s.to_str()) {
            return vec![ModuleResult { key: "WM Theme".to_string(), value: name.to_string() }];
        }
    }
    Vec::new()
}

fn theme() -> Vec<ModuleResult> {
    // Check dark mode: HKCU\...\Personalize\AppsUseLightTheme (0=dark, 1=light)
    let light = read_registry_dword(HKEY_CURRENT_USER,
        r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize", "AppsUseLightTheme");
    let mode = match light {
        Some(0) => "Dark",
        Some(1) => "Light",
        _ => return Vec::new(),
    };
    vec![ModuleResult { key: "Theme".to_string(), value: mode.to_string() }]
}

fn icons() -> Vec<ModuleResult> {
    Vec::new()
}

fn font() -> Vec<ModuleResult> {
    vec![ModuleResult { key: "Font".to_string(), value: "Segoe UI".to_string() }]
}

fn cursor() -> Vec<ModuleResult> {
    let cursor = read_registry_string(HKEY_CURRENT_USER,
        r"Control Panel\Cursors", "Scheme");
    if let Some(c) = cursor {
        vec![ModuleResult { key: "Cursor".to_string(), value: c }]
    } else {
        Vec::new()
    }
}

fn terminal() -> Vec<ModuleResult> {
    // First try environment variables (works in Windows Terminal, VSCode, etc.)
    let from_env = std::env::var("WT_SESSION").ok()
        .map(|_| "Windows Terminal".to_string())
        .or_else(|| std::env::var("TERM_PROGRAM").ok())
        .or_else(|| {
            let term = std::env::var("TERM").ok()?;
            if term != "dumb" && term != "xterm" { Some(term) } else { None }
        });
    if let Some(t) = from_env { return vec![ModuleResult { key: "Terminal".to_string(), value: t }]; }

    unsafe {
        let hwnd = GetConsoleWindow();
        if hwnd.is_null() { return Vec::new(); }

        let mut buf = [0u16; 4096];
        let len = GetWindowModuleFileNameW(hwnd, buf.as_mut_ptr(), buf.len() as u32);
        if len == 0 { return Vec::new(); }

        let path = from_wide(&buf[..len as usize]);
        let name = Path::new(&path).file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.trim_end_matches(".exe").to_string())
            .unwrap_or_else(|| "Console Host".to_string());
        vec![ModuleResult { key: "Terminal".to_string(), value: name }]
    }
}

fn terminal_font() -> Vec<ModuleResult> {
    // Windows Terminal font from settings
    let wt_settings = Path::new(&std::env::var("LOCALAPPDATA").unwrap_or_default())
        .join(r"Packages\Microsoft.WindowsTerminal_8wekyb3d8bbwe\LocalState\settings.json");
    if wt_settings.exists() {
        if let Ok(content) = std::fs::read_to_string(&wt_settings) {
            if let Some(line) = content.lines().find(|l| l.contains("fontFace") || l.contains("font")) {
                let font = line.split(':').nth(1)
                    .map(|s| s.trim().trim_matches(',').trim_matches('"').to_string());
                if let Some(f) = font {
                    return vec![ModuleResult { key: "Terminal Font".to_string(), value: f }];
                }
            }
        }
    }
    // Legacy console: read registry
    let font = read_registry_string(HKEY_CURRENT_USER,
        r"Console", "FaceName");
    if let Some(f) = font {
        vec![ModuleResult { key: "Terminal Font".to_string(), value: f.trim_matches('\0').to_string() }]
    } else {
        Vec::new()
    }
}

fn cpu() -> Vec<ModuleResult> {
    // Get CPU brand from registry
    let brand = read_registry_string(HKEY_LOCAL_MACHINE,
        r"HARDWARE\DESCRIPTION\System\CentralProcessor\0", "ProcessorNameString");
    unsafe {
        let mut si: SYSTEM_INFO = std::mem::zeroed();
        GetSystemInfo(&mut si);
        let cores = si.dwNumberOfProcessors;

        let value = match brand {
            Some(b) if cores > 1 => format!("{b} ({cores})"),
            Some(b) => b,
            None => format!("{cores} cores"),
        };
        vec![ModuleResult { key: "CPU".to_string(), value }]
    }
}

fn gpu() -> Vec<ModuleResult> {
    unsafe {
        let mut dd: DISPLAY_DEVICEW = std::mem::zeroed();
        dd.cb = std::mem::size_of::<DISPLAY_DEVICEW>() as u32;

        let mut gpus: Vec<String> = Vec::new();
        let mut dev_num = 0u32;
        while EnumDisplayDevicesW(std::ptr::null(), dev_num, &mut dd, 0) != 0 {
            let name = from_wide(&dd.DeviceString).trim().to_string();
            if !name.is_empty() && !gpus.contains(&name) {
                gpus.push(name);
            }
            dev_num += 1;
        }
        if gpus.is_empty() { return Vec::new(); }
        vec![ModuleResult { key: "GPU".to_string(), value: gpus.join(" / ") }]
    }
}

fn memory() -> Vec<ModuleResult> {
    unsafe {
        let mut mem: MEMORYSTATUSEX = std::mem::zeroed();
        mem.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
        if GlobalMemoryStatusEx(&mut mem) == 0 { return Vec::new(); }

        let total = mem.ullTotalPhys;
        let avail = mem.ullAvailPhys;
        let used = total.saturating_sub(avail);

        fn fmt(b: u64) -> String {
            if b >= 1_073_741_824 { format!("{:.1} GiB", b as f64 / 1_073_741_824.0) }
            else if b >= 1_048_576 { format!("{:.1} MiB", b as f64 / 1_048_576.0) }
            else { format!("{b} B") }
        }
        vec![ModuleResult { key: "Memory".to_string(), value: format!("{} / {}", fmt(used), fmt(total)) }]
    }
}

fn swap() -> Vec<ModuleResult> {
    unsafe {
        let mut mem: MEMORYSTATUSEX = std::mem::zeroed();
        mem.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
        if GlobalMemoryStatusEx(&mut mem) == 0 { return Vec::new(); }

        let total = mem.ullTotalPageFile;
        let avail = mem.ullAvailPageFile;
        let used = total.saturating_sub(avail);

        fn fmt(b: u64) -> String {
            if b >= 1_073_741_824 { format!("{:.1} GiB", b as f64 / 1_073_741_824.0) }
            else { format!("{:.0} MiB", b as f64 / 1_048_576.0) }
        }
        vec![ModuleResult { key: "Swap".to_string(), value: format!("{} / {}", fmt(used), fmt(total)) }]
    }
}

fn disk() -> Vec<ModuleResult> {
    unsafe {
        let path = to_wide(r"C:\");
        let mut free_user: u64 = 0;
        let mut total: u64 = 0;
        let mut free_total: u64 = 0;
        if GetDiskFreeSpaceExW(path.as_ptr(), &mut free_user, &mut total, &mut free_total) == 0 {
            return Vec::new();
        }
        let used = total.saturating_sub(free_total);
        fn fmt(b: u64) -> String {
            if b >= 1_073_741_824 { format!("{:.1} GiB", b as f64 / 1_073_741_824.0) }
            else if b >= 1_048_576 { format!("{:.1} MiB", b as f64 / 1_048_576.0) }
            else { format!("{b} B") }
        }
        vec![ModuleResult { key: "Disk".to_string(), value: format!("{} / {}", fmt(used), fmt(total)) }]
    }
}

fn local_ip() -> Vec<ModuleResult> {
    unsafe {
        let mut buf_len: u32 = 0;
        let rc = GetAdaptersAddresses(AF_INET, GAA_FLAG_SKIP_ANYCAST | GAA_FLAG_SKIP_MULTICAST | GAA_FLAG_SKIP_DNS_SERVER,
            std::ptr::null_mut(), std::ptr::null_mut(), &mut buf_len);
        if rc != ERROR_BUFFER_OVERFLOW { return Vec::new(); }

        let mut buf = vec![0u8; buf_len as usize];
        let adapters = buf.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH;
        let rc2 = GetAdaptersAddresses(AF_INET,
            GAA_FLAG_SKIP_ANYCAST | GAA_FLAG_SKIP_MULTICAST | GAA_FLAG_SKIP_DNS_SERVER,
            std::ptr::null_mut(), adapters, &mut buf_len);
        if rc2 != NO_ERROR { return Vec::new(); }

        let mut ips: Vec<String> = Vec::new();
        let mut aa = adapters;
        while !aa.is_null() {
            let mut ua = (*aa).FirstUnicastAddress;
            while !ua.is_null() {
                let sa = &(*ua).Address;
                if sa.iSockaddrLength as usize >= std::mem::size_of::<SOCKADDR_IN>() {
                    let sin = &*(sa.lpSockaddr as *const SOCKADDR_IN);
                    if sin.sin_family == AF_INET as u16 {
                        let bytes = sin.sin_addr.S_un;
                        if bytes[0] != 127 && !(bytes[0] == 169 && bytes[1] == 254) {
                            ips.push(format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3]));
                        }
                    }
                }
                ua = (*ua).Next;
            }
            aa = (*aa).Next;
        }
        if ips.is_empty() { return Vec::new(); }
        vec![ModuleResult { key: "Local IP".to_string(), value: ips.join(" / ") }]
    }
}

fn battery() -> Vec<ModuleResult> {
    unsafe {
        let mut ps: SYSTEM_POWER_STATUS = std::mem::zeroed();
        if GetSystemPowerStatus(&mut ps) == 0 { return Vec::new(); }
        if ps.BatteryFlag == 0xFF { return Vec::new(); } // No battery

        let pct = ps.BatteryLifePercent;
        let status = match ps.ACLineStatus {
            1 => "Charging",
            _ => "Discharging",
        };
        if pct <= 100 {
            vec![ModuleResult { key: "Battery".to_string(), value: format!("{pct}% ({status})") }]
        } else {
            Vec::new()
        }
    }
}

fn power_adapter() -> Vec<ModuleResult> {
    unsafe {
        let mut ps: SYSTEM_POWER_STATUS = std::mem::zeroed();
        if GetSystemPowerStatus(&mut ps) == 0 { return Vec::new(); }
        let status = match ps.ACLineStatus {
            1 => "Plugged In",
            0 => "Unplugged",
            _ => return Vec::new(),
        };
        vec![ModuleResult { key: "Power Adapter".to_string(), value: status.to_string() }]
    }
}

fn locale() -> Vec<ModuleResult> {
    unsafe {
        let mut buf = [0u16; 128];
        let len = GetUserDefaultLocaleName(buf.as_mut_ptr(), buf.len() as i32);
        if len == 0 { return Vec::new(); }
        let lang = from_wide(&buf);
        vec![ModuleResult { key: "Locale".to_string(), value: lang }]
    }
}
