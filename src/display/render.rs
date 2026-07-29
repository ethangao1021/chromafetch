use crate::info::SystemInfo;
use crate::logo::Logo;
use crate::config::DisplaySection;

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
                            format!("  {}{}", pad_key("", display_cfg), res.value)
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

        let fixed_logo_width = if logo_width > 0 { logo_width } else { 0 };

        if !logo_line.is_empty() {
            let logo_part = format!("{logo_line:width$}", width = fixed_logo_width);
            let padding = "  ";
            println!("{logo_part}{padding}{result_line}");
        } else if !result_line.is_empty() {
            let logo_pad = " ".repeat(fixed_logo_width + 2);
            println!("{logo_pad}{result_line}");
        } else {
            println!();
        }
    }
}

fn pad_key(key: &str, _cfg: &DisplaySection) -> String {
    if key.is_empty() {
        String::new()
    } else {
        format!("{key}")
    }
}

fn format_colors(value: &str) -> String {
    let colors = ["\x1b[31m", "\x1b[33m", "\x1b[32m", "\x1b[36m", "\x1b[34m", "\x1b[35m"];
    let reset = "\x1b[0m";
    let mut result = String::new();
    for (i, ch) in value.chars().enumerate() {
        let c = colors[i % colors.len()];
        result.push_str(c);
        result.push(ch);
    }
    result.push_str(reset);
    result
}
