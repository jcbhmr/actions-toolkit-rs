use actions_core as core;
use std::error::Error;

fn main() {
    let result = || -> Result<(), Box<dyn Error>> {
        let name = core::get_input_with_options(
            "name",
            core::GetInputOptions {
                required: true,
                ..Default::default()
            },
        )?;
        let favorite_color = core::get_input("favorite-color")?;
        core::info!("Hello {name}!");
        core::set_output("message", "Wow! Rust is awesome!");
        Ok(())
    }();
    if let Err(error) = result {
        core::set_failed!("{error}");
    }
}
