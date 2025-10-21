use anyhow::{Ok, Result};
use deno_core::{extension, JsRuntime, ModuleSpecifier, RuntimeOptions};
use ops_fs::{op_read_file, op_remove_file, op_write_file};
use std::rc::Rc;
use ts_loader::TsModuleLoader;

mod ops_fs;
mod ts_loader;

extension!(najsr, ops = [op_read_file, op_write_file, op_remove_file,]);

pub struct RuntimeHandle {
    rt: JsRuntime,
}

impl RuntimeHandle {
    pub fn new_with_loader() -> Result<Self> {
        let mut rt = JsRuntime::new(RuntimeOptions {
            module_loader: Some(Rc::new(TsModuleLoader)),
            extensions: vec![najsr::init_ops()],
            ..Default::default()
        });

        let bootstrap_script = include_str!("bootstrap.js");
        rt.execute_script("<bootstrap>", bootstrap_script)?;

        Ok(Self { rt })
    }

    pub async fn eval_main(&mut self, path: &str) -> Result<()> {
        let abs = std::fs::canonicalize(path)?;
        let spec = ModuleSpecifier::from_file_path(&abs)
            .map_err(|_| anyhow::anyhow!("Cannot convert {}", abs.display()))?;

        let mod_id = self.rt.load_main_es_module(&spec).await?;
        let result = self.rt.mod_evaluate(mod_id);

        result.await?;

        Ok(())
    }
}
