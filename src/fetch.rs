use crate::host::Target;

fn download(version : &str) {
    let target = Target::new();


    if target.architecture_to_string() == "arm" {
        arm_file(target, version);

    } else if target.platform_to_string() == "Solaris" {
        solaris_file(target, version);

    } else {
        default_file(target, version);
    }
}

fn arm_file(target : Target, version : &str) {
    let platform = target.platform_to_string();
    let kind = target.kind_to_string();
    let file : String = format!("NVIDIA-{}-armv7l-gnueabihf-{}.run", kind, version);
    let url : String = format!("https://us.download.nvidia.com/{}/{}-x86-ARM/{}/{}", platform, kind, version, file);
}

fn solaris_file(target : Target, version : &str) {
    let platform = target.platform_to_string();
    let kind = target.kind_to_string();
    let file : String = format!("NVIDIA-{}-x86-${}.run", platform, version);
    let url : String = format!("https://us.download.nvidia.com/solaris/{}/{}", version, file);
}

fn default_file(target : Target, version : &str) {
    let platform = target.platform_to_string();
    let kind = target.kind_to_string();
    let architecture = target.architecture_to_string();
    let file : String = format!("NVIDIA-{}-{}-{}.run", kind, architecture, version);
    let url : String = format!("https://us.download.nvidia.com/{}/{}-{}/{}/{}", platform, kind, architecture, version, file);
}

