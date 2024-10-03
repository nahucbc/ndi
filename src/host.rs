use uname::uname;

struct Target {
    platform : Platform,
    kind : Kind, 
    architecture : Architecture
}

impl Target {
    pub fn new(platform : Platform, kind : Kind, architecture : Architecture) -> Target {
        Target {platform, kind, architecture}
    }
}

pub enum Platform {
    XFree86,
    Solaris
}

pub enum Kind {
    Linux,
    FreeBSD,
    Solaris
}

pub enum Architecture {
    X86_64,
    X86,
    Aarch64,
    Arm
}

pub fn get_target() -> Target {
    let host = uname().unwrap();
    let kind = get_kind(&host.sysname).unwrap();
    let platform = get_platform(&kind).unwrap();
    let architecture = get_architecture(&host.machine).unwrap();
    let target = Target::new(platform, kind, architecture);
    return target;
}

pub fn get_platform(kind : &Kind) -> Option<Platform> {
    match kind {
        Kind::Linux => Some(Platform::XFree86),
        Kind::FreeBSD => Some(Platform::XFree86),
        Kind::Solaris => Some(Platform::Solaris),
        _ => None
    }
}

pub fn get_kind(kind : &str) -> Option<Kind> {
    match kind {
        "Linux" => Some(Kind::Linux),
        "FreeBSD" => Some(Kind::FreeBSD),
        "SunOS" => Some(Kind::Solaris),
        _ => None
    }
}

pub fn get_architecture(architecture : &str) -> Option<Architecture> {
    match architecture {
        "x86_64" => Some(Architecture::X86_64),
        "i686" => Some(Architecture::X86),
        "aarch64" => Some(Architecture::Aarch64),
        "armv7l" => Some(Architecture::Arm),
        _ => None
    }
}