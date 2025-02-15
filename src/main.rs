use clap::{Parser, Subcommand};

// Traits provide certain functionality (like methods) or characterstics (like ability to use this type in printf) to any type
#[derive(Parser, Debug)]
// additional functionality. If user provide `-h` or `-V`, it will print this info
#[command(
    version = "0.0.0",
    about = "habit tracker",
    long_about = None
)]
struct Args {
    #[command(subcommand)]
    // Tells clap that this field is a subcommand, and determines which subcommand is used
    command: Commands, // Holds the command that was ran
}

// TODO: what is clone?
#[derive(Subcommand, Debug)] // Subcommand trait generates code to parse subcommands and their flags
enum Commands {
    /// Add a new habit
    Add {
        #[arg(short, long, help = "Name of the habit")]
        goal: String,

        #[arg(short, long, help = "How often")]
        frequency: String,

        #[arg(short, long, help = "Time to spend per session (optional)")]
        time: Option<String>, // Similar to std::optional. Can take on either value `Some(String)` or `None`

        #[arg(short, long, help = "Category of the habit (optional)")]
        category: Option<String>,
    },
    /// Delete a habit
    Delete {
        #[arg(short, long)]
        goal: String,
    },
    /// List all habits
    List,
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
            println!("Added goal: {goal}\nDone every {frequency}");
            if let Some(goal_duration) = time {
                println!("For: {goal_duration}");
            }
            if let Some(category_name) = category {
                println!("Category: {category_name}");
            }
        }
        Commands::Delete { goal } => println!("Deleted {goal}"), // Every match arm is an expr, therefore doesn't need a semicolon
        Commands::List => println!("Soon to come..."),
    }
}
