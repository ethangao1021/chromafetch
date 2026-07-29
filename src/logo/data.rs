fn sgr(code: &str) -> String {
    format!("\x1b[{}m", code)
}

pub struct LogoEntry {
    pub names: &'static [&'static str],
    pub lines: &'static str,
    pub colors: &'static [&'static str],
}

pub fn resolve(name: &str) -> Option<&'static LogoEntry> {
    ALL_LOGOS.iter().find(|l| l.names.iter().any(|n| n.eq_ignore_ascii_case(name)))
}

pub const ALL_LOGOS: &[LogoEntry] = &[
    UBUNTU, ARCH, FEDORA, DEBIAN, NIXOS, MANJARO,
    VOID, GENTOO, ALPINE, POPOS, LINUXMINT, OPENSUSE,
    CENTOS, SLACKWARE, SOLUS, ENDEAVOUROS, ARTIX, LUBUNTU,
    KUBUNTU, LINUX, FREEBSD, TAILS, STEAMOS, RASPIAN,
    ZORIN, ELEMENTARY, DEEPIN,
];

pub const UBUNTU: LogoEntry = LogoEntry {
    names: &["ubuntu", "debian"],
    lines: include_str!("../logos/ubuntu.txt"),
    colors: &["31", "31"],
};

pub const ARCH: LogoEntry = LogoEntry {
    names: &["arch", "archmerge", "archlinux"],
    lines: include_str!("../logos/arch.txt"),
    colors: &["36", "36"],
};

pub const FEDORA: LogoEntry = LogoEntry {
    names: &["fedora"],
    lines: include_str!("../logos/fedora.txt"),
    colors: &["34", "37"],
};

pub const DEBIAN: LogoEntry = LogoEntry {
    names: &["debian"],
    lines: include_str!("../logos/debian.txt"),
    colors: &["31", "37"],
};

pub const NIXOS: LogoEntry = LogoEntry {
    names: &["nixos"],
    lines: include_str!("../logos/nixos.txt"),
    colors: &["36", "37", "34", "35", "91", "93"],
};

pub const MANJARO: LogoEntry = LogoEntry {
    names: &["manjaro"],
    lines: include_str!("../logos/manjaro.txt"),
    colors: &["32"],
};

pub const VOID: LogoEntry = LogoEntry {
    names: &["void"],
    lines: include_str!("../logos/void.txt"),
    colors: &["32", "37"],
};

pub const GENTOO: LogoEntry = LogoEntry {
    names: &["gentoo"],
    lines: include_str!("../logos/gentoo.txt"),
    colors: &["35", "37"],
};

pub const ALPINE: LogoEntry = LogoEntry {
    names: &["alpine"],
    lines: include_str!("../logos/alpine.txt"),
    colors: &["34"],
};

pub const POPOS: LogoEntry = LogoEntry {
    names: &["pop", "pop_os", "popos"],
    lines: include_str!("../logos/pop.txt"),
    colors: &["37", "34"],
};

pub const LINUXMINT: LogoEntry = LogoEntry {
    names: &["linuxmint", "mint", "linux mint"],
    lines: include_str!("../logos/linuxmint.txt"),
    colors: &["32", "37"],
};

pub const OPENSUSE: LogoEntry = LogoEntry {
    names: &["opensuse", "suse", "opensuse-leap", "opensuse-tumbleweed"],
    lines: include_str!("../logos/opensuse.txt"),
    colors: &["32", "37"],
};

pub const CENTOS: LogoEntry = LogoEntry {
    names: &["centos", "rhel"],
    lines: include_str!("../logos/centos.txt"),
    colors: &["33", "37", "32"],
};

pub const SLACKWARE: LogoEntry = LogoEntry {
    names: &["slackware"],
    lines: include_str!("../logos/slackware.txt"),
    colors: &["37", "34"],
};

pub const SOLUS: LogoEntry = LogoEntry {
    names: &["solus"],
    lines: include_str!("../logos/solus.txt"),
    colors: &["37", "33", "31", "35", "34", "36"],
};

pub const ENDEAVOUROS: LogoEntry = LogoEntry {
    names: &["endeavouros", "endeavour"],
    lines: include_str!("../logos/endeavouros.txt"),
    colors: &["36", "37", "34"],
};

pub const ARTIX: LogoEntry = LogoEntry {
    names: &["artix"],
    lines: include_str!("../logos/artix.txt"),
    colors: &["36"],
};

pub const LUBUNTU: LogoEntry = LogoEntry {
    names: &["lubuntu"],
    lines: include_str!("../logos/lubuntu.txt"),
    colors: &["36", "34"],
};

pub const KUBUNTU: LogoEntry = LogoEntry {
    names: &["kubuntu"],
    lines: include_str!("../logos/kubuntu.txt"),
    colors: &["36", "34"],
};

pub const LINUX: LogoEntry = LogoEntry {
    names: &["linux"],
    lines: include_str!("../logos/linux.txt"),
    colors: &["33", "37", "34"],
};

pub const FREEBSD: LogoEntry = LogoEntry {
    names: &["freebsd", "freebsd"],
    lines: include_str!("../logos/freebsd.txt"),
    colors: &["31", "37"],
};

pub const TAILS: LogoEntry = LogoEntry {
    names: &["tails"],
    lines: include_str!("../logos/tails.txt"),
    colors: &["37", "34", "35"],
};

pub const STEAMOS: LogoEntry = LogoEntry {
    names: &["steamos"],
    lines: include_str!("../logos/steamos.txt"),
    colors: &["37", "32"],
};

pub const RASPIAN: LogoEntry = LogoEntry {
    names: &["raspbian", "raspberry pi"],
    lines: include_str!("../logos/raspbian.txt"),
    colors: &["32", "37"],
};

pub const ZORIN: LogoEntry = LogoEntry {
    names: &["zorin"],
    lines: include_str!("../logos/zorin.txt"),
    colors: &["36", "34"],
};

pub const ELEMENTARY: LogoEntry = LogoEntry {
    names: &["elementary"],
    lines: include_str!("../logos/elementary.txt"),
    colors: &["34", "37"],
};

pub const DEEPIN: LogoEntry = LogoEntry {
    names: &["deepin"],
    lines: include_str!("../logos/deepin.txt"),
    colors: &["34", "37"],
};

pub fn render_line(line: &str, colors: &[&str]) -> String {
    let mut out = String::new();
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '$' {
            match chars.next() {
                Some('$') => out.push('$'),
                Some('1') => out.push_str(&sgr(colors[0])),
                Some('2') if colors.len() > 1 => out.push_str(&sgr(colors[1])),
                Some('3') if colors.len() > 2 => out.push_str(&sgr(colors[2])),
                Some('4') if colors.len() > 3 => out.push_str(&sgr(colors[3])),
                Some('5') if colors.len() > 4 => out.push_str(&sgr(colors[4])),
                Some('6') if colors.len() > 5 => out.push_str(&sgr(colors[5])),
                Some('7') if colors.len() > 6 => out.push_str(&sgr(colors[6])),
                Some('8') if colors.len() > 7 => out.push_str(&sgr(colors[7])),
                Some(c) => {
                    out.push('$');
                    out.push(c);
                }
                None => out.push('$'),
            }
        } else {
            out.push(ch);
        }
    }
    out
}
