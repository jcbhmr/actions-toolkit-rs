use std::error::Error;
use std::{env, fs};
use std::path::Path;
use std::io::Write;
use uuid::Uuid;

pub fn issue_file_command(command: &str, message: Option<&str>) -> Result<(), Box<dyn Error>> {
    let file_path = env::var(format!("GITHUB_{command}"))?;
    if !Path::new(&file_path).exists() {
        return Err(format!("{file_path} does not exist").into());
    }
    let mut file = fs::OpenOptions::new().append(true).open(file_path)?;
    writeln!(&mut file, "{}", message.unwrap_or(""))?;
    Ok(())
}

pub fn prepare_key_value_message(key: &str, value: Option<&str>) -> Result<String, Box<dyn Error>> {
    let delimiter = format!("ghadelimiter_{}", Uuid::new_v4().hyphenated());
    let converted_value = value.unwrap_or("");
    if key.contains(&delimiter) {
        return Err("name contains delimiter".into());
    }
    if converted_value.contains(&delimiter) {
        return Err("value contains delimiter".into());
    }
    #[cfg(target_os = "windows")]
    let os_eol = "\r\n";
    #[cfg(not(target_os = "windows"))]
    let os_eol = "\n";
    Ok(format!("{key}<<{delimiter}{os_eol}{converted_value}{os_eol}{delimiter}"))
}