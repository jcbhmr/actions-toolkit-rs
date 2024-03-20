![🚧 Under construction 👷‍♂️](https://i.imgur.com/LEP2R3N.png)

- [x] Create actions-toolkit.rs
- [ ] ⭐ actions-core
- [ ] 🤢 actions-exec
- [ ] actions-glob
- [ ] 🤢 actions-http-client
- [ ] 🤢 actions-io
- [ ] ⭐ actions-tool-cache
- [ ] ⭐ actions-github
- [ ] actions-artifact
- [ ] actions-cache
- [ ] Publish to crates.io
- [ ] Each package has its own readme
- [ ] Each package has its own set of examples
- [ ] Most things have good documentation
- [ ] [configure-executable-action](https://github.com/jcbhmr/configure-executable-action)
- [ ] [hello-world-rust-action](https://github.com/jcbhmr/hello-world-rust-action)

# GitHub Actions Toolkit for Rust

🦀 The unofficial toolkit for developing Rust-based GitHub Actions

<table align=center><td>

```rs
let name = actions_core::get_input("name")?;
actions_core::info!("Hello, {name}!");

let hash = actions_glob::hash_files("*.json")?;
actions_core::set_output("hash", hash);
```

</table>

<p align=center>
  <a href="https://docs.rs/actions-core/latest/actions_core">docs.rs/actions-core</a>
  | <a href="https://github.com/jcbhmr/hello-world-rust-action">hello-world-rust-action</a>
  | <a href="https://github.com/jcbhmr/configure-executable-action">jcbhmr/configure-executable-action</a>
  | <a href="https://github.com/actions/toolkit">Official actions/toolkit</a>
</p>

😵 Don't know where to start? Read the [🦀 How to write your GitHub Actions in Rust](https://dev.to/#) post. 🚀

- **[actions-core](./crates/actions-core):** ✅ Get inputs, set outputs, and other basic operations for GitHub Actions
- **[actions-exec](./crates/actions-exec):** 🏃‍♂️ Easier interface for running subprocesses in GitHub Actions
- **[actions-glob](./crates/actions-glob):** 🍦 Glob expression library for GitHub Actions
- **[actions-http-client](./crates/actions-http-client):** 📞 An HTTP client optimized for GitHub Actions
- **[actions-io](./crates/actions-io):** ✏️ Common I/O utilities as a single package for GitHub Actions
- **[actions-tool-cache](./crates/actions-tool-cache):** 🔨 Functions for downloading and caching tools in GitHub Actions
- **[actions-github](./crates/actions-github):** :octocat: Provides an Octokit.rs client already setup in the GitHub Actions context
- **[actions-artifact](./crates/actions-artifact):** 💾 Functions to interact with Actions artifacts
- **[actions-cache](./crates/actions-cache):** 🎯 Functions to cache dependencies and build outputs for GitHub Actions
- **[actions-attest](./crates/actions-attest):** 🔏 Functions to write attestations for workflow artifacts

## Installation

![Cargo](https://img.shields.io/static/v1?style=for-the-badge&message=Cargo&color=e6b047&logo=Rust&logoColor=000000&label=)

Your reading the root monorepo readme. 😉 To install a specific `actions-*` package just click one of the links above ☝ and install that package. 🚀

You'll probably also want to [create an `action.yml` manifest and use configure-executable-action](https://github.com/jcbhmr/configure-executable-action) to manage the glue that will invoke your `target/release/*` executable.

## Usage

![Rust](https://img.shields.io/static/v1?style=for-the-badge&message=Rust&color=000000&logo=Rust&logoColor=FFFFFF&label=)

Here's an example that uses some (not all) of the `actions-*` packages in this monorepo listed above. Each package listed above ☝ also has its own readme with specific examples related to its specialty.

```rs
use actions_core as core;
use actions_github as github;
use std::error::Error;
use chrono::prelude::*;

fn main() {
  let result = || -> Result<(), Box<dyn Error>> {
    // The `who-to-greet` input is defined in action metadata file
    let who_to_greet = core::get_input_opts("who-to-greet", {})?;
    core::info!("Hello, {who_to_greet}!");

    // Get the current time and set it as an output
    let time = Utc::now().to_string()
    core::set_output("time", time);

    // Output the payload for debugging
    let json = serde_json::to_string_pretty(github::context::payload)?;
    core::info!("The event payload: {json}");
    Ok(())
  }()
  if let Err(error) = result {
    core::set_failed!("{error}");
  }
}
```

🔰 Based on the [hello-world-javascript-action](https://github.com/actions/hello-world-javascript-action) code example.

🤖 Check out [configure-executable-action](https://github.com/jcbhmr/configure-executable-action) for an easy way to configure and release your `target/release/*` executable! 😉

## Alternatives

There have been a few incomplete attempts to port the GitHub Actions actions/toolkit API to Rust before. [kjvalencik/actions](https://github.com/kjvalencik/actions) published [actions-toolkit](https://crates.io/crates/actions-toolkit) and [actions-core](https://crates.io/crates/actions-core). [romnn/action-rs](https://github.com/romnn/action-rs) published [action-artifact](https://crates.io/crates/action-artifact), [action-core](https://crates.io/crates/action-core), [action-derive](https://crates.io/crates/action-derive), [action-exec](https://crates.io/crates/action-exec), [action-github](https://crates.io/crates/action-github), [action-glob](https://crates.io/crates/action-glob), [action-http-client](https://crates.io/crates/action-http-client), and [action-io](https://crates.io/crates/action-io). Both of these projects appear to be stale that don't have much of an ecosystem surrounding them.

This actions-toolkit.rs project and the accompanying configure-executable-action configurator **aim to make Rust-based GitHub Actions a second-class citizen** -- an improvement on the current third-class status of relying Bash-based wrapper action configuration to call your Rust binary.

:octocat: Interested in writing your GitHub Action another way? Check out the official guides on [Creating a compsite action](https://docs.github.com/en/actions/creating-actions/creating-a-composite-action), [Creating a Docker container action](https://docs.github.com/en/actions/creating-actions/creating-a-docker-container-action), and [Creating a JavaScript action](https://docs.github.com/en/actions/creating-actions/creating-a-javascript-action).

## Development

![Rust](https://img.shields.io/static/v1?style=for-the-badge&message=Rust&color=000000&logo=Rust&logoColor=FFFFFF&label=)
![Cargo](https://img.shields.io/static/v1?style=for-the-badge&message=Cargo&color=e6b047&logo=Rust&logoColor=000000&label=)
![Docs.rs](https://img.shields.io/static/v1?style=for-the-badge&message=Docs.rs&color=000000&logo=Docs.rs&logoColor=FFFFFF&label=)

This project really tries to mirror the API and package surface of [actions/toolkit] as closely as is reasonable. **The goal is to replicate the well-known [@actions/core](https://www.npmjs.com/package/@actions/core) and friends API surface so that you only have to learn how to do things one way.** This has the convenient side effect of making existing documentation designed for JavaScript-based GitHub Actions "work" for Rust-based GitHub Actions.

To get started tinkering you can clone this repository and run `cargo test` to see everything happen! 🤩 There's no current root package so you'll need to specify a `--package <package_name>` or `-p <package_name>` argument to some `cargo` commands.

~~This is a monorepo where each package independently manages its own version. This means that releasing the release & tag semantics are a bit wacky -- instead of a single tag like `v1.0.0` for each version of the entire project there's now N different tags, one for each `actions-*` package, each named according to `${package_name}-${package_version}` convention.~~ Nothing is currently published yet so this doesn't apply. I also want to do some research to see what the convention around monorepo tag naming is.

This repository was originally named "toolkit.rs" but since the organization/user name isn't actions-related ([@jcbhmr](https://github.com/jcbhmr) instead of [@actions](https://github.com/actions)) I added a prefix to clarify the name.
