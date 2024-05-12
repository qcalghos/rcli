use std::path::PathBuf;

use crate::{process_http_serve, CmdExector};
use clap::Parser;
use enum_dispatch::enum_dispatch;

use super::verify_path;
#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
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
impl CmdExector for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_serve(self.directory, self.port).await
    }
}
