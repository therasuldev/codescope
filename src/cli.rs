use clap::{Parser, Subcommand};

/// codescope — a fast CLI for exploring codebases.
#[derive(Parser)]
#[command(name = "cs", version, about = "Explore, count and audit your codebase")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Print the contents of files inside one or more paths.
    Cat {
        /// One or more file or folder paths.
        #[arg(required = true)]
        paths: Vec<String>,
    },
    /// Count lines of code across one or more paths.
    Loc {
        /// One or more file or folder paths.
        #[arg(required = true)]
        paths: Vec<String>,
    },
    /// Find empty files and folders across one or more paths.
    Empty {
        /// One or more file or folder paths.
        #[arg(required = true)]
        paths: Vec<String>,
    },
}
