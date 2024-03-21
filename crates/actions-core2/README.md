![🚧 Under construction 👷‍♂️](https://i.imgur.com/LEP2R3N.png)

# actions-core

✅ Get inputs, set outputs, and other basic operations for GitHub Actions

<table align=center><td>

```rs
let name = core::get_input_with_options("name", &core::InputOptions {
  required: true,
  ..Default::default()
})?;
let favorite_color = core::get_input("favorite-color");
core::info(format!("Hello {name}!"));
core::set_output("message", format!("I like {favorite_color} too!"));
```

</table>

👀 Looking for more GitHub Actions crates? Check out [the actions-toolkit.rs project](https://github.com/jcbhmr/actions-toolkit.rs).

## Installation

```sh
cargo add actions-core2
```

⚠️ Use `use actions_core` in your Rust code. The package name differs from the crate name.

## Usage

![Rust](https://img.shields.io/static/v1?style=for-the-badge&message=Rust&color=000000&logo=Rust&logoColor=FFFFFF&label=)

```rs
use actions_core as core;
use std::error::Error;

fn main() {
  let result = || -> Result<(), Box<dyn Error>> {
    let name = core::get_input_with_options("name", core::InputOptions {
        required: true,
        ..Default::default()
    })?;
    let favorite_color = core::get_input("favorite-color")?;
    core::info!("Hello {name}!");
    core::set_output("message", "Wow! Rust is awesome!");
    Ok(())
  }();
  if let Err(error) = result {
    core::set_failed!("{error}");
  }
}
```

🤔 But how do I actually use the generated executable in my `action.yml`? Check out [configure-executable-action](https://github.com/jcbhmr/configure-executable-action)!

## Development

![Rust](https://img.shields.io/static/v1?style=for-the-badge&message=Rust&color=000000&logo=Rust&logoColor=FFFFFF&label=)
![Cargo](https://img.shields.io/static/v1?style=for-the-badge&message=Cargo&color=e6b047&logo=Rust&logoColor=000000&label=)
![Docs.rs](https://img.shields.io/static/v1?style=for-the-badge&message=Docs.rs&color=000000&logo=Docs.rs&logoColor=FFFFFF&label=)

This project is part of the [actions-toolkit.rs](https://github.com/jcbhmr/actions-toolkit.rs) project.

🆘 I'm not a very proficient Rust programmer. If you see something that could be better, please tell me! ❤️ You can open an Issue, Pull Request, or even just comment on a commit. You'll probably be granted write access. 😉

Todo list:

- [x] Replicate the public API surface from [@actions/core](https://www.npmjs.com/package/@actions/core). Falsey string behaviour included.
- [ ] Decide on `get_input("name", Some(...))` vs `get_input_with_options("name", ...)` vs `get_input!("name", ...)`. Need to find existing Rust projects to see the convention.
- [ ] Figure out when to use `AsRef<str>`, `&str`, `String`, `Cow<str>`, etc. for parameters and return types. I need to do some recon on existing Rust projects.
- [ ] Publish this crate to crates.io. That also entails setting up GitHub Actions to publish the crate on each appropriate monorepo release.
- [ ] Copy this content to the crate README.
- [ ] Add examples. At least two.
- [ ] Add documentation to the public API. Not just "get_input() gets the input".
