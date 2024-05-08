use clap::Parser;

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
