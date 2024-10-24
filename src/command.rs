use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "print list of project folder")]
    List,
    #[command(about = "add project")]
    Add,
    #[command(about = "remove project")]
    Remove,
    #[command(about = "change directory to project folder")]
    Jump
}