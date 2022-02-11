mod args;
mod cmd;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
pub use args::GlobalArgs;
use cmd::Commands;


#[derive(Parser)]
#[clap(about, version, author)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

