use std::path::PathBuf;

const CHARSETS: &[(&str, &[u8])] = &[
    ("standard", b"@%#*+=-:. "),
    ("detailed", b"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. "),
    ("block", b"\xe2\x96\x88\xe2\x96\x93\xe2\x96\x92\xe2\x96\x91 "),
    ("simple", b"#0. "),
    ("binary", b"# "),
];

fn find_charset(name: &str) -> Option<&'static [u8]> {
    CHARSETS.iter().find(|(n, _)| *n == name).map(|(_, c)| *c)
}

fn luminance(r: u8, g: u8, b: u8) -> f64 {
    0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64
}

fn map_char(lum: f64, chars: &[u8], invert: bool) -> char {
    let idx = if invert {
        (255.0 - lum) * (chars.len() as f64 - 1.0) / 255.0
    } else {
        lum * (chars.len() as f64 - 1.0) / 255.0
    };
    let idx = idx.round().max(0.0).min((chars.len() - 1) as f64) as usize;
    chars[idx] as char
}

pub struct AsciiOpts {
    pub width: u32,
    pub color: bool,
    pub invert: bool,
    pub charset: String,
    pub contrast: f64,
    pub dither: bool,
    pub save: Option<PathBuf>,
}

pub fn run(path: &str, opts: &AsciiOpts) -> Result<(), String> {
    let img = image::open(path).map_err(|e| format!("failed to open image: {e}"))?;

    let chars = find_charset(&opts.charset)
        .ok_or_else(|| format!("unknown charset '{}'", opts.charset))?;

    let (orig_w, orig_h) = (img.width(), img.height());
    let aspect = orig_h as f64 / orig_w as f64;

    let font_aspect = 0.43;
    let ascii_h = (opts.width as f64 * aspect * font_aspect).round() as u32;
    let ascii_w = opts.width;

    let resized = img.resize_exact(ascii_w, ascii_h, image::imageops::FilterType::Lanczos3);
    let rgb = resized.to_rgb8();

    if opts.dither && !opts.color {
        dither_fs(&rgb, ascii_w, ascii_h, opts.contrast, opts.invert, chars, opts.save.as_ref())
    } else {
        direct_render(&rgb, ascii_w, ascii_h, opts.contrast, opts.invert, opts.color, chars, opts.save.as_ref())
    }
}

fn apply_contrast(lum: f64, contrast: f64) -> f64 {
    if contrast == 1.0 { return lum; }
    let normalized = lum / 255.0;
    let adjusted = (normalized - 0.5) * contrast + 0.5;
    adjusted.max(0.0).min(1.0) * 255.0
}

fn direct_render(
    rgb: &image::RgbImage, w: u32, h: u32,
    contrast: f64, invert: bool, color: bool,
    chars: &[u8], save: Option<&PathBuf>,
) -> Result<(), String> {
    let mut lines: Vec<String> = Vec::new();

    for y in 0..h {
        let mut line = String::new();
        for x in 0..w {
            let pixel = rgb.get_pixel(x, y);
            let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
            let lum = apply_contrast(luminance(r, g, b), contrast);
            let ch = map_char(lum, chars, invert);

            if color {
                line.push_str(&format!("\x1b[38;2;{r};{g};{b}m{ch}\x1b[0m"));
            } else {
                line.push(ch);
            }
        }
        lines.push(line);
    }

    let output = lines.join("\n");
    if let Some(p) = save {
        std::fs::write(p, &output).map_err(|e| format!("failed to write file: {e}"))?;
        println!("ASCII art saved to: {}", p.display());
    } else {
        println!("{output}");
    }
    Ok(())
}

fn dither_fs(
    rgb: &image::RgbImage, w: u32, h: u32,
    contrast: f64, invert: bool,
    chars: &[u8], save: Option<&PathBuf>,
) -> Result<(), String> {
    let levels = chars.len() as f64;
    let uw = w as usize;
    let uh = h as usize;
    let mut px: Vec<f64> = Vec::with_capacity(uw * uh);

    for y in 0..uh {
        for x in 0..uw {
            let p = rgb.get_pixel(x as u32, y as u32);
            px.push(apply_contrast(luminance(p[0], p[1], p[2]), contrast));
        }
    }

    for y in 0..uh {
        for x in 0..uw {
            let i = y * uw + x;
            let old_lum = px[i];
            let quantized = if invert {
                (levels - 1.0) - ((old_lum / 255.0) * (levels - 1.0)).round()
            } else {
                ((old_lum / 255.0) * (levels - 1.0)).round()
            };
            let quantized = quantized.max(0.0).min(levels - 1.0);
            let new_lum = quantized * (255.0 / (levels - 1.0));
            let err = old_lum - new_lum;
            px[i] = new_lum;

            if x + 1 < uw { px[i + 1] += err * 7.0 / 16.0; }
            if y + 1 < uh {
                let next = (y + 1) * uw;
                if x > 0 { px[next + x - 1] += err * 3.0 / 16.0; }
                px[next + x] += err * 5.0 / 16.0;
                if x + 1 < uw { px[next + x + 1] += err * 1.0 / 16.0; }
            }
        }
    }

    let mut lines: Vec<String> = Vec::new();
    for y in 0..uh {
        let mut line = String::new();
        for x in 0..uw {
            let lum = px[y * uw + x].max(0.0).min(255.0);
            let idx = if invert {
                ((255.0 - lum) / 255.0 * (levels - 1.0)).round() as usize
            } else {
                (lum / 255.0 * (levels - 1.0)).round() as usize
            };
            let idx = idx.min(chars.len() - 1);
            line.push(chars[idx] as char);
        }
        lines.push(line);
    }

    let output = lines.join("\n");
    if let Some(p) = save {
        std::fs::write(p, &output).map_err(|e| format!("failed to write file: {e}"))?;
        println!("ASCII art saved to: {}", p.display());
    } else {
        println!("{output}");
    }
    Ok(())
}
