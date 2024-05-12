use crate::{process_csv, CmdExector};

use super::verify_file;
use anyhow::anyhow;
use clap::Parser;
use core::fmt;
use std::str::FromStr;
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short,long,name="input",value_parser=verify_file)]
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

impl CmdExector for CsvOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let output = if let Some(output) = self.output.clone() {
            output
        } else {
            format!("output.{}", self.format)
        };
        process_csv(&self.input, &output, self.format)?;
        Ok(())
    }
}
