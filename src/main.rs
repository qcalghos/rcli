
use clap::Parser;
use anyhow::Result;
use rcli::{process_csv, Opts, SubCommand};


fn main()->Result<()> {
    let cli = Opts::parse();
    // println!("{:?}", cli);
    match cli.cmd{
        SubCommand::Csv(opts)=>{
            process_csv(&opts.input, &opts.output)?
        }
    }
    Ok(())
}

