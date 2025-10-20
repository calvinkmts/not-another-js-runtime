use deno_core::{JsRuntime, RuntimeOptions, anyhow::Ok};

mod ops_fs;
mod ts_loader;

pub struct RuntimeHandle {
    rt: JsRuntime,
}

impl RuntimeHandle {
    pub fn new_with_loader() -> Result<Self> {
        let mut rt = JsRuntime::new(RuntimeOptions {
            extensions: vec![ops_fs::ext_fs()],
            ..Default::default()
        });

        let bootstrap_script = include_str!("bootstrap.js");
        rt.execute_script("<bootstrap>", bootstrap_script)?;

        Ok(Self { rt })
    }

    pub fn js_runtime_mut(&mut self) -> &mut JsRuntime {
        &mut self.rt
    }
}
