use std::{any::Any, error::Error, path::Path};

#[test]
#[allow(unreachable_code)]
fn api() {
    return;

    // https://www.jsdocs.io/package/@actions/core

    use actions_core::*;

    // Variables
    #[allow(deprecated)]
    let _: Summary = MARKDOWN_SUMMARY;
    let _: Summary = SUMMARY;

    // Functions
    let _: fn(&str) -> Result<(), _> = add_path;
    let _: fn(&str) -> Result<(), _> = debug;
    let _: fn() -> Result<(), _> = end_group;
    let _: fn(&str) -> Result<(), _> = error;
    let _: fn(&str, &dyn Any) -> Result<(), _> = export_variable;
    let _: fn(&str, Option<&InputOptions>) -> Result<bool, _> = get_boolean_input;
    let _: fn(&str) -> Result<String, _> = get_id_token;
    let _: fn(&str, Option<&InputOptions>) -> Result<String, _> = get_input;
    let _: fn(&str, Option<&InputOptions>) -> Result<Vec<String>, _> = get_multiline_input;
    let _: fn(&str) -> Result<String, _> = get_state;
    let _: fn(&str, fn() -> ()) -> Result<(), _> = group;
    let _: fn(&str) -> Result<(), _> = info;
    let _: fn() -> Result<bool, _> = is_debug;
    let _: fn(&str) -> Result<(), _> = notice;
    let _: fn(&str, &dyn Any) -> Result<(), _> = save_state;
    let _: fn(bool) -> Result<(), _> = set_command_echo;
    let _: fn(&str) -> Result<(), _> = set_failed;
    let _: fn(&str, &dyn Any) -> Result<(), _> = set_output;
    let _: fn(&str) -> Result<(), _> = set_secret;
    let _: fn(&str) -> Result<(), _> = start_group;
    let _: fn(&str) -> String = to_platform_path;
    let _: fn(&str) -> String = to_posix_path;
    let _: fn(&str) -> String = to_win32_path;
    let _: fn(&str) -> Result<(), _> = warning;

    // Interfaces
    {
        let ap = None::<AnnotationProperties>.unwrap();
        let _: Option<u32> = ap.end_column;
        let _: Option<u32> = ap.end_line;
        let _: Option<String> = ap.file;
        let _: Option<u32> = ap.start_column;
        let _: Option<u32> = ap.start_line;
        let _: Option<String> = ap.title;
    }
    {
        let io = None::<InputOptions>.unwrap();
        let _: Option<bool> = io.required;
        let _: Option<bool> = io.trim_whitespace;
    }

    // Enums
    {
        let s: ExitCode = ExitCode::Success;
        let _: u32 = s as u32;
        let f: ExitCode = ExitCode::Failure;
        let _: u32 = f as u32;
    }

    // Namespaces
    {
        use actions_core::platform::*;
        let _: &str = ARCH;
        let _: bool = IS_LINUX;
        let _: bool = IS_MAC_OS;
        let _: bool = IS_WINDOWS;
        let _: &str = PLATFORM;
        let _: fn() -> Result<Details, _> = get_details;
    }
}
