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
    lines: "                             ....\n\
              $2.',:clooo:  $1.:looooo:.\n\
           $2.;looooooooc  $1.oooooooooo'\n\
        $2.;looooool:,''.  $1:ooooooooooc\n\
       $2;looool;.         $1'oooooooooo,\n\
      $2;clool'             $1.cooooooc.  $2,,\n\
         $2...                $1......  $2.:oo,\n\
  $1.;clol:,.                        $2.loooo'\n\
 $1:ooooooooo,                        $2'ooool\n\
$1'ooooooooooo.                        $2loooo.\n\
$1'ooooooooool                         $2coooo.\n\
 $1,loooooooc.                        $2.loooo.\n\
   $1.,;;;'.                          $2;ooooc\n\
       $2...                         $2,ooool.\n\
    $2.cooooc.              $1..',,'.  $2.cooo.\n\
      $2;ooooo:.           $1;oooooooc.  $2:l.\n\
       $2.coooooc,..      $1coooooooooo.\n\
         $2.:ooooooolc:. $1.ooooooooooo'\n\
           $2.':loooooo;  $1,oooooooooc\n\
               $2..';::c'  $1.;loooo:'",
    colors: &["31", "31"],
};

pub const ARCH: LogoEntry = LogoEntry {
    names: &["arch", "archmerge", "archlinux"],
    lines: "                  -`\n\
                 .o+`\n\
                `ooo/\n\
               `+oooo:\n\
              `+oooooo:\n\
              -+oooooo+:\n\
            `/:-:++oooo+:\n\
           `/++++/+++++++:\n\
          `/++++++++++++++:\n\
         `/+++o$2oooooooo$1oooo/`\n\
        ./$2ooosssso++osssssso$1+`\n\
$2       .oossssso-````/ossssss+`\n\
      -osssssso.      :ssssssso.\n\
     :osssssss/        osssso+++.\n\
    /ossssssss/        +ssssooo/-\n\
  `/ossssso+/:-        -:/+osssso+-\n\
 `+sso+:-`                 `.-/+oso:\n\
`++:.                           `-/+/\n\
.`                                 `/",
    colors: &["36", "36"],
};

pub const FEDORA: LogoEntry = LogoEntry {
    names: &["fedora"],
    lines: "             .',;::::;,'.\n\
         .';:cccccccccccc:;,.\n\
      .;cccccccccccccccccccccc;.\n\
    .:cccccccccccccccccccccccccc:.\n\
  .;ccccccccccccc;$2.:dddl:.$1;ccccccc;.\n\
 .:ccccccccccccc;$2OWMKOOXMWd$1;ccccccc:.\n\
.:ccccccccccccc;$2KMMc$1;cc;$2xMMc$1;ccccccc:.\n\
,cccccccccccccc;$2MMM.$1;cc;$2;WW:$1;cccccccc,\n\
:cccccccccccccc;$2MMM.$1;cccccccccccccccc:\n\
:ccccccc;$2oxOOOo$1;$2MMM000k.$1;cccccccccccc:\n\
cccccc;$20MMKxdd:$1;$2MMMkddc.$1;cccccccccccc;\n\
ccccc;$2XMO'$1;cccc;$2MMM.$1;cccccccccccccccc'\n\
ccccc;$2MMo$1;ccccc;$2MMW.$1;ccccccccccccccc;\n\
ccccc;$20MNc.$1ccc$2.xMMd$1;ccccccccccccccc;\n\
cccccc;$2dNMWXXXWM0:$1;cccccccccccccc:,\n\
cccccccc;$2.:odl:.$1;cccccccccccccc:,.\n\
ccccccccccccccccccccccccccccc:'.\n\
:ccccccccccccccccccccccc:;,..\n\
 ':cccccccccccccccc::;,.",
    colors: &["34", "37"],
};

pub const DEBIAN: LogoEntry = LogoEntry {
    names: &["debian"],
    lines: "        $2_,met$$$$$$$$$$gg.\n\
     ,g$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$P.\n\
   ,g$$$$P\"\"       \"\"\"Y$$$$.\".\n\
  ,$$$$P'              `$$$$$$.\n\
',$$$$P       ,ggs.     `$$$$b:\n\
`d$$$$'     ,$P\"'   $1.$2    $$$$$$\n\
 $$$$P      d$'     $1,$2    $$$$P\n\
 $$$$:      $$$.   $1-$2    ,d$$$$'\n\
 $$$$;      Y$b._   _,d$P'\n\
 Y$$$$.    $1`.$2`\"Y$$$$$$$$P\"'\"\n\
 `$$$$b      $1\"-.__\n\
  $2`Y$$$$b\n\
   `Y$$$$.\n\
     `$$$$b.\n\
       `Y$$$$b.\n\
         `\"Y$$b._\n\
             `\"\"\"\"",
    colors: &["31", "37"],
};

