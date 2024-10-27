use clap::{Args, Subcommand};
use crate::script::ShellType;

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    Init(InitArgs),
}

#[derive(Debug, Args)]
pub struct InitArgs {
    #[arg(value_enum)]
    pub shell_type: ShellType,
}