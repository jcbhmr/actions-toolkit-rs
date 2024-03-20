//! ✅ Get inputs, set outputs, and other basic operations for GitHub Actions
//!
//! <table align=center><td>
//!
//! ```rs
//! let name = core::get_input_with_options("name", core::GetInputOptions {
//!    required: true,
//!   ..Default::default()
//! })?;
//! let favorite_color = core::get_input("favorite-color")?;
//! core::info!("Hello {name}!");
//! let message = format!("I like {favorite_color} too!");
//! core::set_output("message", message);
//! ```
//!
//! </table>
//!
//! 👀 Looking for more GitHub Actions crates? Check out [the actions-toolkit.rs](https://github.com/jcbhmr/actions-toolkit.rs) project.
//!
//! ## Installation
//!
//! ```sh
//! cargo add actions-core2
//! ```
//!
//! ⚠️ Use `use actions_core` in your Rust code. The package name differs from the crate name.
//!
//! ## Usage
//!
//! ![Rust](https://img.shields.io/static/v1?style=for-the-badge&message=Rust&color=000000&logo=Rust&logoColor=FFFFFF&label=)
//!
//! ```rs
//! use actions_core as core;
//! use std::error::Error;
//!
//! fn main() {
//!   let result = || -> Result<(), Box<dyn Error>> {
//!     let name = core::get_input_with_options("name", core::InputOptions {
//!         required: true,
//!         ..Default::default()
//!     })?;
//!     let favorite_color = core::get_input("favorite-color")?;
//!     core::info!("Hello {name}!");
//!     core::set_output("message", "Wow! Rust is awesome!");
//!     Ok(())
//!   }();
//!   if let Err(error) = result {
//!     core::set_failed!("{error}");
//!   }
//! }
//! ```
//!
//! 🤔 But how do I actually use the generated executable in my `action.yml`? Check out [configure-executable-action](https://github.com/jcbhmr/configure-executable-action)!
//!
//! ## Development
//!
//! ![Rust](https://img.shields.io/static/v1?style=for-the-badge&message=Rust&color=000000&logo=Rust&logoColor=FFFFFF&label=)
//! ![Cargo](https://img.shields.io/static/v1?style=for-the-badge&message=Cargo&color=e6b047&logo=Rust&logoColor=000000&label=)
//! ![Docs.rs](https://img.shields.io/static/v1?style=for-the-badge&message=Docs.rs&color=000000&logo=Docs.rs&logoColor=FFFFFF&label=)
//!
//! This project is part of the [actions-toolkit.rs](https://github.com/jcbhmr/actions-toolkit.rs) project.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputOptions {
    pub required: bool,
    pub trim_whitespace: bool,
}

impl Default for InputOptions {
    fn default() -> Self {
        Self {
            required: false,
            trim_whitespace: true,
        }
    }
}

#[deprecated]
pub enum ExitCode {
    Success = 0,
    Failure = 1,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AnnotationProperties<'a> {
    pub title: &'a str,
    pub file: &'a str,
    pub start_line: u32,
    pub end_line: u32,
    pub start_column: u32,
    pub end_column: u32,
}

impl AnnotationProperties<'_> {
    fn to_string(&self) -> String {
        let mut properties = Vec::new();
        if !self.title.is_empty() {
            properties.push(format!("title={}", encode_command_property(self.title)));
        }
        if !self.file.is_empty() {
            properties.push(format!("file={}", encode_command_property(self.file)));
        }
        if self.start_line != 0 {
            properties.push(format!("line={}", self.start_line));
        }
        if self.end_line != 0 {
            properties.push(format!("endLine={}", self.end_line));
        }
        if self.start_column != 0 {
            properties.push(format!("col={}", self.start_column));
        }
        if self.end_column != 0 {
            properties.push(format!("endCol={}", self.end_column));
        }
        properties.join(",")
    }
}

fn encode_command_property(property: &str) -> String {
    property
        .replace("%", "%25")
        .replace("\r", "%0D")
        .replace("\n", "%0A")
        .replace(":", "%3A")
        .replace(",", "%2C")
}

fn encode_command_data(data: &str) -> String {
    data
        .replace("%", "%25")
        .replace("\r", "%0D")
        .replace("\n", "%0A")
}

pub fn export_variable(name: impl AsRef<str>, value: impl std::fmt::Display) {
    let name = name.as_ref();
    let value = value.to_string();
    let github_env = std::env::var("GITHUB_ENV").unwrap_or_default();
    if github_env.is_empty() {
        println!(
            "::set-env name={}::{}",
            encode_command_property(name),
            encode_command_data(&value)
        );
    } else {
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(github_env)
            .unwrap();
        let delimiter = uuid::Uuid::new_v4();
        use std::io::Write;
        writeln!(file, "{name}<<{delimiter}\n{value}\n{delimiter}").unwrap();
    }
}

