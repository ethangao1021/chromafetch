mod cli;
mod config;
mod detect;
mod display;
mod flags;
mod img2ascii;
mod info;
mod logo;

use clap::Parser;
use cli::Commands;
use config::Config;
use info::LogoMode;

fn main() {
    let cli = cli::Cli::parse();

    if let Some(cmd) = &cli.command {
        match cmd {
            Commands::Ascii { path, width, color, invert, save } => {
                let opts = img2ascii::AsciiOpts {
                    width: *width,
                    color: *color,
                    invert: *invert,
                    save: save.clone(),
                };
                if let Err(e) = img2ascii::run(path, &opts) {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
                return;
            }
        }
    }

    if cli.list_flags {
        for name in flags::list_flags() {
            println!("{name}");
        }
        return;
    }

    if cli.structure {
        for m in info::DEFAULT_MODULE_ORDER {
            println!("{m}");
        }
        return;
    }

    let mut config = if let Some(path) = &cli.config {
        Config::load(Some(path))
    } else {
        Config::load(None)
    };

    if let Some(flag) = &cli.flag {
        config.apply_flag(flag);
    }

    if let Some(theme) = &cli.theme_apply {
        if let Err(e) = config.apply_theme(theme) {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }

    if cli.configure {
        if let Err(e) = config.save(None) {
            eprintln!("error: {e}");
        } else {
            println!("Config saved.");
        }
        return;
    }

    if cli.theme_export {
        if let Ok(content) = toml::to_string_pretty(&config) {
            println!("{content}");
        }
        return;
    }

    let logo_mode = match cli.logo.as_str() {
        "none" => LogoMode::NoneColor,
        "ascii" => LogoMode::Ascii,
        _ => LogoMode::Auto,
    };

    let no_color = cli.no_color;

    let logo = logoselect(&logo_mode);

    let enabled: Vec<String> = config
        .modules
        .order
        .iter()
        .filter(|m| !config.modules.disabled.contains(m))
        .cloned()
        .collect();

    let info = detect::run_detection(&enabled, &config.modules.disabled);

    let display_cfg = &config.display;
    display::render::render(&info, &logo, display_cfg, &enabled, no_color);
}

fn logoselect(mode: &LogoMode) -> logo::Logo {
    match mode {
        LogoMode::Auto | LogoMode::Ascii => logo::detect_distro_logo(),
        LogoMode::NoneColor => logo::Logo { lines: vec![], width: 0, height: 0 },
    }
}
