use anyhow::Result;
use clap::Parser;
use rcli::CmdExector;
use rcli::Opts;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Opts::parse();
    cli.cmd.execute().await?;
    Ok(())
}