pub fn set_secret(secret: impl AsRef<str>) {
    let secret = secret.as_ref();
    println!("::add-mask::{}", encode_command_data(secret));
}

pub fn add_path(input_path: impl AsRef<str>) {
    let input_path = input_path.as_ref();
    let github_path = std::env::var("GITHUB_PATH").unwrap_or_default();
    if github_path.is_empty() {
        println!("::add-path::{}", encode_command_data(input_path));
    } else {
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(github_path)
            .unwrap();
        use std::io::Write;
        writeln!(file, "{input_path}").unwrap();
    }
    let path = std::env::var("PATH").unwrap();
    const PATH_DELIMITER: &str = if cfg!(windows) { ";" } else { ":" };
    std::env::set_var("PATH", format!("{input_path}{PATH_DELIMITER}{path}"));
}

pub fn get_input(name: impl AsRef<str>) -> String {
    get_input_with_options(name, &InputOptions::default()).unwrap()
}

pub fn get_input_with_options(
    name: impl AsRef<str>,
    options: &InputOptions,
) -> Result<String, Box<dyn std::error::Error>> {
    let name = name.as_ref();
    let name_env = name.replace(" ", "_").to_uppercase();
    let value = std::env::var(format!("INPUT_{name_env}")).unwrap_or_default();
    if options.required && value.is_empty() {
        return Err(format!("{name} is required").into());
    }
    if options.trim_whitespace {
        Ok(value.trim().into())
    } else {
        Ok(value)
    }
}

pub fn get_multiline_input(name: impl AsRef<str>) -> Vec<String> {
    get_multiline_input_with_options(name, &InputOptions::default()).unwrap()
}

pub fn get_multiline_input_with_options(
    name: impl AsRef<str>,
    options: &InputOptions,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let value = get_input_with_options(name, options)?;
    let lines: Vec<String> = value
        .lines()
        .filter_map(|x| if x.is_empty() { None } else { Some(x.into()) })
        .collect();
    if options.trim_whitespace {
        Ok(lines.into_iter().map(|x| x.trim().into()).collect())
    } else {
        Ok(lines)
    }
}

pub fn get_boolean_input(name: impl AsRef<str>) -> bool {
    get_boolean_input_with_options(name, &InputOptions::default()).unwrap()
}

pub fn get_boolean_input_with_options(
    name: impl AsRef<str>,
    options: &InputOptions,
) -> Result<bool, Box<dyn std::error::Error>> {
    let name = name.as_ref();
    let value = get_input_with_options(name, options)?;
    const TRUE_VALUES: &[&str] = &["true", "True", "TRUE"];
    const FALSE_VALUES: &[&str] = &["false", "False", "FALSE"];
    if TRUE_VALUES.contains(&value.as_str()) {
        Ok(true)
    } else if FALSE_VALUES.contains(&value.as_str()) {
        Ok(false)
    } else {
        Err(format!("{name} is not a valid boolean").into())
    }
}

pub fn set_output(name: impl AsRef<str>, value: impl std::fmt::Display) {
    let name = name.as_ref();
    let value = value.to_string();
    let github_output = std::env::var("GITHUB_OUTPUT").unwrap_or_default();
    if github_output.is_empty() {
        println!("::set-output name={}::{}", encode_command_property(name), encode_command_data(&value));
    } else {
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(github_output)
            .unwrap();
        let delimiter = uuid::Uuid::new_v4();
        use std::io::Write;
        writeln!(file, "{name}<<{delimiter}\n{value}\n{delimiter}").unwrap();
    }
}

pub fn set_command_echo(enabled: bool) {
    println!("::echo::{}", if enabled { "on" } else { "off" });
}

pub fn set_failed(message: impl std::fmt::Display) -> ! {
    error(message);
    panic!();
}

pub fn is_debug() -> bool {
    std::env::var("RUNNER_DEBUG").is_ok_and(|x| x == "1")
}

pub fn debug(message: impl std::fmt::Display) {
    debug_with_properties(message, &AnnotationProperties::default());
}

pub fn debug_with_properties(
    message: impl std::fmt::Display,
    properties: &AnnotationProperties,
) {
    let message = message.to_string();
    println!("::debug {}::{}", properties.to_string(), encode_command_data(&message));
}

pub fn error(message: impl std::fmt::Display) {
    error_with_properties(message, &AnnotationProperties::default());
}

pub fn error_with_properties(
    message: impl std::fmt::Display,
    properties: &AnnotationProperties,
) {
    let message = message.to_string();
    println!("::error {}::{}", properties.to_string(), encode_command_data(&message));
}

pub fn warning(message: impl std::fmt::Display) {
    warning_with_properties(message, &AnnotationProperties::default());
}

pub fn warning_with_properties(
    message: impl std::fmt::Display,
    properties: &AnnotationProperties,
) {
    let message = message.to_string();
    println!("::warning {}::{}", properties.to_string(), encode_command_data(&message));
}

