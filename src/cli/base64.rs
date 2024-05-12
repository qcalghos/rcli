use core::fmt;
use std::str::FromStr;

use crate::{process_decode, process_encode, CmdExector};

use super::verify_file;
use anyhow::anyhow;
use clap::Parser;
use enum_dispatch::enum_dispatch;
#[enum_dispatch(CmdExector)]
#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(Base64DecodeOpts),
}
#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short,long,value_parser=verify_file,default_value="-")]
    pub input: String,
    #[arg(long,value_parser=parse_base64_format,default_value="standard")]
    pub format: Base64Format,
}
#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short,long,value_parser=verify_file,default_value="-")]
    pub input: String,
    #[arg(long,value_parser=parse_base64_format,default_value="standard")]
    pub format: Base64Format,
}
#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}
fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}
impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow!("Invalid format.")),
        }
    }
}
impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
impl From<Base64Format> for &'static str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

// impl CmdExector for Base64SubCommand {
//     async fn execute(self) -> anyhow::Result<()> {
//         match self {
//             Base64SubCommand::Encode(opts) => opts.execute().await,
//             Base64SubCommand::Decode(opts) => opts.execute().await,
//         }
//     }
// }
impl CmdExector for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encoded = process_encode(&self.input, self.format)?;
        println!("{}", encoded);
        Ok(())
    }
}
impl CmdExector for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decoded = process_decode(&self.input, self.format)?;
        //TODO:判断decode出来的数据是否为字符串
        let decoded = String::from_utf8(decoded)?;
        println!("{}", decoded);
        Ok(())
    }
}
