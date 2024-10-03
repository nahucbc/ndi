enum Platform {
    Linux,
    XFreeBSD,
    Solaris
}

enum Architecture {
    X86_64,
    X86,
    Aarch64,
    Arm
}

fn get_platform(platform : &str) -> Option<Platform> {
    match platform {
        "Linux" => Some(Platform::Linux),
        "FreeBSD" => Some(Platform::XFreeBSD),
        "SunOS" => Some(Platform::Solaris),
        _ => None
    }
}

fn get_architecture(architecture : &str) -> Option<Architecture> {
    match architecture {
        "x86_64" => Some(Architecture::X86_64),
        "i686" => Some(Architecture::X86),
        "aarch64" => Some(Architecture::Aarch64),
        "armv7l" => Some(Architecture::Arm),
        _ => None
    }
}