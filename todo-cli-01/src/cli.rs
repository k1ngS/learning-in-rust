use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple and efficient TODO list manager")]
#[command(version = "1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new todo item
    Add {
        /// The todo title
        title: String,
        /// Optional description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// List all todos
    List {
        /// Show only completed todos
        #[arg(short, long)]
        completed: bool,
        /// Show only pending todos
        #[arg(short, long)]
        pending: bool,
    },
    /// Mark a todo as completed
    Complete {
        /// Todo ID to complete
        id: u32,
    },
    /// Mark a todo as incomplete
    Incomplete {
        /// Todo ID to mark as incomplete
        id: u32,
    },
    /// Remove a todo
    Remove {
        /// Todo ID to remove
        id: u32,
    },
    /// Show details of a specific todo
    Show {
        /// Todo ID to show
        id: u32,
    },
}
