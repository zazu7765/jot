use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

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
        #[arg(short, long)]
        date: Option<String>,

        page: Option<u32>,
    },
    Search {
        #[arg(short, long)]
        date: Option<String>,
        #[arg(short, long)]
        content: Option<Vec<String>>,

        tag: Option<String>,
    },
    Edit {
        tag: String,
    },
    Delete {
        tag: String,
    },

}

#[derive(Serialize, Deserialize)]
struct Note {
    tag: String,
    content: String,
    date: String,
}

fn main() {
    let cli = CLI::parse();
    match &cli.command {
        Some(Commands::Add { name, tag }) => {
            let tag = tag.as_deref().unwrap_or("randomhashstring").to_string();
            let data = name.join(" ");
            let date = chrono::Local::now().format("%d/%m/%y").to_string();
            let note = Note {
                tag,
                content: data,
                date,
            };
            println!("Added entry: {}", serde_json::to_string(&note).unwrap());
        }
        Some(Commands::List { page, date }) => {
            let _page = page.unwrap_or_default();
            let _date = date.as_deref().unwrap_or("");
        }
        Some(Commands::Search { date, tag, content }) => {
            let date = date.as_deref().unwrap_or("");
            let tag = tag.as_deref().unwrap_or("");
            let data = content.as_deref().unwrap_or(&["".to_string()]).join(" ");
            match (date.is_empty(), data.is_empty()) {
                (false, false) => println!("Search by date and content:\n{}\n{}", date, data),
                (false, true) => println!("Search by date and not content:\n{}", date),
                (true, false) => println!("Search by content and not date:\n{}", data),
                _ => println!("Filter by tag only {}", tag),
            }
        }
        Some(Commands::Edit { tag }) => {
            println!("Editing Entry with Tag {}", tag);
        }
        Some(Commands::Delete { tag }) => {
            println!("Deleting Entry with Tag {}", tag);
        }
        None => { println!("No Command Given!") }
    }
}
