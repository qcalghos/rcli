pub mod cli;
pub mod process;
mod utils;
pub use cli::{Base64Format, Base64SubCommand, Opts, SubCommand, TextSigFormat, TextSubCommand};
pub use process::{
    process_csv, process_decode, process_encode, process_genpass, process_text_sign,
    process_text_verify,process_text_generate
};
pub use utils::get_reader;
