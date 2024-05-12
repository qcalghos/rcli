use crate::{process_genpass, CmdExector};
use clap::Parser;
use zxcvbn::zxcvbn;
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
impl CmdExector for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let password = process_genpass(
            self.length,
            self.no_uppercase,
            self.no_lowercase,
            self.no_number,
            self.no_symbol,
        )?;
        println!("{}", password);
        //在stderr中打印密码强度
        let result = zxcvbn(&password, &[])?;
        eprintln!("密码强度:{}", result.score());
        Ok(())
    }
}
