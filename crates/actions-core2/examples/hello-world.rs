use actions_core as core;
use std::error::Error;

fn main() {
    let result = || -> Result<(), Box<dyn Error>> {
        let name = core::get_input_with_options(
            "name",
            &core::InputOptions {
                required: true,
                ..Default::default()
            },
        )?;
        let favorite_color = core::get_input("favorite-color");
        core::info(format!("Hello {name}!"));
        core::set_output("message", format!("I like {favorite_color} too!"));
        Ok(())
    }();
    if let Err(error) = result {
        core::set_failed(error);
    }
}
