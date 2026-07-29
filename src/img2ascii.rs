use std::path::PathBuf;

const CHARS: &[u8] = b"@%#*+=-:. ";

pub struct AsciiOpts {
    pub width: u32,
    pub color: bool,
    pub invert: bool,
    pub save: Option<PathBuf>,
}

pub fn run(path: &str, opts: &AsciiOpts) -> Result<(), String> {
    let img = image::open(path).map_err(|e| format!("failed to open image: {e}"))?;

    let (orig_w, orig_h) = (img.width(), img.height());
    let aspect = orig_h as f64 / orig_w as f64;
    let term_aspect = 0.45;
    let ascii_h = (opts.width as f64 * aspect * term_aspect).round() as u32;
    let ascii_w = opts.width;

    let resized = img.resize_exact(ascii_w, ascii_h, image::imageops::FilterType::Lanczos3);
    let rgb = resized.to_rgb8();

    let mut lines: Vec<String> = Vec::new();

    for y in 0..ascii_h {
        let mut line = String::new();
        for x in 0..ascii_w {
            let pixel = rgb.get_pixel(x, y);
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];

            let brightness = (r as u16 + g as u16 + b as u16) / 3;
            let idx = if opts.invert {
                (255 - brightness) * (CHARS.len() as u16 - 1) / 255
            } else {
                brightness * (CHARS.len() as u16 - 1) / 255
            };
            let ch = CHARS[idx as usize] as char;

            if opts.color {
                line.push_str(&format!("\x1b[38;2;{r};{g};{b}m{ch}\x1b[0m"));
            } else {
                line.push(ch);
            }
        }
        lines.push(line);
    }

    let output = lines.join("\n");

    if let Some(save_path) = &opts.save {
        std::fs::write(save_path, &output)
            .map_err(|e| format!("failed to write file: {e}"))?;
        println!("ASCII art saved to: {}", save_path.display());
    } else {
        println!("{output}");
    }

    Ok(())
}
