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
use std::collections::HashMap;
use std::error::Error;

/// This struct is used by [get_input_with_options], [get_multiline_input_with_options], and [get_boolean_input_with_options]. The normal [get_input], [get_multiline_input], and [get_boolean_input] functions are also available which do not use these options and assume the default values.
/// 
/// This struct implements the [Default] trait so that you can use `..Default::default()` to use the default values. But it only has two feilds so you may find this pattern a bit overkill. 🤷‍♀️
/// 
/// ```rs
/// // We're using `?` to immediately bubble up the error.
/// let name = core::get_input_with_options("name", core::InputOptions {
///   required: true,
///   ..Default::default()
/// })?;
/// ```
pub struct InputOptions {
    /// Whether or not the input is required. When this is set to `true` and the input is not provided, the [get_input_with_options] or similar function will return an [Error]-ed [Result]. This defaults to `false`.
    pub required: bool,
    /// Runs `.trim()` on the input if set to `true`. This defaults to `true`.
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

/// This value only has function in the JavaScript implementation to `process.exitCode = $X` in `core.setFailed()`. There's no equivalent of `process.exitCode = $X` in Rust.
#[deprecated]
pub enum ExitCode {
    Success = 0,
    Failure = 1,
}

/// Used with [error_with_properties], [warning_with_properties], and [notice_with_properties] to link a message to a specific file or section of a file. If this is specified, the message will show up in the GitHub web UI when browsing that file in a Pull Request. This annotation information is particularly useful when printing compiler or build tool messages. All of the fields **are optional** even though they all need a concrete value. If the value is "falsey" (`0` or `""``) then the value won't be included in the message log and acts as though it were an [Option].
/// 
/// ```rs
/// core::error_with_properties(format!("Oh no! {error:?}"), core::AnnotationProperties {
///   title: "Compiler error",
///   file: "src/main.rs",
///   start_line: 1,
///   end_line: 5,
///   ..Default::default()
/// });
/// ```
pub struct AnnotationProperties<'a> {
    /// Pseudo-[Option] where `""` is [None] and any other non-zero length string is [Some]. This string will show up as bold text on the GitHub Actions summary page for the workflow run.
    pub title: &'a str,
    /// Pseudo-[Option] where `""` is [None] and any other non-zero length string is [Some]. If this is specified then the message is considered to be tied to a specific file. This field should be specified if `start_line` and `stop_line` are specified. If not it's a soft error.
    pub file: &'a str,
    /// Pseudo-[Option] where `0` is [None] and any other non-zero integer is [Some]. This field should be specified if `file` is specified. If not it's a soft error.
    pub start_line: u32,
    /// Pseudo-[Option] where `0` is [None] and any other non-zero integer is [Some]. This field should be specified if `file` is specified. If not it's a soft error.
    pub end_line: u32,
    /// Pseudo-[Option] where `0` is [None] and any other non-zero integer is [Some]. This field should be specified if `stop_column` is specified. If not it's a soft error.
    pub start_column: u32,
    /// Pseudo-[Option] where `0` is [None] and any other non-zero integer is [Some]. This field should be specified if `stop_column` is specified. If not it's a soft error.
    pub end_column: u32,
}

pub fn export_variable(name: &str, value: Option<String>) -> Result<(), Box<dyn Error>> {
    let converted_value = to_command_value(value);
    env::set_var(name, converted_value);
    let file_path = env::var("GITHUB_ENV").unwrap_or_default();
    if !file_path.is_empty() {
        issue_file_command("ENV", Some(prepare_key_value_message(name, value)?))?;
    } else {
        issue_command(
            "set-env",
            CommandProperties::from([("name".into(), name.into())]),
            Some(converted_value),
        )?;
    }
    Ok(())
}

pub fn set_secret(secret: &str) -> Result<(), Box<dyn Error>> {
    issue_command("add-mask", CommandProperties::new(), Some(secret))?;
    Ok(())
}

pub fn add_path(input_path: &str) -> Result<(), Box<dyn Error>> {
    let file_path = env::var("GITHUB_PATH").unwrap_or_default();
    if !file_path.is_empty() {
        issue_file_command("PATH", Some(input_path))?;
    } else {
        issue_command("add-path", CommandProperties::new(), Some(input_path))?;
    }
    #[cfg(target_os = "windows")]
    let path_delimiter = ";";
    #[cfg(not(target_os = "windows"))]
    let path_delimiter = ":";
    env::set_var(
        "PATH",
        format!("{}{path_delimiter}{input_path}", env::var("PATH")?),
    );
    Ok(())
}

pub fn get_input(name: &str, options: Option<&InputOptions>) -> Result<String, Box<dyn Error>> {
    let value =
        env::var(format!("INPUT_{}", name.replace(' ', "_").to_uppercase())).unwrap_or_default();
    if let Some(options) = options {
        if options.required.unwrap_or_default() && value.is_empty() {
            return Err(format!("input {name} required").into());
        }
        if options.trim_whitespace.is_some_and(|x| x == false) {
            return Ok(value);
        }
    }
    Ok(value.trim().into())
}

