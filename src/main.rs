use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short, long)]
    debug: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        name: Vec<String>,
        #[arg(short, long)]
        tag: Option<String>,
    },
    List {
        page: Option<u32>,
    },
    Search {
        #[arg(short, long)]
        date: Option<String>,
        #[arg(short, long)]
        tag: Option<String>,
        content: Option<Vec<String>>,
    },
    Edit {},
    Delete {},

}

fn main() {
    let cli = CLI::parse();
    match &cli.command {
        Some(Commands::Add { name, tag }) => {
            let data = name.join(" ");
            println!("{:?}", data);
        }
        Some(Commands::List { page }) => {
            let page = page.unwrap_or_default();
        }
        Some(Commands::Search { date, tag, content }) => {
            let date = date.as_deref().unwrap_or("");
            let tag = tag.as_deref().unwrap_or("");
            match (date.is_empty(), tag.is_empty()) {
                (false, false) => println!("Filter by date and tag"),
                (false, true) => println!("Filter by date NOT tag"),
                (true, false) => println!("Filter by tag NOT date"),
                _ => println!("Filter by content only"),
            }
        }
        Some(Commands::Edit {}) => {}
        Some(Commands::Delete {}) => {}
        None => { println!("No Command Given!") }
    }
}
