//! Provides workflows in the [cargo-xtask][cargo-xtask] pattern.
//!
//! [cargo-xtask]: https://github.com/matklad/cargo-xtask

use std::{env, path::PathBuf};

use clap::{Parser, Subcommand};
use xshell::Shell;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {}

fn main() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    sh.change_dir(project_root());

    Ok(())
}

/// Returns the project root directory.
fn project_root() -> PathBuf {
    let dir =
        env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());
    PathBuf::from(dir).parent().unwrap().to_owned()
}
