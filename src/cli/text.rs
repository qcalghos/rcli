use super::{verify_file,verify_path};
use anyhow::anyhow;
use clap::Parser;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(name = "sign", about = "Sign a message with a private/shared key.")]
    Sign(TextSignOpts),
    #[command(name = "verify", about = "Verify a signed message.")]
    Verify(TextVerifyOpts),
    #[command(name = "generate", about = "Generate a key.")]
    Generate(TextKeyGenerateOpts),
}
#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short,long,value_parser=verify_file,default_value="-")]
    pub input: String,
    #[arg(short,long,value_parser=verify_file,)]
    pub key: String,
    #[arg(long,value_parser=parse_format,default_value="blake3")]
    pub format: TextSigFormat,
}
#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short,long,value_parser=verify_file,default_value="-")]
    pub input: String,
    #[arg(short,long,value_parser=verify_file,)]
    pub key: String,
    #[arg(short, long)]
    pub sig: String,
    #[arg(long,value_parser=parse_format,default_value="blake3")]
    pub format: TextSigFormat,
}
#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(short,long,default_value="blake3",value_parser=parse_format)]
    pub format:TextSigFormat,
    #[arg(short,long,default_value="",value_parser=verify_path)]
    pub output:PathBuf,//输出路径
}
#[derive(Debug, Parser, Clone, Copy)]
pub enum TextSigFormat {
    Blake3,
    Ed25519,
}
fn parse_format(format: &str) -> Result<TextSigFormat, anyhow::Error> {
    format.parse()
}
impl FromStr for TextSigFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSigFormat::Blake3),
            "ed25519" => Ok(TextSigFormat::Ed25519),
            _ => Err(anyhow!("Invalid format.")),
        }
    }
}
impl From<TextSigFormat> for &'static str {
    fn from(value: TextSigFormat) -> Self {
        match value {
            TextSigFormat::Blake3 => "blake3",
            TextSigFormat::Ed25519 => "ed25519",
        }
    }
}
impl fmt::Display for TextSigFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}

