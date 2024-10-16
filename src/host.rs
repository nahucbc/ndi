use uname::uname;


pub fn get_target() -> (&'static str, &'static str, &'static str) {
    let host = uname().expect("Expected host, found none");
    let kind : &'static str = get_kind(&host.sysname).expect("Expected a kind, found none");
    let platform : &'static str = get_platform(kind).expect("Expected platform, found none");
    let architecture : &'static str = get_architecture(&host.machine).expect("Expected architecture, found none");
    return (platform, kind, architecture).into();
}

fn get_kind(sysname : &String) -> Option<&'static str>{
    let sys : &str = &sysname;
    match sys {
        "Linux" => Some("Linux"),
        "FreeBSD" => Some("FreeBSD"),
        "SunOS" => Some("Solaris"),
         _ => None
    }
}

fn get_platform(kind : &str) -> Option<&'static str> {
    match kind {
        "Linux" | "FreeBSD" => Some("XFree86"),
        "Solaris" => Some("Solaris"),
        _ => None
    }
}

fn get_architecture(machine : &String) -> Option<&'static str>{
    let architecture : &str = &machine;
    match architecture {
        "x86_64" => Some("x86_64"),
        "i686" => Some("i686"),
        "aarch64" => Some("aarch64"),
        "armv7l" => Some("armv7l"),
         _ => None
    }
}