pub fn get_multiline_input(
    name: &str,
    options: Option<&InputOptions>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let inputs = get_input(name, options)?
        .split("\n")
        .map(|x| x.into())
        .collect::<Vec<String>>();
    if options.is_some_and(|options| options.trim_whitespace.is_some_and(|x| x == false)) {
        return Ok(inputs);
    }
    Ok(inputs.iter().map(|x| x.trim().into()).collect())
}

pub fn get_boolean_input(
    name: &str,
    options: Option<&InputOptions>,
) -> Result<bool, Box<dyn Error>> {
    let true_value = vec!["true", "True", "TRUE"];
    let false_value = vec!["false", "False", "FALSE"];
    let value = get_input(name, options)?;
    if true_value.contains(&value.as_str()) {
        return Ok(true);
    }
    if false_value.contains(&value.as_str()) {
        return Ok(false);
    }
    Err(format!("{name} not `true | True | TRUE | false | False | FALSE`").into())
}

pub fn set_output(name: &str, value: Option<String>) -> Result<(), Box<dyn Error>> {
    let file_path = env::var("GITHUB_OUTPUT").unwrap_or_default();
    if !file_path.is_empty() {
        issue_file_command("OUTPUT", Some(prepare_key_value_message(name, value)?))?;
    } else {
        println!();
        issue_command(
            "set-output",
            CommandProperties::from([("name".into(), name.into())]),
            Some(to_command_value(value)),
        )?;
    }
    Ok(())
}

pub fn set_command_echo(enabled: bool) -> Result<(), Box<dyn Error>> {
    issue(
        "echo",
        Some(if enabled { "on".into() } else { "off".into() }),
    )?;
    Ok(())
}

pub fn set_failed(message: Box<dyn Error>) -> Result<(), Box<dyn Error>> {
    error(message, None);
    Ok(())
}

pub fn is_debug() -> bool {
    env::var("RUNNER_DEBUG").is_ok_and(|x| x == "1")
}

pub fn debug(message: &str) -> Result<(), Box<dyn Error>> {
    issue_command("debug", CommandProperties::new(), Some(message.into()))?;
    Ok(())
}

pub fn error(
    message: Box<dyn Error>,
    properties: Option<AnnotationProperties>,
) -> Result<(), Box<dyn Error>> {
    let properties = properties.unwrap_or(AnnotationProperties {
        title: None,
        file: None,
        start_line: None,
        end_line: None,
        start_column: None,
        end_column: None,
    });
    issue_command(
        "error",
        to_command_properties(properties),
        Some(format!("{message}")),
    );
    Ok(())
}

pub fn warning(
    message: Box<dyn Error>,
    properties: Option<AnnotationProperties>,
) -> Result<(), Box<dyn Error>> {
    let properties = properties.unwrap_or(AnnotationProperties {
        title: None,
        file: None,
        start_line: None,
        end_line: None,
        start_column: None,
        end_column: None,
    });
    issue_command(
        "warning",
        to_command_properties(properties),
        Some(format!("{message}")),
    );
    Ok(())
}

pub fn notice(
    message: Box<dyn Error>,
    properties: Option<AnnotationProperties>,
) -> Result<(), Box<dyn Error>> {
    let properties = properties.unwrap_or(AnnotationProperties {
        title: None,
        file: None,
        start_line: None,
        end_line: None,
        start_column: None,
        end_column: None,
    });
    issue_command(
        "notice",
        to_command_properties(properties),
        Some(format!("{message}")),
    );
    Ok(())
}

pub fn info(message: &str) {
    println!("{message}");
}

pub fn start_group(name: &str) -> Result<(), Box<dyn Error>> {
    issue("group", Some(name.into()))?;
    Ok(())
}

pub fn end_group() -> Result<(), Box<dyn Error>> {
    issue("endgroup", None)?;
    Ok(())
}

pub fn group<T, F: FnOnce() -> T>(name: &str, f: F) -> Result<T, Box<dyn Error>> {
    start_group(name)?;
    let result = f();
    end_group()?;
    Ok(result)
}

pub fn save_state(name: &str, value: Option<String>) -> Result<(), Box<dyn Error>> {
    let file_path = env::var("GITHUB_STATE").unwrap_or_default();
    if !file_path.is_empty() {
        issue_file_command("STATE", Some(prepare_key_value_message(name, value)?))?;
    } else {
        issue_command(
            "save-state",
            CommandProperties::from([("name".into(), name.into())]),
            Some(to_command_value(value)),
        )?;
    }
    Ok(())
}

pub fn get_state(name: &str) -> Result<String, Box<dyn Error>> {
    Ok(env::var(format!("STATE_{name}")).unwrap_or_default())
}

pub fn get_id_token(audience: Option<String>) -> Result<String, Box<dyn Error>> {
    Ok(OidcClient::get_id_token(audience)?)
}
