use anyhow::{Context, Ok, Result};
use clap::Parser;

#[derive(Parser)]
#[command(name = "najsr", version, about = "Not another JS Runtimer (CLI)")]
struct Cli {
    #[arg(long)]
    verbose: bool,

    #[arg(short = 'e', long = "eval")]
    eval: Option<String>,

    file: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Cli::parse();

    if args.verbose {
        eprintln!("[najsr] starting runtime...");
    }

    let mut rt =
        runtime_core::RuntimeHandle::new_with_loader().context("failed to create runtime")?;

    let file = args.file.as_deref().context("please pass a file")?;
    rt.eval_main(file)
        .await
        .with_context(|| format!("failed running {}", file))?;

    Ok(())
}
