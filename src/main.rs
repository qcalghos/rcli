use anyhow::Result;
use clap::Parser;
use rcli::{
    cli::{Base64SubCommand, Opts, SubCommand},
    process_csv, process_decode, process_encode, process_genpass, TextSubCommand,
};

fn main() -> Result<()> {
    let cli = Opts::parse();
    // println!("{:?}", cli);
    match cli.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, &output, opts.format)?
        }
        SubCommand::GenPass(opts) => {
            // println!("{:?}", opts);
            process_genpass(
                opts.length,
                opts.no_uppercase,
                opts.no_lowercase,
                opts.no_number,
                opts.no_symbol,
            )?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => process_encode(&opts.input, opts.format)?,
            Base64SubCommand::Decode(opts) => process_decode(&opts.input, opts.format)?,
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                println!("{:?}", opts);
            }
            TextSubCommand::Verify(opts) => {
                println!("{:?}", opts);
            }
            TextSubCommand::Generate(opts) => {
                println!("{:?}", opts);
            }
        },
    }
    Ok(())
}
