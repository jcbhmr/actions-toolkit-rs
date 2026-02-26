## Usage

### As a binary

```rs
use actions_core::InputOptions;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let name = actions_core::get_input("name", Some(InputOptions {
        required: Some(true),
        ..Default::default(),
    })).unwrap();
    let message = format!("Hello, {}!", name);
    actions_core::set_output("message", message);
    Ok(())
}
```

```yml
runs:
  using: 'node24'
  main: 'main.mjs'
```

```js
import * as process from "node:process";
import * as childProcess from "node:child_process";
import * as path from "node:path";
import * as os from "node:os";

const binPath = path.join(import.meta.dirname, "target/release/MYAPP");
if (process.execve) {
    process.execve(binPath);
}
const result = childProcess.spawnSync(binPath, { stdio: "inherit" });
if (result.error != null) {
    throw result.error;
}
if (result.signal != null) {
    process.kill(process.pid, result.signal);
    process.exit(128 + os.constants.signals[result.signal]);
}
if (result.status != null) {
    process.exit(result.status);
}
throw new TypeError("Expected 'error', 'signal', or 'status'");
```

### As a Node.js addon

```rs
use actions_core::InputOptions;
use napi_derive::napi;
use napi::bindgen_prelude::*;
use std::error::Error;
use std::result::Result;

fn main() -> Result<(), Box<dyn Error>> {
  let name = actions_core::get_input("name", Some(InputOptions {
    required: Some(true),
    ..Default::default(),
  })).unwrap();
  let message = format!("Hello, {}!", name);
  actions_core::set_output("message", message);
  Ok(())
}

#[napi(module_exports)]
fn module_exports(mut exports: Object, env: Env) -> napi::Result<()> {
  napi_current_env::CURRENT_ENV.sync_scope(env, || main().unwrap());
  Ok(())
}
```

## Development

This project is intended to mirror the official JavaScript https://github.com/actions/toolkit repository in purpose and functionality.

There are lots of `@actions/*` JavaScript packages that we need to implement or wrap in Rust:

- `@actions/artifact`
- `@actions/attest`
- `@actions/cache`
- `@actions/core`
- `@actions/exec`
- `@actions/github`
- `@actions/glob`
- `@actions/http-client`
- `@actions/io`
- `@actions/tool-cache`

There are three environments that our Rust versions of these packages should support:

- Compiled as **part of** a Node.js native addon using [NAPI-RS](https://napi.rs/) and running in Node.js, Deno, or Bun in a GitHub Actions runner environment
- Compiled as **part of** a WebAssembly module using [wasm-bindgen](https://wasm-bindgen.github.io/wasm-bindgen/) and running in Node.js, Deno, or Bun (**not** the browser) in a GitHub Actions runner environment
- Compiled as **part of** a regular Rust binary and running in a GitHub Actions runner environment
