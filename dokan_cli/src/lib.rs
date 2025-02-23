use crate::command::add::add;
use crate::command::list::list;
use crate::command::remove::remove;
use crate::command::Commands;
use clap::Parser;
use std::process::ExitCode;
use crate::command::cd::cd;

pub mod command;
pub(crate) mod util;

#[derive(Debug, Parser)]
#[command(bin_name = "dokan", version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

pub fn run_dokan() -> ExitCode {
    match Args::parse().command {
        Commands::List(args) => list(args),
        Commands::Add(args) => add(args.path),
        Commands::Remove(args) => remove(args.name),
        Commands::Cd(args) => cd(args),
        _ => {}
    }

    ExitCode::SUCCESS
}
