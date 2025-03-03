use clap::{Parser, Subcommand};

// Traits provide certain functionality (like methods) or characterstics (like ability to use this type in printf) to any type
#[derive(Parser, Debug)]
#[command(
    version = "0.0.0",
    about = "habit tracker",
    long_about = None
)]
pub struct Args {
    #[command(subcommand)]
    // Tells clap that this field is a subcommand, and determines which subcommand is used
    pub command: Commands, // Holds the command that was ran
}

// TODO: what is clone?
#[derive(Subcommand, Debug)] // Subcommand trait generates code to parse subcommands and their flags
pub enum Commands {
    /// Add a new habit
    Add { // TODO: why this not pub?
        // This is a variant!! (that carries values with it)
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
