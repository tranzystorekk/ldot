use std::path::PathBuf;

use clap::{IntoApp, Parser};
use clap_complete::Shell;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn generate_completion(shell: Shell) {
    clap_complete::generate(
        shell,
        &mut Cli::into_app(),
        PKG_NAME,
        &mut std::io::stdout(),
    );
}

/// List hidden files and directories AKA "dotfiles"
#[derive(Parser)]
#[clap(version)]
pub struct Cli {
    /// Use a long listing format
    #[clap(short, long)]
    pub list: bool,

    /// Sort by file size, largest first
    #[clap(short = 'S', long, overrides_with = "sort-by-time")]
    pub sort_by_size: bool,

    /// Sort by time, newest first
    #[clap(short = 't', long)]
    pub sort_by_time: bool,

    /// Reverse order when sorting
    #[clap(short, long)]
    pub reverse: bool,

    /// With -l, print human-readable file sizes, e.g. 10M
    #[clap(short, long)]
    pub human_readable: bool,

    /// Specify when to colorize the output
    #[clap(long, possible_values = ["always", "auto", "never"], default_value = "auto")]
    pub color: String,

    /// Generate a shell completion script and exit
    #[clap(long, value_name = "SHELL", arg_enum)]
    pub completion: Option<Shell>,

    /// List contents inside the specified directory
    #[clap(value_name = "ROOT_DIR")]
    pub from_dir: Option<PathBuf>,
}
