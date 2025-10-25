mod cli;
mod commands;
mod storage;
mod todo;

use clap::Parser;
use cli::{Cli, Commands};
use colored::*;
use std::process;

fn main() {
    let cli = Cli::parse();

    // Load todos from file
    let mut todo_list = match storage::load_todos() {
        Ok(list) => list,
        Err(e) => {
            eprintln!("{} Failed to load todos: {}", "Error:".red().bold(), e);
            process::exit(1);
        }
    };

    // Execute command
    match cli.command {
        Commands::Add { title, description } => {
            commands::add_todo(&mut todo_list, title, description);
        }
        Commands::List { completed, pending } => {
            commands::list_todos(&todo_list, completed, pending);
        }
        Commands::Complete { id } => {
            commands::complete_todo(&mut todo_list, id);
        }
        Commands::Incomplete { id } => {
            commands::incomplete_todo(&mut todo_list, id);
        }
        Commands::Remove { id } => {
            commands::remove_todo(&mut todo_list, id);
        }
        Commands::Show { id } => {
            commands::show_todo(&todo_list, id);
        }
    }

    // Save todos to file
    if let Err(e) = storage::save_todos(&todo_list) {
        eprintln!("{} Failed to save todos: {}", "Error:".red().bold(), e);
        process::exit(1);
    }
}
