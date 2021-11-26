mod cli;

use std::os::unix::process::CommandExt;
use std::process::Command;

use anyhow::ensure;
use clap::Parser;
use glob::glob;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();

    let from_dir = cli.from_dir;
    ensure!(from_dir.is_dir(), "Specified root path is not a directory");

    let mut ls_cmd = Command::new("ls");

    if cli.list {
        ls_cmd.arg("-l");
    }

    if cli.human_readable {
        ls_cmd.arg("-h");
    }

    if cli.sort_by_size {
        ls_cmd.arg("-S");
    }

    if cli.sort_by_time {
        ls_cmd.arg("-t");
    }

    if cli.reverse {
        ls_cmd.arg("-r");
    }

    std::env::set_current_dir(from_dir)?;
    let dots: Vec<_> = glob(".[!.]*")?.try_collect()?;

    if dots.is_empty() {
        return Ok(());
    }

    let error = ls_cmd.arg("--color=auto").arg("-d").args(dots).exec();

    Err(error.into())
}
