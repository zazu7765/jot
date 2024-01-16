use std::fs::{create_dir_all, File};
use std::io::{Read, Seek};

use clap::{Parser, Subcommand};
use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
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

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    tag: String,
    content: String,
    date: String,
}

fn main() {
    let mut config_path = dirs::home_dir().unwrap();
    config_path.push(".config");
    create_dir_all(&config_path).expect("Could not create directories!");
    config_path.push("journal");
    config_path.set_extension("json");
    let mut db = File::options().read(true).write(true).create(true).open(&config_path).expect("Could not create file!");
    let mut data = String::new();
    db.read_to_string(&mut data).expect("Unable to Load Data!");
    let mut json: Vec<Note> = serde_json::from_str(&data).unwrap_or_else(|_e| Vec::new());
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Add { name, tag }) => {
            let tag = tag.to_owned().unwrap_or(
                Alphanumeric.sample_string(&mut thread_rng(), 5).to_lowercase()
            );
            let data = name.join(" ");
            let date = chrono::Local::now().format("%d/%m/%y").to_string();
            let note = Note {
                tag,
                content: data,
                date,
            };
            json.push(note);
            db.set_len(0).expect("Couldn't reset file");
            db.rewind().unwrap();
            serde_json::to_writer(db, &json).expect("Couldn't write json to file!");
            if cli.debug {
                // println!("Added entry: {}", serde_json::to_string(&note).unwrap());
            };
        }
        Some(Commands::List { page, date }) => {
            let _page = page.unwrap_or_default();
            let _date = date.as_deref().unwrap_or("");
            for item in json {
                println!("Tag: {}\tContent: {}",item.tag, item.content);
            }
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
            if cli.debug {
                println!("Editing Entry with Tag {}", tag);
            }
        }
        Some(Commands::Delete { tag }) => {
            if cli.debug {
                println!("Deleting Entry with Tag {}", tag);
            }
        }
        None => { println!("No Command Given!") }
    }
}
