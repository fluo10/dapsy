mod init;
mod playlist;
mod track;

use init::InitCmd;
pub use super::GlobalArgs;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Init(InitCmd),
}