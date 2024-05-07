
use clap::Parser;
use anyhow::Result;
use rcli::{process_csv, Opts, SubCommand};


fn main()->Result<()> {
    let cli = Opts::parse();
    println!("{:?}", cli);
    match cli.cmd{
        SubCommand::Csv(opts)=>{
            let output=if let Some(output)=opts.output{
                output.clone()
            }else{
                format!("output.{}",opts.format)
            };
            process_csv(&opts.input, &output,opts.format)?
        }
    }
    Ok(())
}

