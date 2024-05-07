

use clap::Parser;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name="rcli",version,author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name="csv",about="Show CSV, or Convert CSV to other formats")]
    Csv(CsvOpts),
}
#[derive(Debug, Parser)]
pub struct CsvOpts{
    #[arg(short,long,name="input",value_parser=verify_input_file)]
    pub input:String,
    #[arg(short,long,name="output",default_value="output.json")]
    pub output:String,
    #[arg(long,name="header",default_value_t=false)]
    pub header:bool,
    #[arg(short,long,name="delimier",default_value_t=',')]
    pub delimier:char
}

fn verify_input_file(filename:&str)->Result<String,&'static str>{
    //判断Path是否存在
    if Path::new(filename).exists(){
        Ok(filename.into())
    }else{
        Err("File does not exit.")
    }
}