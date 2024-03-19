use std::error::Error;
use std::process::Command;

struct NameVersion {
    pub name: String,
    pub version: String,
}

#[cfg(target_os = "windows")]
fn get_windows_info() -> Result<NameVersion, Box<dyn Error>> {
    let version = Command::new("powershell")
        .arg("-command")
        .arg("(Get-CimInstance -ClassName Win32_OperatingSystem).Version")
        .output()?
        .stdout;
    let version = String::from_utf8(version)?;
    let name = Command::new("powershell")
        .arg("-command")
        .arg("(Get-CimInstance -ClassName Win32_OperatingSystem).Caption")
        .output()?
        .stdout;
    let name = String::from_utf8(name)?;
    Ok(NameVersion {
        name: name.trim().to_string(),
        version: version.trim().to_string(),
    })
}

#[cfg(target_os = "macos")]
fn get_macos_info() -> Result<NameVersion, Box<dyn Error>> {
    let version = Command::new("sw_vers")
        .arg("-productVersion")
        .output()?
        .stdout;
    let version = String::from_utf8(version)?;
    let name = Command::new("sw_vers").arg("-productName").output()?.stdout;
    let name = String::from_utf8(name)?;
    Ok(NameVersion {
        name: name.trim().to_string(),
        version: version.trim().to_string(),
    })
}

#[cfg(target_os = "linux")]
fn get_linux_info() -> Result<NameVersion, Box<dyn Error>> {
    let name = Command::new("lsb_release").arg("-is").output()?.stdout;
    let name = String::from_utf8(name)?;
    let version = Command::new("lsb_release").arg("-rs").output()?.stdout;
    let version = String::from_utf8(version)?;
    Ok(NameVersion {
        name: name.trim().to_string(),
        version: version.trim().to_string(),
    })
}

#[cfg(target_os = "windows")]
pub const PLATFORM: &str = "win32";
#[cfg(target_os = "macos")]
pub const PLATFORM: &str = "darwin";
#[cfg(target_os = "linux")]
pub const PLATFORM: &str = "linux";
#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
compile_error!("unsupported target_os");

#[cfg(target_arch = "x86_64")]
pub const ARCH: &str = "x86_64";
#[cfg(target_arch = "x86")]
pub const ARCH: &str = "x86";
#[cfg(target_arch = "aarch64")]
pub const ARCH: &str = "arm64";
#[cfg(target_arch = "arm")]
pub const ARCH: &str = "arm";
#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "aarch64",
    target_arch = "arm"
)))]
compile_error!("unsupported target_arch");

pub const IS_WINDOWS: bool = cfg!(target_os = "windows");
pub const IS_MACOS: bool = cfg!(target_os = "macos");
pub const IS_LINUX: bool = cfg!(target_os = "linux");

pub struct Details {
    name: String,
    platform: String,
    arch: String,
    version: String,
    is_windows: bool,
    is_macos: bool,
    is_linux: bool,
}

pub fn get_details() -> Result<Details, Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    let name_version = get_windows_info()?;
    #[cfg(target_os = "macos")]
    let name_version = get_macos_info()?;
    #[cfg(target_os = "linux")]
    let name_version = get_linux_info()?;
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    compile_error!("unsupported target_os");
    Ok(Details {
        name: name_version.name,
        platform: PLATFORM.to_string(),
        arch: ARCH.to_string(),
        version: name_version.version,
        is_windows: IS_WINDOWS,
        is_macos: IS_MACOS,
        is_linux: IS_LINUX,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_os_info() {
        #[cfg(target_os = "windows")]
        let info = get_windows_info().unwrap();
        #[cfg(target_os = "macos")]
        let info = get_macos_info().unwrap();
        #[cfg(target_os = "linux")]
        let info = get_linux_info().unwrap();
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        compile_error!("unsupported target_os");
        println!("name={:?} version={:?}", info.name, info.version);
    }
}
