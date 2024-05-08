use anyhow::anyhow;
use clap::Parser;
use core::fmt;
use std::{path::Path, str::FromStr};

#[derive(Debug, Parser)]
#[command(name="rcli",version,author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
}
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short,long,name="input",value_parser=verify_input_file)]
    pub input: String,
    #[arg(short, long, name = "output")]
    pub output: Option<String>,
    #[arg(long, name = "header", default_value_t = false)]
    pub header: bool,
    #[arg(short, long, name = "delimiter", default_value_t = ',')]
    pub delimiter: char,
    #[arg(short,long,name="format",default_value="json",value_parser=parse_format)]
    pub format: OutputFormat,
}
#[derive(Debug, Parser, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}
#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8, //密码长度
    #[arg(long, default_value_t = false)]
    pub no_lowercase: bool, //是否包含小写
    #[arg(long, default_value_t = false)]
    pub no_uppercase: bool,
    #[arg(long, default_value_t = false)]
    pub no_number: bool, //是否包含数字
    #[arg(long, default_value_t = false)]
    pub no_symbol: bool,
}
fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    //判断Path是否存在
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exit.")
    }
}
fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}
impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "toml" => Ok(OutputFormat::Toml),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow!("Invalid format")),
        }
    }
}
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Toml => "toml",
            OutputFormat::Yaml => "yaml",
        }
    }
}
impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //这里要看一下写法
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
