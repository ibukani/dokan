pub mod add;
pub mod list;
pub mod remove;
pub mod cd;

use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    #[command(alias = "ls", about = "print list of project folder")]
    List(ListArgs),
    #[command(alias = "a",about = "add project")]
    Add(AddArgs),
    #[command(alias = "r" ,about = "remove project")]
    Remove(RemoveArgs),
    #[command(alias = "z", about = "change directory to project folder")]
    Cd(CdArgs),
    #[command(alias = "j", about = "select project to jump")]
    Jump( JumpArgs),
}

#[derive(Debug, Args)]
pub(crate) struct ListArgs {
    #[arg(short = 'n', long, default_value = "true", help = "show project name")]
    pub(crate) name: bool,

    #[arg(short = 'p', long, default_value = "false" ,help = "show project path")]
    pub(crate) path: bool,

    #[arg(short = 't', long, default_value = "false", help = "show project timestamp")]
    pub(crate) timestamp: bool,

    #[arg(short = 'a', long, default_value = "false", help = "show all project information")]
    pub(crate) all: bool,
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

#[derive(Debug, Args)]
pub struct CdArgs {
    #[arg(index = 1, help = "if not specified, open select menu to change directory")]
    pub(crate) name: String,
}

#[derive(Debug, Args)]
pub struct JumpArgs {

}
