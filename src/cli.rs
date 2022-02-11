mod args;
mod cmd;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
pub use args::GlobalArgs;
use cmd::MainSubcommand;


#[derive(Parser)]
#[clap(about, version, author)]
pub struct MainCommand {
    #[clap(subcommand)]
    pub subcommand: MainSubcommand,
}

impl MainCommand {
    pub fn run(&self) {
        self.subcommand.run();
    }
}