pub const NIXOS: LogoEntry = LogoEntry {
    names: &["nixos"],
    lines: "          $1▗▄▄▄       $2▗▄▄▄▄    ▄▄▄▖\n\
          $1▜███▙       $2▜███▙  ▟███▛\n\
           $1▜███▙       $2▜███▙▟███▛\n\
            $1▜███▙       $2▜██████▛\n\
     $1▟█████████████████▙ $2▜████▛     $3▟▙\n\
    $1▟███████████████████▙ $2▜███▙    $3▟██▙\n\
           $6▄▄▄▄▖           $2▜███▙  $3▟███▛\n\
          $6▟███▛             $2▜██▛ $3▟███▛\n\
         $6▟███▛               $2▜▛ $3▟███▛\n\
$6▟███████████▛                  $3▟██████████▙\n\
$6▜██████████▛                  $3▟███████████▛\n\
      $6▟███▛ $5▟▙               $3▟███▛\n\
     $6▟███▛ $5▟██▙             $3▟███▛\n\
    $6▟███▛  $5▜███▙           $3▝▀▀▀▀\n\
    $6▜██▛    $5▜███▙ $4▜██████████████████▛\n\
     $6▜▛     $5▟████▙ $4▜████████████████▛\n\
           $5▟██████▙         $4▜███▙\n\
          $5▟███▛▜███▙         $4▜███▙\n\
         $5▟███▛  ▜███▙         $4▜███▙\n\
         $5▝▀▀▀    ▀▀▀▀▘         $4▀▀▀▘",
    colors: &["36", "37", "34", "35", "91", "93"],
};

pub const MANJARO: LogoEntry = LogoEntry {
    names: &["manjaro"],
    lines: "██████████████████  ████████\n\
██████████████████  ████████\n\
██████████████████  ████████\n\
██████████████████  ████████\n\
████████            ████████\n\
████████  ████████  ████████\n\
████████  ████████  ████████\n\
████████  ████████  ████████\n\
████████  ████████  ████████\n\
████████  ████████  ████████\n\
████████  ████████  ████████\n\
████████  ████████  ████████\n\
████████  ████████  ████████\n\
████████  ████████  ████████",
    colors: &["32"],
};

pub const VOID: LogoEntry = LogoEntry {
    names: &["void"],
    lines: "                __.;=====;.__\n\
            _.=+==++=++=+=+===;.\n\
             -=+++=+===+=+=+++++=_\n\
        .     -=:``     `--==+=++==.\n\
       _vi,    `            --+=++++:\n\
      .uvnvi.       _._       -==+==+.\n\
     .vvnvnI`    .;==|==;.     :|=||=|.\n\
$2+QmQQm$1pvvnv;$2 _yYsyQQWUUQQQm #QmQ#$1:$2QQQWUV$QQm.\n\
 $2-QQWQW$1pvvo$2wZ?.wQQQE$1==<$2QWWQ/QWQW.QQWW$1(:$2 jQWQE\n\
  $2-$QQQQmmU'  jQQQ$1@+=<$2QWQQ)mQQQ.mQQQC$1+;$2jWQQ@'\n\
   $2-$WQ8Y$1nI:$2   QWQQwgQQWV$1`$2mWQQ.jQWQQgyyWW@!\n\
     $1-1vvnvv.     `~+++`        ++|+++\n\
      +vnvnnv,                 `-|===\n\
       +vnvnvns.           .      :=-\n\
        -Invnvvnsi..___..=sv=.     `\n\
          +Invnvnvnnnnnnnnvvnn;.\n\
            ~|Invnvnvvnvvvnnv}+`\n\
               -~|{*l}*|~",
    colors: &["32", "37"],
};

