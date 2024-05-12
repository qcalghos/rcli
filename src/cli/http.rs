use std::path::PathBuf;

use crate::{process_http_serve, CmdExector};
use clap::Parser;

use super::verify_path;
#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(name = "serve", about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}
#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short,long,default_value=".",value_parser=verify_path)]
    pub directory: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExector for HttpSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpSubCommand::Serve(opts) => process_http_serve(opts.directory, opts.port).await,
        }
    }
}
