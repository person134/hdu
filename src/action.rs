use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hdu", version, about = "A lightweight cross-platform disk usage visualizer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short = 'r', long = "rate", help = "Refresh rate in milliseconds", default_value = "1000")]
    pub refresh_rate: u64,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scan a directory and print sorted results
    Scan {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Print directory tree
    Tree {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Watch disk usage (repeating TUI scan)
    Watch {
        #[arg(short = 'r', long = "rate", help = "Refresh rate in milliseconds", default_value = "1000")]
        refresh_rate: u64,
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}
