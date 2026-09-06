mod cli;
mod commands;
mod core;

use clap::Parser;
use cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Cat { paths } => commands::cat::run(&paths),
        Commands::Loc { paths } => commands::loc::run(&paths),
        Commands::Empty { paths } => commands::empty::run(&paths),
    }
}
