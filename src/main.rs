mod command;

use clap::{Parser};
use crate::command::Commands;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands
}


fn main() {
    let _ = Args::parse();
}
