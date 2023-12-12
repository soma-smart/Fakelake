mod cli;
mod config;
mod generate;
mod logger;
mod errors;
mod providers;

use crate::cli::{Cli, Commands};
use crate::generate::generate_from_paths;
use clap::Parser;
use log::error;

fn main() {
    let cli = Cli::parse();

    match cli.verbose {
        true => logger::init(1),
        false => logger::init(0),
    }

    match cli.command {
        Commands::Generate {
            path_to_config: paths_to_config,
        } => {
            match generate_from_paths(paths_to_config) {
                Ok(_) => (),
                Err(e) => {
                    error!("Error: {:?}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
