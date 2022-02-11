mod convert;

use convert::TrackConvertCommand;
use clap::{Parser,Subcommand};
use super::GlobalArgs;

#[derive(Parser)]
#[clap(about, version, author)]
pub struct TrackCommand {
    #[clap(subcommand)]
    pub subcommand: TrackSubcommand,
}

impl TrackCommand {
    pub fn run(&self) {
        self.subcommand.run();
    }
}

#[derive(Subcommand)]
pub enum TrackSubcommand {
    Convert(TrackConvertCommand),
}

impl TrackSubcommand {
    pub fn run(&self) {
        match self {
            TrackSubcommand::Convert(x) => {
                x.run();
            }
        }
    }
}