pub const GENTOO: LogoEntry = LogoEntry {
    names: &["gentoo"],
    lines: "         -/oyddmdhs+:.\n\
     -o$2dNMMMMMMMMNNmhy+$1-`\n\
   -y$2NMMMMMMMMMMMNNNmmdhy$1+-\n\
 `o$2mMMMMMMMMMMMMNmdmmmmddhhy$1/`\n\
 om$2MMMMMMMMMMMN$1hhyyyo$2hmdddhhhd$1o`\n\
.y$2dMMMMMMMMMMd$1hs++so/s$2mdddhhhhdm$1+`\n\
 oy$2hdmNMMMMMMMN$1dyooy$2dmddddhhhhyhN$1d.\n\
  :o$2yhhdNNMMMMMMMNNNmmdddhhhhhyym$1Mh\n\
    .:$2+sydNMMMMMNNNmmmdddhhhhhhmM$1my\n\
       /m$2MMMMMMNNNmmmdddhhhhhmMNh$1s:\n\
    `o$2NMMMMMMMNNNmmmddddhhdmMNhs$1+`\n\
  `s$2NMMMMMMMMNNNmmmdddddmNMmhs$1/.\n\
 /N$2MMMMMMMMNNNNmmmddddmNMNdso$1:`\n\
+M$2MMMMMMNNNNNmmmmdmNMNdso$1/-\n\
yM$2MNNNNNNNmmmmmNNMmhs+/$1-`\n\
/h$2MMNNNNNNNNMNdhs++/$1-`\n\
`/$2ohdmmddhys+++/:$1.`\n\
  `-//////:--.",
    colors: &["35", "37"],
};

pub const ALPINE: LogoEntry = LogoEntry {
    names: &["alpine"],
    lines: "       .hddddddddddddddddddddddh.\n\
      :dddddddddddddddddddddddddd:\n\
     /dddddddddddddddddddddddddddd/\n\
    +dddddddddddddddddddddddddddddd+\n\
  `sdddddddddddddddddddddddddddddddds`\n\
 `ydddddddddddd++hdddddddddddddddddddy`\n\
.hddddddddddd+`  `+ddddh:-sdddddddddddh.\n\
hdddddddddd+`      `+y:    .sddddddddddh\n\
ddddddddh+`   `//`   `.`     -sddddddddd\n\
ddddddh+`   `/hddh/`   `:s-    -sddddddd\n\
ddddh+`   `/+/dddddh/`   `+s-    -sddddd\n\
ddd+`   `/o` :dddddddh/`   `oy-    .yddd\n\
hdddyo+ohddyosdddddddddho+oydddy++ohdddh\n\
.hddddddddddddddddddddddddddddddddddddh.\n\
 `yddddddddddddddddddddddddddddddddddy`\n\
  `sdddddddddddddddddddddddddddddddds`\n\
    +dddddddddddddddddddddddddddddd+\n\
     /dddddddddddddddddddddddddddd/\n\
      :dddddddddddddddddddddddddd:\n\
       .hddddddddddddddddddddddh.",
    colors: &["34"],
};

pub const POPOS: LogoEntry = LogoEntry {
    names: &["pop", "pop_os", "popos"],
    lines: "             /////////////\n\
         /////////////////////\n\
      ///////$2*767$1////////////////\n\
    //////$27676767676*$1//////////////\n\
   /////$276767$1//$27676767$1//////////////\n\
  /////$2767676$1///$2*76767$1///////////////\n\
 ///////$2767676$1///$276767$1.///$27676*$1///////\n\
/////////$2767676$1//$276767$1///$2767676$1////////\n\
//////////$276767676767$1////$276767$1/////////\n\
///////////$276767676$1//////$27676$1//////////\n\
////////////,$27676$1,///////$2767$1///////////\n\
/////////////*$27676$1///////$276$1////////////\n\
///////////////$27676$1////////////////////\n\
 ///////////////$27676$1///$2767$1////////////\n\
  //////////////////////$2'$1////////////\n\
   //////$2.7676767676767676767,$1//////\n\
    /////$2767676767676767676767$1/////\n\
      ///////////////////////////\n\
         /////////////////////\n\
             /////////////",
    colors: &["37", "34"],
};

