use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

mod cli;
use cli::*;

#[derive(Serialize, Deserialize, Debug)]
struct Habit {
    goal: String,
    frequency: String,
    time: Option<String>,
    category: Option<String>,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Add {
            goal,
            frequency,
            time,
            category,
        } => {
            let habit = Habit {
                goal,
                frequency,
                time,
                category
            };
            let val = serde_json::to_string_pretty(&habit).expect("Could not parse input");
            println!("{val}");
            fs::write("habits.json", val).expect("Failed to write to file");
            // println!("Added goal: {goal}\nDone every {frequency}");
            // if let Some(goal_duration) = time {
            //     println!("For: {goal_duration}");
            // }
            // if let Some(category_name) = category {
            //     println!("Category: {category_name}");
            // }
        }
        Commands::Delete { goal } => println!("Deleted {goal}"), // Every match arm is an expr, therefore doesn't need a semicolon
        Commands::List => println!("Soon to come..."),
    }
}
