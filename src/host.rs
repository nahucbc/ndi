use uname::uname;

enum Platform {
    XFree86,
    Solaris
}

impl Platform {

    fn new(kind : &Kind) -> Option<Platform> {
        match kind {
            Kind::Linux => Some(Platform::XFree86),
            Kind::FreeBSD => Some(Platform::XFree86),
            Kind::Solaris => Some(Platform::Solaris),
        }
    }
    
    fn to_string(&self) -> String {
        match self {
            Platform::XFree86 => String::from("XFree86"),
            Platform::Solaris => String::from("Solaris")
        }
    }
}

#[derive(Clone, Copy)]
enum Kind {
    Linux,
    FreeBSD,
    Solaris
}

impl Kind{

    fn new(kind : &str) -> Option<Kind> {
        match kind {
            "Linux" => Some(Kind::Linux),
            "FreeBSD" => Some(Kind::FreeBSD),
            "SunOS" => Some(Kind::Solaris),
            _ => None
        }
    }
    
    fn to_string(&self) -> String {
        match self {
            Kind::FreeBSD => String::from("FreeBSD"),
            Kind::Linux => String::from("Linux"),
            Kind::Solaris => String::from("Solaris")
        }
    }
}

enum Architecture {
    X86_64,
    X86,
    Aarch64,
    Arm
}

impl Architecture {

    fn new(architecture : &str) -> Option<Architecture> {
        match architecture {
            "x86_64" => Some(Architecture::X86_64),
            "i686" => Some(Architecture::X86),
            "aarch64" => Some(Architecture::Aarch64),
            "armv7l" => Some(Architecture::Arm),
            _ => None
        }
    }
    
    fn to_string(&self) -> String {
        match self {
            Architecture::X86_64 => String::from("x86_64"),
            Architecture::X86 => String::from("x86"),
            Architecture::Aarch64 => String::from("aarch64"),
            Architecture::Arm => String::from("arm")
        }
    }
}

pub struct Target {
    platform : Platform,
    kind : Kind, 
    architecture : Architecture
}

impl Target {

    pub fn new() -> Target {
        let host = uname().unwrap();
        let kind = Kind::new(&host.sysname).unwrap();
        let platform = Platform::new(&kind).unwrap();
        let architecture = Architecture::new(&host.machine).unwrap();
        Target {platform, kind, architecture}
    }

    pub fn platform_to_string(&self) -> String {let string =  self.platform.to_string(); string}
    pub fn kind_to_string(&self) -> String {let string =  self.kind.to_string(); string}
    pub fn architecture_to_string(&self) -> String {let string =  self.architecture.to_string(); string}
}
