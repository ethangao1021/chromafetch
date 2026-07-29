pub struct Logo {
    pub lines: Vec<String>,
    pub width: usize,
    #[allow(dead_code)]
    pub height: usize,
}

impl Logo {
    fn from_ascii(ascii: &str) -> Self {
        let lines: Vec<String> = ascii.lines().map(|l| l.to_string()).collect();
        let height = lines.len();
        let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
        Self { lines, width, height }
    }
}

fn detect_distro() -> &'static str {
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
        Some("ubuntu") | Some("debian") => "ubuntu",
        Some("arch") => "arch",
        Some("fedora") => "fedora",
        Some("nixos") => "nixos",
        Some("manjaro") => "manjaro",
        Some("void") => "void",
        Some("gentoo") => "gentoo",
        Some("opensuse") | Some("suse") => "opensuse",
        Some("alpine") => "alpine",
        Some("centos") | Some("rhel") => "centos",
        Some("pop") => "popos",
        Some("mint") => "mint",
        Some("slackware") => "slackware",
        Some("endeavouros") => "endeavour",
        Some("solus") => "solus",
        Some("freebsd") => "freebsd",
        _ => "linux",
    }
}

pub fn detect_distro_logo() -> Logo {
    let art = match detect_distro() {
        "ubuntu" => UBUNTU,
        "arch" => ARCH,
        "fedora" => FEDORA,
        "nixos" => NIXOS,
        "manjaro" => MANJARO,
        "void" => VOID,
        "gentoo" => GENTOO,
        "alpine" => ALPINE,
        "popos" => POPOS,
        "mint" => MINT,
        "endeavour" => ENDEAVOUR,
        "opensuse" => OPENSUSE,
        "centos" => CENTOS,
        "slackware" => SLACKWARE,
        "solus" => SOLUS,
        "freebsd" => FREEBSD,
        _ => LINUX_LOGO,
    };
    Logo::from_ascii(art)
}

const UBUNTU: &str = r"             .-:::::::-.
            o-:::::::::-o
           o-::::::--::-:o
          o+:::.``````.-:+o
         o+:--`        `--:+o
        o+-:``  .-::-.. ``.:+o
        o/:.` :/++++++/:.``-+o
        //-.` -/+oo+++//-`.:/-
        o/:` `:+++++++//-`.:+o
        o+:--`  `.-::-.. ``:+o
         o+:--`        `--:+o
          o+::.````````.-:+o
           o-::::::--::::-:o
            .-::::::::::-.
";

const ARCH: &str = r"             /\            /\
            /\ \          / /\ 
           /  \ \        / /  \ 
          / /\ \ \      / / /\ \
         / /  \ \ \    / / /  \ \
        / /    \ \ \  / / /    \ \
       / /      \ \ \/ / /      \ \
      / /        \ \ \/ /        \ \
     / /          \ \ \/          \ \
    / /            \ \/            \ \
   /_/              \/              \_\
";

const FEDORA: &str = r"         :/-----------\:
        :---------------:
       -------------------
      ---------------------
     :----:  :----:  :----:
    :-----:  :----:  :-----:
   :-----::  :----:  ::-----:
  :-----::   :----:   ::-----:
 :-----::    :----:    ::-----:
 :----::     :----:     ::----:
 :----::     :----:     ::----:
  :----::   :------:   ::----:
   :----:: :--------: ::----:
    :----::--------::----:
     :-----------------:
      :---------------:
       :-------------:
";

const NIXOS: &str = r"          ::::::::::::::::::
        ::::'''''''''''''''::::
      ::''                    '':: 
     :'                          ':
    :                              :
   :                                :
  :                                  :
 :                                    :
 :                                    :
  :                                  :
   :                                :
    :                              :
     :'                          ':
      ::,.                    .,::
        ::::::::::::::::::::::::::
          `:::::::::::::::::::'
";

const MANJARO: &str = r"████████████████████████████████
████████████████████████████████
████████████████████████████████
████████                       
████████                       
████████                       
████████                       
████████                       
████████                       
████████                       
████████                       
████████                       
████████                       
████████                       
████████                       
████████                       
";

const VOID: &str = r"          __   __ 
         / /  / / 
        / /  / /  
       / /  / /   
      / /  / /    
     / /  / /     
    / /__/ /      
   /______/       
  /  ____/        
 /  /__           
/_____/           
";

const GENTOO: &str = r"          _-----_
         |       |
         |  ()   | 
         |       |
          \     /
           \   /
            \ /
             V
";

const ALPINE: &str = r"       /\\ /\\
      /  \ /  \
     /    Y    \
    /     |     \
   /      |      \
  /       |       \
 /        |        \
 \        |        /
  \       |       /
   \      |      /
    \     |     /
     \    |    /
      \  / \  /
       \/   \/
";

const POPOS: &str = r"         ______
      .-'      '-.
     /     __     \
    |     /  \     |
    |     |  |     |
    |     \__/     |
     \           /
      '-.___.-'
";

const MINT: &str = r"            _____
       ____|     |____
      |                |
      |   __________   |
      |  |   __   |  |  
      |  |  |  |  |  |
      |  |  |__|  |  |
      |  |__________|  |
      |                |
       |______________|
";

const ENDEAVOUR: &str = r"          .~&&&&&&&&&&&~.
        ~&&&&&&&&&&&&&&&&~    
      ~&&&&&&&~     ~&&&&&&&~  
     &&&&&&&          &&&&&&&
    &&&&&&&            &&&&&&&
   &&&&&&&              &&&&&&&
   &&&&&&&              &&&&&&&
   &&&&&&&              &&&&&&&
    &&&&&&&            &&&&&&&
     &&&&&&&          &&&&&&&
      ~&&&&&&&~     ~&&&&&&&~
        ~&&&&&&&&&&&&&&&&~
          .~&&&&&&&&&&&~.
";

const OPENSUSE: &str = r"       .'''''''.
      /         \
     |  .'''''.  |
     | |       | |
     | |       | |
     |  '.....'  |
      \         /
       '.___.-'
";

const CENTOS: &str = r"         .::::::.
        :::::::::
       :::::::::
       ':::::::'
        '::::'
          '
";

const SLACKWARE: &str = r"          _________
        /________ /
       /________/
      /________/
     /________/
    /________/
   /________/
  /________/
";

const SOLUS: &str = r"         .''''''''''.
        /      .-''-.\
       |      /      \
       \    .'        '
        '--'          
";

const FREEBSD: &str = r"         /\\_
        /  \\
       / /\ \\
      / /  \ \\
     / /    \ \\
    / /      \ \\
   / /        \ \\
  / /          \ \\
 / /            \ \\
 \ \            / /
  \ \          / /
   \ \        / /
    \ \      / /
     \ \    / /
      \ \  / /
       \ \/ /
        \__/
";

const LINUX_LOGO: &str = r"       .---.
      /     \
     | () () |
      \  ^  /
       |||||
       |||||
";
