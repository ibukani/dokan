use std::process::ExitCode;
use clap::Parser;
use crate::command::add::add;
use crate::command::Commands;

pub mod command;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

pub fn run() -> ExitCode {
    match Args::parse().command {
        Commands::Add {} => add(),
        _ => {}
    }

    ExitCode::SUCCESS
}
