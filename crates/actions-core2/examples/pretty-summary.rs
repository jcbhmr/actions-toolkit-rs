use actions_core as core;
use std::error::Error;

fn main() {
    let result = || -> Result<(), Box<dyn Error>> {
        let name = core::get_input("name");
        let mut summary = core::SUMMARY.lock().unwrap();
        summary.add_heading("Hello world!");
        summary.add_raw(format!("Hello, {}!", name));
        summary.add_code_block_with_lang("console.log(42);", "js");
        Ok(())
    }();
    if let Err(error) = result {
        core::set_failed(error);
    }
}