pub const LINUXMINT: LogoEntry = LogoEntry {
    names: &["linuxmint", "mint", "linux mint"],
    lines: "            $2_.-ppOOOOOOqq-._\n\
         .oOOOOPPPPPPPPPPOOOOo.\n\
      .oOOOO$1.=oOOOOOOOOOOo=.$2OOOOo.\n\
    .:OOO$1.=oOOOOOOOOOOOOOOOOo=.$2OOO:.\n\
   .OOO$1.OOOOOOOOOOOOOOOOOOOOOOOO.$2OOO.\n\
  .OOO$1.OO    OOO:´   `::´    `:OOO.$2OO:\n\
 .OOO$1.OOO    OO                OOO.$2OOO:\n\
 OOO$1.OOOO    OO    oo    oo    OOOO.$2OOO\n\
:OOO$1:OOOO    OO    OO    OO    OOOO:$2OOO:\n\
:OOO$1:OOOO    OO    OO    OO    OOOO:$2OOO:\n\
'OOO$1'OOOO    OO    OO    OO    OOOO'$2OOO'\n\
 OOO$1'OOOO    OO____OO____OO    OOOO'$2OOO'\n\
 'OOO$1'OOO    'OOOOOOOOOOOO'    OOOO'$2OOO\n\
  'OOO$1'OOO                    .OOO'$2OOO'\n\
   'OOO$1'OOOO:ooooooooooooooo:OOOO'$2OOO'\n\
    ':OOOo$1'=OOOOOOOOOOOOOOOOO='$2oOOO:'\n\
      ':OOOOo$1'=OOOOOOOOOOO='$2oOOOO:'\n\
         ``-OOOOooooooooooOOOO-´´\n\
             ```-=:OOOO:=-´´´",
    colors: &["32", "37"],
};

pub const OPENSUSE: LogoEntry = LogoEntry {
    names: &["opensuse", "suse", "opensuse-leap", "opensuse-tumbleweed"],
    lines: "           $2.;ldkO0000Okdl;.\n\
       .;d00xl:^''''''''^:ok00d;.\n\
     .d00l'                'o00d.\n\
   .d0Kd'$1  Okxol:;,.          $2:O0d\n\
  .OK$1KKK0kOKKKKKKKKKKOxo:,      $2lKO.\n\
 ,0K$1KKKKKKKKKKKKKKK0P^$2,,,$1^dx:$2    ;00,\n\
.OK$1KKKKKKKKKKKKKKKk'$2.oOPPb.$1'0k.$2   cKO.\n\
:KK$1KKKKKKKKKKKKKKK: $2kKx..dd $1lKd$2   'OK:\n\
dKK$1KKKKKKKKKOx0KKKd $2^0KKKO' $1kKKc$2   dKd\n\
dKK$1KKKKKKKKKK;.;oOKx,..$2^$1..;kKKK0.$2  dKd\n\
:KK$1KKKKKKKKKK0o;...^cdxxOK0O/^^'  $2.0K:\n\
 kKK$1KKKKKKKKKKKKK0x;,,......,;od  $2lKk\n\
 '0K$1KKKKKKKKKKKKKKKKKKKK00KKOo^  $2c00'\n\
  'kK$1KKOxddxkOO00000Okxoc;''   $2.dKk'\n\
    l0Ko.                    .c00l'\n\
     'l0Kk:.              .;xK0l'\n\
        'lkK0xl:;,,,,;:ldO0kl'\n\
            '^:ldxkkkkxdl:^'",
    colors: &["32", "37"],
};

pub const CENTOS: LogoEntry = LogoEntry {
    names: &["centos", "rhel"],
    lines: "                 ..\n\
               .PLTJ.\n\
              <><><><>\n\
     $2KKSSV' 4KKK $1LJ$4 KKKL.'VSSKK\n\
     $2KKV' 4KKKKK $1LJ$4 KKKKAL 'VKK\n\
     $2V' ' 'VKKKK $1LJ$4 KKKKV' ' 'V\n\
     $2.4MA.' 'VKK $1LJ$4 KKV' '.4Mb.\n\
   $4. $2KKKKKA.' 'V $1LJ$4 V' '.4KKKKK $3.\n\
 $4.4D $2KKKKKKKA.'' $1LJ$4 ''.4KKKKKKK $3FA.\n\
$4<QDD ++++++++++++  $3++++++++++++ GFD>\n\
 '$4VD $3KKKKKKKK'.. $2LJ $1..'KKKKKKKK $3FV\n\
   $4' $3VKKKKK'. .4 $2LJ $1K. .'KKKKKV $3'\n\
      $3'VK'. .4KK $2LJ $1KKA. .'KV'\n\
     $3A. . .4KKKK $2LJ $1KKKKA. . .4\n\
     $3KKA. 'KKKKK $2LJ $1KKKKK' .4KK\n\
     $3KKSSA. VKKK $2LJ $1KKKV .4SSKK\n\
              $2<><><><>\n\
               $2'MKKM'\n\
                 $2''",
    colors: &["33", "37", "32"],
};

