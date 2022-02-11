mod cli;
mod config;
mod converter;
mod playlist;
mod track;
mod library;

pub use track::Track;
pub use library::Library;
pub use config::Config;
use cli::MainCommand;
use clap::Parser;


fn main() {
    let command = MainCommand::parse();
    command.run();
}
