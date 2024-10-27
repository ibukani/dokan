mod command;
mod script;

use std::process::ExitCode;
use clap::Parser;
use crate::command::Commands;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

pub fn run_tool() -> ExitCode {
    match Args::parse().command {
        Commands::Init(_args) => {
            script::print_script();
            ExitCode::SUCCESS
        }
    }
}
