use crate::info::SystemInfo;
use crate::logo::Logo;
use crate::config::DisplaySection;

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

fn strip_ansi(s: &str) -> String {
    let mut out = String::new();
    let mut in_escape = false;
    for ch in s.chars() {
        if ch == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if ch == 'm' {
                in_escape = false;
            }
        } else {
            out.push(ch);
        }
    }
    out
}

fn pad_to_visible(s: &str, width: usize) -> String {
    let vis = visible_width(s);
    if vis >= width {
        s.to_string()
    } else {
        let pad = " ".repeat(width - vis);
        format!("{s}{pad}")
    }
}

pub fn render(info: &SystemInfo, logo: &Logo, display_cfg: &DisplaySection, module_order: &[String], no_color: bool) {
    let logo_lines = &logo.lines;
    let logo_width = logo.width;
    let logo_height = logo_lines.len();

    let mut result_lines: Vec<String> = Vec::new();

    for module_name in module_order {
        if let Some(results) = info.modules.get(module_name) {
            for res in results {
                let line = match module_name.as_str() {
                    "Title" => {
                        if no_color {
                            res.value.clone()
                        } else {
                            format!("\x1b[1m{}\x1b[0m", res.value)
                        }
                    }
                    "Separator" => {
                        if no_color {
                            res.value.clone()
                        } else {
                            format!("\x1b[90m{}\x1b[0m", res.value)
                        }
                    }
                    "Break" => String::new(),
                    "Colors" if no_color => res.value.clone(),
                    "Colors" => format_colors(&res.value),
                    _ => {
                        let sep = &display_cfg.separator;
                        if res.key.is_empty() && res.value.is_empty() {
                            String::new()
                        } else if res.key.is_empty() {
                            format!("  {}", res.value)
                        } else {
                            let formatted_key = if no_color {
                                res.key.clone()
                            } else {
                                format!("\x1b[36m{}\x1b[0m", res.key)
                            };
                            format!("  {formatted_key}{sep}{}", res.value)
                        }
                    }
                };
                result_lines.push(line);
            }
        }
    }

    let max_lines = logo_height.max(result_lines.len());

    for i in 0..max_lines {
        let logo_line = logo_lines.get(i).map(|s| s.as_str()).unwrap_or("");
        let result_line = result_lines.get(i).map(|s| s.as_str()).unwrap_or("");

        let display_logo_line = if no_color {
            strip_ansi(logo_line)
        } else {
            logo_line.to_string()
        };

        if !display_logo_line.is_empty() {
            let logo_padded = pad_to_visible(&display_logo_line, logo_width);
            let padding = "  ";
            println!("{logo_padded}{padding}{result_line}");
        } else if !result_line.is_empty() || (i < result_lines.len() && result_lines[i].is_empty()) {
            let logo_pad = " ".repeat(logo_width + 2);
            println!("{logo_pad}{result_line}");
        } else {
            println!();
        }
    }
}

fn format_colors(value: &str) -> String {
    let colors = ["\x1b[31m", "\x1b[33m", "\x1b[32m", "\x1b[36m", "\x1b[34m", "\x1b[35m"];
    let reset = "\x1b[0m";
    let mut result = String::new();
    for (i, ch) in value.chars().enumerate() {
        result.push_str(colors[i % colors.len()]);
        result.push(ch);
    }
    result.push_str(reset);
    result
}
