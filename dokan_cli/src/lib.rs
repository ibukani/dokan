use crate::command::add::add;
use crate::command::list::list;
use crate::command::remove::remove;
use crate::command::Commands;
use clap::Parser;
use std::process::ExitCode;

pub mod command;
pub(crate) mod util;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

pub fn run() -> ExitCode {
    match Args::parse().command {
        Commands::List {} => list(),
        Commands::Add(args) => add(args.path),
        Commands::Remove {} => remove(),
        _ => {}
    }

    ExitCode::SUCCESS
}
