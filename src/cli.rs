use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "sysfetch", version, about = "Fast system info fetcher with pride flag color themes")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short = 'c', long, help = "Run interactive configuration wizard")]
    pub configure: bool,

    #[arg(short = 'f', long, value_name = "FLAG", help = "Apply a pride flag color preset")]
    pub flag: Option<String>,

    #[arg(short = 'l', long, help = "List available flag presets")]
    pub list_flags: bool,

    #[arg(short = 'L', long, value_name = "MODE", default_value = "auto", help = "Logo mode: auto, ascii, none")]
    pub logo: String,

    #[arg(short = 's', long, help = "Print default module structure and exit")]
    pub structure: bool,

    #[arg(long, help = "Disable colored output")]
    pub no_color: bool,

    #[arg(long, value_name = "PATH", help = "Use custom config file")]
    pub config: Option<String>,

    #[arg(long, help = "Export current config to stdout")]
    pub theme_export: bool,

    #[arg(long, value_name = "URL", help = "Download and apply a remote theme")]
    pub theme_apply: Option<String>,

    #[arg(short = 'j', long, help = "Output as JSON")]
    pub json: bool,

    #[arg(short = 'w', long, value_name = "SECONDS", help = "Watch mode: refresh every N seconds")]
    pub watch: Option<u64>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Convert an image to ASCII art")]
    Ascii {
        #[arg(help = "Path to the image file")]
        path: String,

        #[arg(long, default_value = "80", help = "Output width in characters")]
        width: u32,

        #[arg(long, help = "Use ANSI truecolor output")]
        color: bool,

        #[arg(long, help = "Invert brightness mapping")]
        invert: bool,

        #[arg(long, value_name = "PATH", help = "Save to file instead of stdout")]
        save: Option<PathBuf>,
    },
}
