# chromafetch

A colorful system info fetcher with pride flag themes, distro logos, and image-to-ASCII conversion.

## Features

- **73+ information modules**: OS, Host, Kernel, Architecture, CPU, GPU, Memory, Disk, Network, Battery, Sensors, Display, DE/WM, Terminal, Packages, DateTime, Locale, Editor, Media, Container detection, Virtualization detection, OpenGL/Vulkan/GTK/Qt versions, and more
- **Cross-platform**: Linux (`/proc`, `/sys`, DMI), macOS (sysctl, IOKit, CoreGraphics), Windows (Win32 FFI, Registry)
- **Distro logos**: Auto-detected ASCII art logos with ANSI colors for 27 distros
- **Pride flag themes**: 10 color presets (rainbow, trans, bisexual, pansexual, etc.)
- **Image-to-ASCII**: Convert images to ASCII art with 5 charsets, Floyd-Steinberg dithering, and color output
- **Configurable**: TOML config file (`~/.config/chromafetch/config.toml`), theme export/import, watch mode, JSON output

## Usage

```
chromafetch [OPTIONS]
chromafetch ascii <PATH> [OPTIONS]
```

### Options

| Flag | Description |
|------|-------------|
| `-c`, `--configure` | Run interactive configuration wizard |
| `-f`, `--flag <FLAG>` | Apply a pride flag color preset |
| `-l`, `--list-flags` | List available flag presets |
| `-L`, `--logo <MODE>` | Logo mode: `auto`, `ascii`, `none` (default: `auto`) |
| `-s`, `--structure` | Print default module structure |
| `--no-color` | Disable colored output |
| `--config <PATH>` | Use custom config file |
| `--theme-export` | Export current config to stdout |
| `--theme-apply <URL>` | Download and apply a remote theme |
| `-j`, `--json` | Output as JSON |
| `-w`, `--watch <SECONDS>` | Watch mode: refresh every N seconds |

### ASCII Subcommand

```
chromafetch ascii <PATH> [--width 80] [--color] [--invert] [--charset standard] [--contrast 1.0] [--dither] [--save <PATH>]
```

- `--color` — ANSI truecolor output
- `--charset` — `standard`, `detailed`, `block`, `simple`, `binary`
- `--dither` — Floyd-Steinberg dithering (monochrome)
- `--contrast` — Contrast adjustment (0.0–2.0)

### Flags

`rainbow`, `trans`, `bisexual`, `pansexual`, `nonbinary`, `aromantic`, `asexual`, `lesbian`, `gay`, `progress`

## Modules (73+)

```
Title       Separator   OS          Host        Kernel      Architecture
OSBuild     Uptime      Processes   LoadAvg     Packages    Shell
Terminal    TermFont    TermSize    TermColor   CPU         CPUUsage
CPUFreq     GPU         GPUUsage    Memory      Swap        Disk
PhysDisk    DiskIO      Display     DE          WM          WMTheme
Theme       Icons       Font        Cursor      LocalIp     PublicIp
Wifi        Bluetooth   NetworkIO   Motherboard BIOS        Chassis
Sound       Monitor     Battery     BattStatus  BattCycles  PowerAdapter
Temperature Fans        Users       DateTime    Timezone    Locale
Editor      Media       Container   Virt        InitSystem  Systemd
PkgManager  PhysMem     OpenGL      Vulkan      GTK         Qt
DiskUsage   PhysDiskIO  Break       Colors
```

## Configuration

Location: `~/.config/chromafetch/config.toml` (Linux/macOS) or `%APPDATA%/chromafetch/config.toml` (Windows)

```toml
[display]
flag = "rainbow"
logo = "auto"
separator = " -> "
color_key = "cyan"
color_value = "reset"

[modules]
order = ["Title", "OS", "Host", "Kernel", ...]
disabled = []
```

## Build

```sh
cargo build --release
```

## Dependencies

- `clap` — CLI argument parsing
- `serde` / `toml` — config serialization
- `rayon` — parallelism (planned)
- `ureq` — HTTP requests for theme downloads and public IP
- `whoami` — username/hostname
- `libc` — POSIX FFI (Unix) / Win32 API bindings
- `image` — image loading for ASCII conversion

## License

MIT