pub const SLACKWARE: LogoEntry = LogoEntry {
    names: &["slackware"],
    lines: "                  ::::::::\n\
            :::::::::::::::::::\n\
         ::::::::::::::::::::::::\n\
       ::::::::$2cllcccccllllllll$1::::::\n\
    :::::::::$2lc               dc$1:::::::\n\
   ::::::::$2cl   clllccllll    oc$1:::::::::\n\
  :::::::::$2o   lc$1::::::::$2co   oc$1::::::::::\n\
 :::::::::$2o    cccclc$1:::::$2clcc$1::::::::::::\n\
 :::::::::::$2lc        cclccclc$1:::::::::::::\n\
::::::::::::::$2lcclcc          lc$1:::::::::::\n\
::::::::::$2cclcc$1:::::$2lccclc     oc$1:::::::::::\n\
::::::::::$2o    l$1::::::::::$2l    lc$1:::::::::::\n\
 :::::$2cll$1:$2o     clcllcccll     o$1:::::::::::\n\
 :::::$2occ$1:$2o                  clc$1:::::::::::\n\
  ::::$2ocl$1:$2ccslclccclclccclclc$1:::::::::::::\n\
   :::$2oclcccccccccccccllllllllllllll$1:::::\n\
    ::$2lcc1lcccccccccccccccccccccccco$1::::\n\
      ::::::::::::::::::::::::::::::::\n\
        ::::::::::::::::::::::::::::\n\
           ::::::::::::::::::::::\n\
                ::::::::::::\n",
    colors: &["37", "34"],
};

pub const SOLUS: LogoEntry = LogoEntry {
    names: &["solus"],
    lines: "$2            -```````````\n\
          `-+/------------.`\n\
       .---:mNo---------------.\n\
     .-----yMMMy:---------------.\n\
   `------oMMMMMm/----------------`\n\
  .------/MMMMMMMN+----------------.\n\
 .------/NMMMMMMMMm-+/--------------.\n\
`------/NMMMMMMMMMN-:mh/-------------`\n\
.-----/NMMMMMMMMMMM:-+MMd//oso/:-----.\n\
-----/NMMMMMMMMMMMM+--mMMMh::smMmyo:--\n\
----+NMMMMMMMMMMMMMo--yMMMMNo-:yMMMMd/.\n\
.--oMMMMMMMMMMMMMMMy--yMMMMMMh:-yMMMy-`\n\
`-sMMMMMMMMMMMMMMMMh--dMMMMMMMd:/Ny+y.\n\
`-/+osyhhdmmNNMMMMMm-/MMMMMMMmh+/ohm+\n\
  .------------:://+-/++++++$1oshddys:\n\
   -hhhhyyyyyyyyyyyhhhhddddhysssso-\n\
    `:ossssssyysssssssssssssssso:`\n\
      `:+ssssssssssssssssssss+-\n\
         `-/+ssssssssssso+/-`\n\
              `.-----..`",
    colors: &["37", "33", "31", "35", "34", "36"],
};

pub const ENDEAVOUROS: LogoEntry = LogoEntry {
    names: &["endeavouros", "endeavour"],
    lines: "                     $2./$1o$3.\n\
                   $2./$1sssso$3-\n\
                 $2`:$1osssssss+$3-\n\
               $2`:+$1sssssssssso$3/.\n\
             $2`-/o$1ssssssssssssso$3/.\n\
           $2`-/+$1sssssssssssssssso$3+:`\n\
         $2`-:/+$1sssssssssssssssssso$3+/.\n\
       $2`.://o$1sssssssssssssssssssso$3++-\n\
      $2.://+$1ssssssssssssssssssssssso$3++:\n\
    $2.:///o$1ssssssssssssssssssssssssso$3++:\n\
  $2`:////$1ssssssssssssssssssssssssssso$3+++.\n\
$2`-////+$1ssssssssssssssssssssssssssso$3++++-\n\
 $2`..-+$1oosssssssssssssssssssssssso$3+++++/`\n\
   $3./++++++++++++++++++++++++++++++/:.\n\
  `:::::::::::::::::::::::::------``",
    colors: &["36", "37", "34"],
};

