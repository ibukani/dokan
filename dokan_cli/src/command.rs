pub mod add;
pub mod list;
pub mod remove;

use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    #[command(about = "print list of project folder")]
    List(ListArgs),
    #[command(about = "add project")]
    Add(AddArgs),
    #[command(about = "remove project")]
    Remove(RemoveArgs),
    #[command(about = "change directory to project folder")]
    Jump,
}

#[derive(Debug, Args)]
pub(crate) struct ListArgs {
    #[arg(short, long)]
    pub all: bool,
}

#[derive(Debug, Args)]
pub(crate) struct AddArgs {
    #[arg(long, required = true)]
    pub(crate) path: PathBuf,
}

#[derive(Debug, Args)]
pub(crate) struct RemoveArgs {
    #[arg(short, long)]
    pub name: String,
}
