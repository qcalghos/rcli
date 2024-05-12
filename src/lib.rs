pub mod cli;
pub mod process;
mod utils;

pub use cli::*;
pub use process::*;
pub use utils::get_reader;

use enum_dispatch::enum_dispatch;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}
