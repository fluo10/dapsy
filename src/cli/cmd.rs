mod init;
mod playlist;
mod track;

use init::InitCmd;
pub use super::GlobalArgs;
use clap::Subcommand;
use track::TrackCommand;

#[derive(Subcommand)]
pub enum MainSubcommand {
    Init(InitCmd),
    Track(TrackCommand),
}

impl MainSubcommand {
    pub fn run(&self) {
        match self {
            MainSubcommand::Init(x) => {
                println!("Run init command");
            },
            MainSubcommand::Track(x) => {
                x.run();
            }
        }
    }
}