use deno_core::op2;

#[op2(async)]
#[string]
pub async fn op_read_file(#[string] path: String) -> Result<String, std::io::Error> {
    tokio::fs::read_to_string(path).await
}

#[op2(async)]
pub async fn op_write_file(
    #[string] path: String,
    #[string] content: String,
) -> Result<(), std::io::Error> {
    tokio::fs::write(path, content).await
}

#[op2(fast)]
pub fn op_remove_file(#[string] path: String) -> Result<(), std::io::Error> {
    std::fs::remove_file(path)
}
