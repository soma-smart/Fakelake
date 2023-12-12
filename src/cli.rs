use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
/// Fakelake : Efficiently generate mockup data for load testing.
pub struct Cli {
    /// Turn verbose information on
    #[arg(short, long, action)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate files from YAML config file
    #[command(arg_required_else_help = true)]
    Generate {
        /// Path to YAML config file(s)
        #[arg(required = true)]
        path_to_config: Vec<PathBuf>,
    },
}
