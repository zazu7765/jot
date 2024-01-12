use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CLI {
    #[command(Subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {},
    List {},
    Search {},
    Edit {},
    Delete {},

}

fn main() {
    let cli = CLI::parse();
    match cli.command {
        Commands::Add {} => {}
        Commands::List {} => {}
        Commands::Search {} => {}
        Commands::Edit {} => {}
        Commands::Delete {} => {}
    }
}
