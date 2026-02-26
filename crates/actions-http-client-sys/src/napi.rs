use std::{cell::RefCell, fs, path::PathBuf};
use napi::{CleanupEnvHook, bindgen_prelude::*};
#[allow(unused_imports)]
use std::result::Result;

struct EnvWithCleanup {
    inner: Env,
    cleanup_env_hook: CleanupEnvHook<()>,
}

impl EnvWithCleanup {
    pub fn new(env: Env, f: impl 'static + FnOnce(())) -> napi::Result<Self> {
        let cleanup_env_hook = env.add_env_cleanup_hook((), f)?;
        Ok(Self {
            inner: env,
            cleanup_env_hook: cleanup_env_hook,
        })
    }
}

impl From<EnvWithCleanup> for Env {
    fn from(value: EnvWithCleanup) -> Self {
        value.inner
    }
}

impl Drop for EnvWithCleanup {
    fn drop(&mut self) {
        self.inner.remove_env_cleanup_hook(self.cleanup_env_hook).expect("remove_env_cleanup_hook should succeed");
    }
}

thread_local! {
    static ENV: RefCell<Option<EnvWithCleanup>> = RefCell::new(None);
}

pub fn set_env(value: Env) -> napi::Result<()> {
    fn env_cleanup_hook(_: ()) {
        let _ = ENV.take();
    }
    let value = EnvWithCleanup::new(value, env_cleanup_hook)?;
    ENV.set(Some(value));
    Ok(())
}

pub fn unwrap_env() -> Env {
    ENV.with_borrow(|env| {
        env.as_ref().unwrap_or_else(|| panic!("{0}::napi::ENV is None\nsuggestion: call {0}::napi::set_env from within #[napi(module_exports)]", env!("CARGO_PKG_NAME"))).inner
    })
}

fn import_meta_resolve(specifier: String) -> napi::Result<String> {
    let env = unwrap_env();
    let self_url = env.get_module_file_name()?;
    let self_path = file_url_to_path(self_url)?;
    let js_path = self_path.with_file_name("import_meta_resolve.mjs");
    const IMPORT_META_RESOLVE_JS: &str = r#"
        const resolve = import.meta.resolve.bind(import.meta);
        export { resolve as default };
    "#;
    fs::write(&js_path, IMPORT_META_RESOLVE_JS).map_err(|e| Error::from_reason(e.to_string()))?;
    todo!()
}

fn file_url_to_path(file_url: String) -> napi::Result<PathBuf> {
    let env = unwrap_env();
    let global = env.get_global()?;
    let process = global.get_named_property::<Object>("process")?;
    let url = process.get_named_property::<Function<String, Object>>("getBuiltinModule")?.call("url".to_owned())?;
    let path = url.get_named_property::<Function<String, String>>("fileURLToPath")?.call(file_url)?;
    let path = PathBuf::from(path);
    Ok(path)
}

pub fn module() -> napi::Result<Object<'static>> {
    let global = unwrap_env().get_global()?;
    let process = global.get_named_property::<Object>("process")?;
    let module = process.get_named_property::<Function<String, Object>>("getBuiltinModule")?.call("module".to_owned())?;
    let require = module.get_named_property::<Function<String, Function<String, Object>>>("createRequire")?.call("file:///home/me/fake.js".to_owned())?;
    let path = module.get_named_property::<Function<String, Object>>("resolve")?.apply(require, "@actions/http-client".to_owned())?;
}
