mod command;

use clap::{Parser};
use crate::command::add::add_command;
use crate::command::Commands;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands
}


fn main() -> Result<(), ()> {
    match Args::parse().command {
        Commands::Add { }  => add_command(),
        _ => {}
    };

    Ok(())
}

pub fn add_project() {

}
