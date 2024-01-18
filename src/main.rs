use std::fs::{create_dir_all, File};
use std::io::{Read, Seek, stdin};

use clap::{Parser, Subcommand};
use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;
use serde::{Deserialize, Serialize};

mod filters;

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
        arg: Vec<String>,
        #[arg(short, long)]
        from: Option<String>,

        #[arg(short, long)]
        to: Option<String>,

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
            if cli.debug {
                println!("Added entry: {}", serde_json::to_string(&note).unwrap());
            };
            json.push(note);
            write_over_config(&mut db, &mut json);
        }
        Some(Commands::List { arg, from, to }) => {
            let args = arg.join(" ");
            match args.as_str(){
                "date" => {
                    let start_date = from.as_deref().unwrap_or("");
                    let end_date = to.as_deref().unwrap_or("");
                    filters::filter_by_date(&mut json, start_date, end_date);
                },
                "tag" => {
                    filters::filter_by_tag(&mut json);
                },
                _ => (),
            }
            for item in json {
                println!("Tag: {}\tContent: {}\tDate: {}", item.tag, item.content, item.date);
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
            if let Some(note) = json.iter_mut().find(|x| x.tag == tag.as_str()) {
                let mut new_content = String::new();
                println!("Please enter your new content: ");
                stdin().read_line(&mut new_content).expect("Unable to read user input!");
                note.content = new_content.trim().to_string();
                write_over_config(&mut db, &mut json);
            }

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

fn write_over_config(db: &mut File, notes: &mut Vec<Note>) {
    db.set_len(0).expect("Couldn't reset file");
    db.rewind().unwrap();
    serde_json::to_writer(db, &notes).expect("Couldn't write json to file!");
}
