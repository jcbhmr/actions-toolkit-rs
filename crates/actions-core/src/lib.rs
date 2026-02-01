pub mod platform;

use std::{any::Any, error::Error};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Summary {
    _private: (),
}

impl Summary {}

#[deprecated]
pub const MARKDOWN_SUMMARY: Summary = Summary { _private: () };

pub const SUMMARY: Summary = Summary { _private: () };

pub fn add_path(path: &str) -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn debug(message: &str) -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn end_group() -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn error(message: &str) -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn export_variable(name: &str, value: &dyn Any) -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn get_boolean_input(name: &str, options: Option<&InputOptions>) -> Result<bool, Box<dyn Error>> {
    todo!()
}

pub fn get_id_token(audience: &str) -> Result<String, Box<dyn Error>> {
    todo!()
}

pub fn get_input(name: &str, options: Option<&InputOptions>) -> Result<String, Box<dyn Error>> {
    todo!()
}

pub fn get_multiline_input(name: &str, options: Option<&InputOptions>) -> Result<Vec<String>, Box<dyn Error>> {
    todo!()
}

pub fn get_state(name: &str) -> Result<String, Box<dyn Error>> {
    todo!()
}

pub fn group<R>(name: &str, f: impl FnOnce() -> R) -> Result<R, Box<dyn Error>> {
    todo!()
}

pub fn info(message: &str) -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn is_debug() -> Result<bool, Box<dyn Error>> {
    todo!()
}

pub fn notice(message: &str) -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn save_state(name: &str, value: &dyn Any) -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn set_command_echo(enabled: bool) -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn set_failed(message: &str) -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn set_output(name: &str, value: &dyn Any) -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn set_secret(secret: &str) -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn start_group(name: &str) -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn to_platform_path(p: &str) -> String {
    todo!()
}

pub fn to_posix_path(p: &str) -> String {
    todo!()
}

pub fn to_win32_path(p: &str) -> String {
    todo!()
}

pub fn warning(message: &str) -> Result<(), Box<dyn Error>> {
    todo!()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AnnotationProperties {
    pub end_column: Option<u32>,
    pub end_line: Option<u32>,
    pub file: Option<String>,
    pub start_column: Option<u32>,
    pub start_line: Option<u32>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct InputOptions {
    pub required: Option<bool>,
    pub trim_whitespace: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ExitCode {
    Success = 0,
    Failure = 1,
}