pub const ARTIX: LogoEntry = LogoEntry {
    names: &["artix"],
    lines: "                   '\n\
                  'o'\n\
                 'ooo'\n\
                'ooxoo'\n\
               'ooxxxoo'\n\
              'oookkxxoo'\n\
             'oiioxkkxxoo'\n\
            ':;:iiiioxxxoo'\n\
               `'.;::ioxxoo'\n\
          '-.      `':;jiooo'\n\
         'oooio-..     `'i:io'\n\
        'ooooxxxxoio:,.   `'-;'\n\
       'ooooxxxxxkkxoooIi:-.  `'\n\
      'ooooxxxxxkkkkxoiiiiiji'\n\
     'ooooxxxxxkxxoiiii:'`     .i'\n\
    'ooooxxxxxoi:::'`       .;ioxo'\n\
   'ooooxooi::'`         .:iiixkxxo'\n\
  'ooooi:'`                `'';ioxxo'\n\
 'i:'`                          '':io'\n\
'`                                   `'",
    colors: &["36"],
};

pub const LUBUNTU: LogoEntry = LogoEntry {
    names: &["lubuntu"],
    lines: "           `.:/ossyyyysso/:.\n\
        `.:yyyyyyyyyyyyyyyyyy:.`\n\
      .:yyyyyyyyyyyyyyyyyyyyyyyy:.\n\
    .:yyyyyyyyyyyyyyyyyyyyyyyyyyyy:.\n\
   -yyyyyyyyyyyyyy$2+hNMMMNh+$1yyyyyyyyy-\n\
  :yy$2mNy+$1yyyyyyyy$2+Nmso++smMdhyysoo+$1yy:\n\
 -yy$2+MMMmmy$1yyyyyy$2hh$1yyyyyyyyyyyyyyyyyyy-\n\
.yyyy$2NMN$1yy$2shhs$1yyy$2+o$1yyyyyyyyyyyyyyyyyyyy.\n\
:yyyy$2oNM+$1yyyy$2+sso$1yyyyyyy$2ss$1yyyyyyyyyyyyy:\n\
:yyyyy$2+dNs$1yyyyyyy$2++$1yyyyy$2oN+$1yyyyyyyyyyyy:\n\
:yyyyy$2oMMmhysso$1yyyyyyyyyy$2mN+$1yyyyyyyyyyy:\n\
:yyyyyy$2hMm$1yyyyy$2+++$1yyyyyyy$2+MN$1yyyyyyyyyyy:\n\
.yyyyyyy$2ohmy+$1yyyyyyyyyyyyy$2NMh$1yyyyyyyyyy.\n\
 -yyyyyyyyyy$2++$1yyyyyyyyyyyy$2MMh$1yyyyyyyyy-\n\
  :yyyyyyyyyyyyyyyyyyyyy$2+mMN+$1yyyyyyyy:\n\
   -yyyyyyyyyyyyyyyyy$2+sdMMd+$1yyyyyyyy-\n\
    .:yyyyyyyyy$2hmdmmNMNdy+$1yyyyyyyy:.\n\
      .:yyyyyyy$2my$1yyyyyyyyyyyyyyy:.\n\
        `.:yyyy$2s$1yyyyyyyyyyyyy:.`\n\
           `.:/oosyyyysso/:.`",
    colors: &["36", "34"],
};

pub const KUBUNTU: LogoEntry = LogoEntry {
    names: &["kubuntu"],
    lines: "$1           `.:/ossyyyysso/:.\n\
        .:oyyyyyyyyyyyyyyyyyyo:`\n\
      -oyyyyyyyo$2dMMy$1yyyyyyysyyyyo-\n\
    -syyyyyyyyyy$2dMMy$1oyyyy$2dmMMy$1yyyys-\n\
   oyyys$2dMy$1syyyy$2dMMMMMMMMMMMMMy$1yyyyyyo\n\
 `oyyyy$2dMMMMy$1syysoooooo$2dMMMMy$1yyyyyyyyo`\n\
 oyyyyyy$2dMMMMy$1yyyyyyyyyyys$2dMMy$1sssssyyyo\n\
-yyyyyyyy$2dMy$1syyyyyyyyyyyyyys$2dMMMMMy$1syyy-\n\
oyyyysoo$2dMy$1yyyyyyyyyyyyyyyyyy$2dMMMMy$1syyyo\n\
yyys$2dMMMMMy$1yyyyyyyyyyyyyyyyyysosyyyyyyyy\n\
yyys$2dMMMMMy$1yyyyyyyyyyyyyyyyyyyyyyyyyyyyy\n\
oyyyyysos$2dy$1yyyyyyyyyyyyyyyyyy$2dMMMMy$1syyyo\n\
-yyyyyyyy$2dMy$1syyyyyyyyyyyyyys$2dMMMMMy$1syyy-\n\
 oyyyyyy$2dMMMy$1syyyyyyyyyyys$2dMMy$1oyyyoyyyo\n\
 `oyyyy$2dMMMy$1syyyoooooo$2dMMMMy$1oyyyyyyyyo\n\
   oyyysyyoyyyys$2dMMMMMMMMMMMy$1yyyyyyyo\n\
    -syyyyyyyyy$2dMMMy$1syyy$2dMMMy$1syyyys-\n\
      -oyyyyyyy$2dMMy$1yyyyyysosyyyyo-\n\
        ./oyyyyyyyyyyyyyyyyyyo/.\n\
           `.:/oosyyyysso/:.`",
    colors: &["36", "34"],
};

