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
    pub(crate) path: Option<bool>,

    #[arg(short = 'a', long)]
    pub(crate) all: Option<bool>,
}

#[derive(Debug, Args)]
pub(crate) struct AddArgs {
    #[arg(index = 1, required = true, help = "project path")]
    pub(crate) path: PathBuf,
}

#[derive(Debug, Args)]
pub(crate) struct RemoveArgs {
    #[arg(index = 1, required = true, help = "project name")]
    pub(crate) name: String,
}
