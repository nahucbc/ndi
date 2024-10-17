use crate::host::get_target;
use std::fs::File;
use std::io::copy;

pub async fn download(version : &String) -> Result<(), Box<dyn std::error::Error>> {
    let (platform, kind, architecture) = get_target();
    let file_name : String;
    let url : String;

    if architecture == "arm" {
        (file_name, url) = arm_target(platform, kind, version);

    } else if architecture == "aarch64" {
        (file_name, url) = aarch64_target(platform, kind, architecture, version);

    } else if platform == "Solaris" {
        (file_name, url) = solaris_target(platform, version);

    } else {
        (file_name, url) = x86_64_target(platform, kind, architecture, version);

    }
    
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let content = response.bytes().await?;
        let mut file = File::create(file_name)?;

        copy(&mut content.as_ref(), &mut file)?;

    } else {
        println!("Error downloading file: {}", response.status());
    }

    Ok(())
    
}

fn aarch64_target(platform : &'static str, kind : &'static str, architecture : &'static str, version : &String)  -> (String, String) {
    let file_name : String = format!("NVIDIA-{}-{}-{}.run", kind, architecture, version);
    let url : String = format!("https://us.download.nvidia.com/{}/{}/{}/{}", platform, architecture, version, file_name);
    return (file_name, url).into();
}

fn arm_target(platform : &'static str, kind : &'static str, version : &String) -> (String, String) {
    let file_name : String = format!("NVIDIA-{}-armv7l-gnueabihf-{}.run", kind, version);
    let url : String = format!("https://us.download.nvidia.com/{}/{}-x86-ARM/{}/{}", platform, kind, version, file_name);
    return (file_name, url).into();
}

fn solaris_target(platform : &'static str, version : &String) -> (String, String) {
    let file_name : String = format!("NVIDIA-{}-x86-{}.run", platform, version);
    let url : String = format!("https://us.download.nvidia.com/solaris/{}/{}", version, file_name);
    return (file_name, url).into();
}

fn x86_64_target(platform : &'static str, kind : &'static str, architecture : &'static str, version : &String)  -> (String, String) {
    let file_name : String = format!("NVIDIA-{}-{}-{}.run", kind, architecture, version);
    let url : String = format!("https://us.download.nvidia.com/{}/{}-{}/{}/{}", platform, kind, architecture, version, file_name);
    return (file_name, url).into();
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn aarch64_target_test() {
        let platform : &'static str = "XFree86";
        let kind : &'static str = "Linux";
        let architecture: &'static str = "aarch64";
        let version : &String = &String::from("550.120");
        
        let (_file_name, url) = crate::fetch::aarch64_target(platform, kind, architecture, version);

        let response = reqwest::get(url).await.expect("request failed");

        assert!(response.status().is_success());
    }

    #[tokio::test]
    async fn arm_target_test() {
        let platform : &'static str = "XFree86";
        let kind : &'static str = "Linux";
        let version : &String = &String::from("390.157");
        
        let (_file_name, url) = crate::fetch::arm_target(platform, kind, version);

        let response = reqwest::get(url).await.expect("request failed");

        assert!(response.status().is_success());
    }

    #[tokio::test]
    async fn solaris_target_test() {
        let platform : &'static str = "Solaris";
        let version = &String::from("550.120");
        
        let (_file_name, url) = crate::fetch::solaris_target(platform, version);

        let response = reqwest::get(url).await.expect("request failed");

        assert!(response.status().is_success());
    }

    #[tokio::test]
    async fn x86_64_target_test() {
        let platform : &'static str = "XFree86";
        let kind : &'static str = "Linux";
        let architecture: &'static str = "x86_64";
        let version : &String = &String::from("550.120");
        
        let (_file_name, url) = crate::fetch::x86_64_target(platform, kind, architecture, version);

        let response = reqwest::get(url).await.expect("request failed");

        assert!(response.status().is_success());
    }
    
}