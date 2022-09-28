use std::path::PathBuf;

use clap::{CommandFactory, Parser};
use clap_complete::Shell;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn generate_completion(shell: Shell) {
    clap_complete::generate(shell, &mut Cli::command(), PKG_NAME, &mut std::io::stdout());
}

/// List hidden files and directories AKA "dotfiles"
#[derive(Parser)]
#[command(version)]
pub struct Cli {
    /// Use a long listing format
    #[arg(short, long)]
    pub list: bool,

    /// Sort by file size, largest first
    #[arg(short = 'S', long, overrides_with = "sort-by-time")]
    pub sort_by_size: bool,

    /// Sort by time, newest first
    #[arg(short = 't', long)]
    pub sort_by_time: bool,

    /// Reverse order when sorting
    #[arg(short, long)]
    pub reverse: bool,

    /// With -l, print human-readable file sizes, e.g. 10M
    #[arg(short = 'H', long)]
    pub human_readable: bool,

    /// Specify when to colorize the output
    #[arg(long, value_parser = ["always", "auto", "never"], default_value = "auto")]
    pub color: String,

    /// Generate a shell completion script and exit
    #[arg(long, value_name = "SHELL", value_enum)]
    pub completion: Option<Shell>,

    /// List contents inside the specified directory
    #[arg(value_name = "ROOT_DIR")]
    pub from_dir: Option<PathBuf>,
}
