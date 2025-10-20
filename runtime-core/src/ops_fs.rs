use deno_core::{Extension, op2};

pub fn ext_fs() -> Extension {
    Extension::builder("najsr_fs")
        .ops(vec![
            op_read_file::decl(),
            op_write_file::decl(),
            op_remove_file::decl(),
        ])
        .build()
}

#[op2(async)]
#[string]
async fn op_read_file(#[string] path: String) -> Result<String, std::io::Error> {
    tokio::fs::read_to_string(path).await
}

#[op2(async)]
async fn op_write_file(
    #[string] path: String,
    #[string] content: String,
) -> Result<(), std::io::Error> {
    tokio::fs::write(path, contents).await
}

#[op2(fast)]
fn op_remove_file(#[string] path: String) -> Result<(), std::io::Error> {
    tokio::fs::remove_file(path)
}