pub fn notice(message: impl std::fmt::Display) {
    notice_with_properties(message, &AnnotationProperties::default());
}

pub fn notice_with_properties(
    message: impl std::fmt::Display,
    properties: &AnnotationProperties,
) {
    let message = message.to_string();
    println!("::notice {}::{}", properties.to_string(), encode_command_data(&message));
}

pub fn info(message: impl std::fmt::Display) {
    println!("{message}");
}

pub fn start_group(name: impl AsRef<str>) {
    let name = name.as_ref();
    println!("::group::{}", encode_command_data(name));
}

pub fn end_group() {
    println!("::endgroup::");
}

pub fn group<T, F: FnOnce() -> T>(name: impl AsRef<str>, f: F) -> T {
    // `drop()` still runs even if `f()` panics.
    struct GroupResource;
    impl Drop for GroupResource {
        fn drop(&mut self) {
            end_group();
        }
    }
    start_group(name);
    let _group = GroupResource{};
    f()
}

pub fn save_state(name: impl AsRef<str>, value: impl std::fmt::Display) {
    let name = name.as_ref();
    let value = value.to_string();
    let github_state = std::env::var("GITHUB_STATE").unwrap_or_default();
    if github_state.is_empty() {
        println!("::save-state name={}::{}", encode_command_property(name), encode_command_data(&value));
    } else {
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(github_state)
            .unwrap();
        let delimiter = uuid::Uuid::new_v4();
        use std::io::Write;
        writeln!(file, "{name}<<{delimiter}\n{value}\n{delimiter}").unwrap();
    }
}

pub fn get_state(name: impl AsRef<str>) -> String {
    let name = name.as_ref();
    std::env::var(format!("STATE_{name}")).unwrap_or_default()
}

pub fn get_id_token() -> Result<String, Box<dyn std::error::Error>> {
    get_id_token_with_audience("")
}

pub fn get_id_token_with_audience(audience: impl AsRef<str>) -> Result<String, Box<dyn std::error::Error>> {
    #[derive(serde::Deserialize)]
    struct TokenResponse {
        value: String,
    }
    let audience = audience.as_ref();
    let mut url = std::env::var("ACTIONS_ID_TOKEN_REQUEST_URL")?;
    if !audience.is_empty() {
        url.push_str(&format!("&audience={audience}"));
    }
    let response = reqwest::blocking::get(url)?;
    let json: TokenResponse = response.json()?;
    let id_token = json.value;
    set_secret(&id_token);
    Ok(id_token)
}

pub fn to_posix_path(path: &str) -> String {
    path.replace("\\", "/")
}

pub fn to_win32_path(path: &str) -> String {
    path.replace("/", "\\")
}

pub fn to_platform_path(path: &str) -> String {
    if cfg!(windows) {
        to_win32_path(path)
    } else {
        to_posix_path(path)
    }
}

pub mod platform {
    #[cfg(target_os = "windows")]
    fn get_windows_info() -> Result<(String, String), Box<dyn std::error::Error>> {
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
        Ok((name.trim().to_string(), version.trim().to_string()))
    }

    #[cfg(target_os = "macos")]
    fn get_macos_info() -> Result<(String, String), Box<dyn std::error::Error>> {
        let version = Command::new("sw_vers")
            .arg("-productVersion")
            .output()?
            .stdout;
        let version = String::from_utf8(version)?;
        let name = Command::new("sw_vers").arg("-productName").output()?.stdout;
        let name = String::from_utf8(name)?;
        Ok((name.trim().to_string(), version.trim().to_string()))
    }

    #[cfg(target_os = "linux")]
    fn get_linux_info() -> Result<(String, String), Box<dyn std::error::Error>> {
        let name = std::process::Command::new("lsb_release").arg("-is").output()?.stdout;
        let name = String::from_utf8(name)?;
        let version = std::process::Command::new("lsb_release").arg("-rs").output()?.stdout;
        let version = String::from_utf8(version)?;
        Ok((name.trim().to_string(), version.trim().to_string()))
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
        pub name: String,
        pub platform: String,
        pub arch: String,
        pub version: String,
        pub is_windows: bool,
        pub is_macos: bool,
        pub is_linux: bool,
    }

    pub fn get_details() -> Result<Details, Box<dyn std::error::Error>> {
        #[cfg(target_os = "windows")]
        let (name, version) = get_windows_info()?;
        #[cfg(target_os = "macos")]
        let (name, version) = get_macos_info()?;
        #[cfg(target_os = "linux")]
        let (name, version) = get_linux_info()?;
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        compile_error!("unsupported target_os");
        Ok(Details {
            name,
            platform: PLATFORM.to_string(),
            arch: ARCH.to_string(),
            version,
            is_windows: IS_WINDOWS,
            is_macos: IS_MACOS,
            is_linux: IS_LINUX,
        })
    }
}