pub const LINUX: LogoEntry = LogoEntry {
    names: &["linux"],
    lines: "        $2#####\n\
       $2#######\n\
       $2##$1O$2#$1O$2##\n\
       $2#$3#####$2#\n\
     $2##$1##$3###$1##$2##\n\
    $2#$1##########$2##\n\
   $2#$1############$2##\n\
   $2#$1############$2###\n\
  $3##$2#$1###########$2##$3#\n\
$3######$2#$1#######$2#$3######\n\
$3#######$2#$1#####$2#$3#######\n\
  $3#####$2#######$3#####",
    colors: &["33", "37", "34"],
};

pub const FREEBSD: LogoEntry = LogoEntry {
    names: &["freebsd", "freebsd"],
    lines: "                        $2`\n\
  $1` `.....---...$2....--.```   -/\n\
  $1+o   .--`         $2/y:`      +.\n\
   $1yo`:.            $2:o      `+-\n\
    $1y/               $2-/`   -o/\n\
   $1.-                  $2::/sy+:.\n\
   $1/                     $2`--  /\n\
  $1`:                          $2:`\n\
  $1`:                          $2:`\n\
   $1/                          $2/\n\
   $1.-                        $2-.\n\
    $1--                      $2-.\n\
     $1`:`                  $2`:`.  .--             `--.\n\
          .---.....----.",
    colors: &["31", "37"],
};

pub const TAILS: LogoEntry = LogoEntry {
    names: &["tails"],
    lines: "      ``\n\
  ./yhNh\n\
syy/Nshh         `:o/\n\
N:dsNshh  █   `ohNMMd\n\
N-/+Nshh      `yMMMMd\n\
N-yhMshh       yMMMMd\n\
N-s:hshh  █    yMMMMd so//.\n\
N-oyNsyh       yMMMMd d  Mms.\n\
N:hohhhd:.     yMMMMd  syMMM+\n\
Nsyh+-..+y+-   yMMMMd   :mMM+\n\
+hy-      -ss/`yMMMM     `+d+\n\
  :sy/.     ./yNMMMMm      ``\n\
    .+ys- `:+hNMMMMMMy/`\n\
      `hNmmMMMMMMMMMMMMdo.\n\
       dMMMMMMMMMMMMMMMMMNh:\n\
       +hMMMMMMMMMMMMMMMMMmy.\n\
         -oNMMMMMMMMMMmy+.`\n\
           `:yNMMMds/.`\n\
              .//`",
    colors: &["37", "34", "35"],
};

pub const STEAMOS: LogoEntry = LogoEntry {
    names: &["steamos"],
    lines: "$1              .,,,,.\n\
        .,'onNMMMMMNNnn',.\n\
     .'oNMANKMMMMMMMMMMMNNn'.\n\
   .'ANMMMMMMMXKNNWWWPFFWNNMNn.\n\
  ;NNMMMMMMMMMMNWW'' ,.., 'WMMM,\n\
 ;NMMMMV+##+VNWWW' .+;'':+, 'WMW,\n\
,VNNWP+$2######$1+WW,  $2+:    $1:+, +MMM,\n\
'$2+#############,   +.    ,+' $1+NMMM\n\
$2  '*#########*'     '*,,*' $1.+NMMMM.\n\
$2     `'*###*'          ,.,;###$1+WNM,\n\
$2         .,;;,      .;##########$1+W\n\
$2,',.         ';  ,+##############'\n\
 '###+. :,. .,; ,###############'\n\
  '####.. `'' .,###############'\n\
    '#####+++################'\n\
      '*##################*'\n\
         ''*##########*''\n\
              ''''''",
    colors: &["37", "32"],
};

