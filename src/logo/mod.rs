pub mod data;

pub struct Logo {
    pub lines: Vec<String>,
    pub width: usize,
    #[allow(dead_code)]
    pub height: usize,
}

impl Logo {
    pub fn from_entry(entry: &data::LogoEntry) -> Self {
        let raw_lines: Vec<&str> = entry.lines.split('\n').collect();
        let height = raw_lines.len();
        let rendered: Vec<String> = raw_lines.iter().map(|l| data::render_line(l, entry.colors)).collect();
        let width = rendered.iter().map(|l| visible_width(l)).max().unwrap_or(0);
        Self { lines: rendered, width, height }
    }
}

fn visible_width(s: &str) -> usize {
    let mut len = 0;
    let mut in_escape = false;
    for ch in s.chars() {
        if ch == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if ch == 'm' {
                in_escape = false;
            }
        } else {
            len += 1;
        }
    }
    len
}

fn detect_distro_id() -> &'static str {
    let content = std::fs::read_to_string("/etc/os-release").ok()
        .or_else(|| std::fs::read_to_string("/usr/lib/os-release").ok());

    let id = content.as_ref().and_then(|c| {
        for line in c.lines() {
            if let Some(val) = line.strip_prefix("ID=") {
                return Some(val.trim_matches('"').to_lowercase());
            }
        }
        None
    });

    match id.as_deref() {
        Some("ubuntu") => "ubuntu",
        Some("debian") => "debian",
        Some("arch") => "arch",
        Some("fedora") => "fedora",
        Some("nixos") => "nixos",
        Some("manjaro") => "manjaro",
        Some("void") => "void",
        Some("gentoo") => "gentoo",
        Some("alpine") => "alpine",
        Some("pop") => "pop",
        Some("linuxmint") => "linuxmint",
        Some(id) if id == "opensuse" || id.starts_with("opensuse-") => "opensuse",
        Some("centos") | Some("rhel") => "centos",
        Some("slackware") => "slackware",
        Some("solus") => "solus",
        Some("endeavouros") => "endeavouros",
        Some("artix") => "artix",
        Some("lubuntu") => "lubuntu",
        Some("kubuntu") => "kubuntu",
        Some("freebsd") => "freebsd",
        Some("tails") => "tails",
        Some("steamos") => "steamos",
        Some("raspbian") => "raspbian",
        Some("zorin") => "zorin",
        Some("elementary") => "elementary",
        Some("deepin") => "deepin",
        Some("linux") => "linux",
        _ => "linux",
    }
}

pub fn detect_distro_logo() -> Logo {
    let id = detect_distro_id();
    if let Some(entry) = data::resolve(id) {
        return Logo::from_entry(entry);
    }
    Logo::from_entry(&data::LINUX)
}
