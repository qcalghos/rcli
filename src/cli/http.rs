use std::path::PathBuf;

use clap::Parser;
use super::verify_path;
#[derive(Debug,Parser)]
pub enum HttpSubCommand{
    #[command(name="serve",about="Serve a directory over HTTP")]
    Serve(HttpServeOpts)
}
#[derive(Debug,Parser)]
pub struct HttpServeOpts{
    #[arg(short,long,default_value=".",value_parser=verify_path)]
    pub directory:PathBuf,
    #[arg(short,long,default_value_t=8080)]
    pub port:u16,
}