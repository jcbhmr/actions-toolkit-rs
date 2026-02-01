use std::error::Error;

pub const ARCH: &str = std::env::consts::ARCH;

pub const IS_LINUX: bool = cfg!(target_os = "linux");

pub const IS_MAC_OS: bool = cfg!(target_os = "macos");

pub const IS_WINDOWS: bool = cfg!(windows);

pub const PLATFORM: &str = std::env::consts::OS;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Details {
    pub name: String,
    pub platform: String,
    pub arch: String,
    pub version: String,
    pub is_windows: bool,
    pub is_mac_os: bool,
    pub is_linux: bool,
}

pub fn get_details() -> Result<Details, Box<dyn Error>> {
    todo!()
}
