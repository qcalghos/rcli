use crate::{process_text_generate, process_text_sign, process_text_verify, CmdExector};

use super::{verify_file, verify_path};
use anyhow::anyhow;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::fs;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
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
    pub format: TextSigFormat,
    #[arg(short,long,default_value="",value_parser=verify_path)]
    pub output: PathBuf, //输出路径
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

// impl CmdExector for TextSubCommand {
//     async fn execute(self) -> anyhow::Result<()> {
//         match self {
//             TextSubCommand::Sign(opts) => opts.execute().await,
//             TextSubCommand::Verify(opts) => opts.execute().await,
//             TextSubCommand::Generate(opts) => opts.execute().await,
//         }
//     }
// }

impl CmdExector for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let signed = process_text_sign(&self.input, &self.key, self.format)?;
        println!("{}", signed);
        Ok(())
    }
}

impl CmdExector for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let verified = process_text_verify(&self.input, &self.key, &self.sig, self.format)?;
        println!("{}", verified);
        Ok(())
    }
}

impl CmdExector for TextKeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let keys = process_text_generate(self.format)?;
        match self.format {
            TextSigFormat::Blake3 => {
                let name = self.output.join("blake3.txt");
                fs::write(name, &keys[0]).await?;
            }
            TextSigFormat::Ed25519 => {
                let name = &self.output;
                fs::write(name.join("ed25519.sk"), &keys[0]).await?;
                fs::write(name.join("ed25519.pk"), &keys[1]).await?;
            }
        }
        Ok(())
    }
}
