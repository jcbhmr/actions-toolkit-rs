//! [@actions/core](https://www.npmjs.com/package/@actions/core) for Rust projects.

mod command;
mod file_command;
mod oidc_utils;
mod path_utils;
pub mod platform;
mod summary;
mod utils;

use std::error::Error;
use std::io::Write;
use std::{env, fs};

pub struct InputOptions {
    pub required: Option<bool>,
    pub trim_whitespace: Option<bool>,
}

pub enum ExitCode {
    Success = 0,
    Failure = 1,
}

pub fn export_variable(name: &str, value: Option<&str>) -> Result<(), Box<dyn Error>> {
    let value = value.unwrap_or("");
    env::set_var(name, value);

    if let Ok(github_env) = env::var("GITHUB_ENV") {
        let mut github_env = fs::OpenOptions::new().append(true).open(github_env)?;
        writeln!(&mut github_env, "{}={}", name, value)?;
    } else {
        println!("::set-env name={},::{}", name, value);
    }

    Ok(())
}
