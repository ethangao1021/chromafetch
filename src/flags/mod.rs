#[allow(dead_code)]
pub struct FlagColors {
    pub name: &'static str,
    pub colors: &'static [&'static str],
}

pub fn list_flags() -> Vec<&'static str> {
    PRESETS.iter().map(|f| f.name).collect()
}

#[allow(dead_code)]
pub fn get_flag(name: &str) -> Option<&'static FlagColors> {
    PRESETS.iter().find(|f| f.name.eq_ignore_ascii_case(name))
}

#[allow(dead_code)]
pub fn apply_flag_to_line(line: &str, _flag: &FlagColors) -> String {
    line.to_string()
}

pub const PRESETS: &[FlagColors] = &[
    FlagColors { name: "rainbow", colors: &["red", "yellow", "green", "cyan", "blue", "magenta"] },
    FlagColors { name: "trans", colors: &["cyan", "white", "white", "magenta", "cyan"] },
    FlagColors { name: "bisexual", colors: &["magenta", "purple", "blue"] },
    FlagColors { name: "pansexual", colors: &["magenta", "yellow", "cyan"] },
    FlagColors { name: "nonbinary", colors: &["yellow", "white", "magenta", "black"] },
    FlagColors { name: "aromantic", colors: &["green", "green", "white", "black", "black"] },
    FlagColors { name: "asexual", colors: &["black", "white", "white", "magenta"] },
    FlagColors { name: "lesbian", colors: &["red", "yellow", "white", "magenta", "purple"] },
    FlagColors { name: "gay", colors: &["blue", "cyan", "white", "green", "green"] },
    FlagColors { name: "progress", colors: &["blue", "magenta", "white", "yellow", "black"] },
];
