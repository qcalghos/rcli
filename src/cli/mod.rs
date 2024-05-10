mod base64;
mod csv;
mod gen_pass;
mod text;

pub use self::base64::Base64Format;
pub use self::base64::Base64SubCommand;
pub use self::csv::OutputFormat;
pub use self::text::{TextSigFormat, TextSubCommand};
use self::{csv::CsvOpts, gen_pass::GenPassOpts};
use ::clap::Parser;
use std::path::Path;
use std::path::PathBuf;
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
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    //判断file是否存在，或者为 "-"

    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exit.")
    }
}
fn verify_path(path:&str)->Result<PathBuf,&'static str>{
    let p=Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    }else{
        Err("File does not exit")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File does not exit."));
        assert_eq!(verify_file("./Cargo.toml"), Ok("./Cargo.toml".into()));
        assert_eq!(verify_file("no-exist"), Err("File does not exit."));
    }
}
