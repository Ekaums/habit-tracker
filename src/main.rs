use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs::OpenOptions,
    io::{Read, Write},
};
mod cli;

#[derive(Serialize, Deserialize, Debug)]
struct Habit {
    goal: String,
    frequency: String,
    time: Option<String>,
    category: Option<String>,
    // TODO: how to track completions? completed dates?
}

fn add_habit(habit : Habit) { // TODO: does this need to be taken by ref? what happening here?

    // Load existing habits from file into vec (open for reading or create new file)
    let mut file = OpenOptions::new()
            .read(true)
            .write(true) // used solely to allow creating a new file
            .create(true)
            .open("habits.json")
            .expect("Error: Could not open habits.json");

    // deserialize data into vec
    let mut content = String::new();
    file.read_to_string(&mut content).expect("failed to read file"); // TODO: what was &mut
    let mut habits : Vec<Habit> = serde_json::from_str(&content).unwrap_or(vec![]); 
    // TODO: from_str knows to parse into Vec cuz we added it. why?
    // Why have to pass by &

    // Add new habit to vec
    habits.push(habit);

    // serialize back to JSON
    let updated_json = serde_json::to_string_pretty(&habits).unwrap(); // TODO: why pass by ref?

    // Write entire content back to file
    let mut file = OpenOptions::new() // shadowing previous def to now clear the existing file and write new contents :>
            .write(true)
            .truncate(true)
            .open("habits.json")
            .expect("Error: Could not open habits.json");

    file.write_all(updated_json.as_bytes()).expect("Failed to write to file"); // TODO: bytes?
}

fn main() {
    let args = cli::Args::parse();

    match args.command {
        cli::Commands::Add {
            goal,
            frequency,
            time,
            category,
        } => {
            let habit = Habit {
                // Create a struct with data that can be serialized into JSON format
                goal,
                frequency,
                time,
                category,
            };
            add_habit(habit);
        }
        cli::Commands::Delete { goal } => println!("Deleted {goal}"), // Every match arm is an expr, therefore doesn't need a semicolon
        cli::Commands::List => println!("Soon to come..."),
    }
}