pub const RASPIAN: LogoEntry = LogoEntry {
    names: &["raspbian", "raspberry pi"],
    lines: "   $2`.::///+:/-.        --///+//-:`\n\
 `+oooooooooooo:   `+oooooooooooo:\n\
  /oooo++//ooooo:  ooooo+//+ooooo.\n\
  `+ooooooo:-:oo-  +o+::/ooooooo:\n\
   `:oooooooo+``    `.oooooooo+-\n\
     `:++ooo/.        :+ooo+/.`$1\n\
        ...`  `.----.` ``..\n\
     .::::-``:::::::::.`-:::-`\n\
    -:::-`   .:::::::-`  `-:::-\n\
   `::.  `.--.`  `` `.---.``.::`\n\
       .::::::::`  -::::::::` `\n\
 .::` .:::::::::- `::::::::::``::.\n\
-:::` ::::::::::.  ::::::::::.`:::-\n\
::::  -::::::::.   `-::::::::  ::::\n\
-::-   .-:::-.``....``.-::-.   -::-\n\
 .. ``       .::::::::.     `..`..\n\
   -:::-`   -::::::::::`  .:::::`\n\
   :::::::` -::::::::::` :::::::.\n\
   .:::::::  -::::::::. ::::::::\n\
    `-:::::`   ..--.`   ::::::.\n\
      `...`  `...--..`  `...`\n\
            .::::::::::\n\
             `.-::::-`",
    colors: &["32", "37"],
};

pub const ZORIN: LogoEntry = LogoEntry {
    names: &["zorin"],
    lines: "        `osssssssssssssssssssso`\n\
       .osssssssssssssssssssssso.\n\
      .+oooooooooooooooooooooooo+.\n\n\n\
  `::::::::::::::::::::::.         .:`\n\
 `+ssssssssssssssssss+:.`     `.:+ssso`\n\
.ossssssssssssssso/.       `-+ossssssso.\n\
ssssssssssssso/-`      `-/osssssssssssss\n\
.ossssssso/-`      .-/ossssssssssssssso.\n\
 `+sss+:.      `.:+ssssssssssssssssss+`\n\
  `:.         .::::::::::::::::::::::`\n\n\n\
      .+oooooooooooooooooooooooo+.\n\
       -osssssssssssssssssssssso-\n\
        `osssssssssssssssssssso`",
    colors: &["36", "34"],
};

pub const ELEMENTARY: LogoEntry = LogoEntry {
    names: &["elementary"],
    lines: "         eeeeeeeeeeeeeeeee\n\
      eeeeeeeeeeeeeeeeeeeeeee\n\
    eeeee  eeeeeeeeeeee   eeeee\n\
  eeee   eeeee       eee     eeee\n\
 eeee   eeee          eee     eeee\n\
eee    eee            eee       eee\n\
eee   eee            eee        eee\n\
ee    eee           eeee       eeee\n\
ee    eee         eeeee      eeeeee\n\
ee    eee       eeeee      eeeee ee\n\
eee   eeee   eeeeee      eeeee  eee\n\
eee    eeeeeeeeee     eeeeee    eee\n\
 eeeeeeeeeeeeeeeeeeeeeeee    eeeee\n\
  eeeeeeee eeeeeeeeeeee      eeee\n\
    eeeee                 eeeee\n\
      eeeeeee         eeeeeee\n\
         eeeeeeeeeeeeeeeee",
    colors: &["34", "37"],
};

pub const DEEPIN: LogoEntry = LogoEntry {
    names: &["deepin"],
    lines: "             ............\n\
         .';;;;;.       .,;,.\n\
      .,;;;;;;;.       ';;;;;;;.\n\
    .;::::::::'     .,::;;,''''',.\n\
   ,'.::::::::    .;;'.          '\n\
  ;'  'cccccc,   ,' :: '..        .:\n\
 ,,    :ccccc.  ;: .c, '' :.       ,;\n\
.l.     cllll' ., .lc  :; .l'       l.\n\
.c       :lllc  ;cl:  .l' .ll.      :'\n\
.l        'looc. .   ,o:  'oo'      c,\n\
.o.         .:ool::coc'  .ooo'      o.\n\
 ::            .....   .;dddo      ;c\n\
  l:...            .';lddddo.     ,o\n\
   lxxxxxdoolllodxxxxxxxxxc      :l\n\
    ,dxxxxxxxxxxxxxxxxxxl.     'o,\n\
      ,dkkkkkkkkkkkkko;.    .;o;\n\
        .;okkkkkdl;.    .,cl:.\n\
            .,:cccccccc:,.",
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
