use crate::todo::{Todo, TodoList};
use chrono::Local;
use colored::*;

pub fn print_todo(todo: &Todo) {
    let status = if todo.completed {
        "✓".green().bold()
    } else {
        "○".yellow().bold()
    };

    let title = if todo.completed {
        todo.title.strikethrough().dimmed()
    } else {
        todo.title.normal()
    };

    let created = todo.created_at.with_timezone(&Local).format("%Y-%m-%d %H:%M");

    println!("{} [{}] {}", status, todo.id.to_string().cyan(), title);

    if let Some(ref description) = todo.description {
        println!("   {}", description.dimmed());
    }

    if todo.completed {
        if let Some(completed_at) = todo.completed_at {
            let completed = completed_at.with_timezone(&Local).format("%Y-%m-%d %H:%M");
            println!("   {} {}", "Completed:".dimmed(), completed.to_string().dimmed());
        }
    } else {
        println!("   {} {}", "Created:".dimmed(), created.to_string().dimmed());
    }

    println!();
}

pub fn list_todos(todo_list: &TodoList, show_completed: bool, show_pending: bool) {
    let todos = match (show_completed, show_pending) {
        (true, false) => todo_list.completed_todos(),
        (false, true) => todo_list.pending_todos(),
        _ => todo_list.list_todos().iter().collect(),
    };

    if todos.is_empty() {
        println!("{}", "No todos found!".yellow());
        return;
    }

    let title = match (show_completed, show_pending) {
        (true, false) => "Completed Todos",
        (false, true) => "Pending Todos",
        _ => "All Todos",
    };

    println!("{}", title.bold().underline());
    println!();

    for todo in todos {
        print_todo(todo);
    }

    // Statistics
    let total = todo_list.list_todos().len();
    let completed = todo_list.completed_todos().len();
    let pending = todo_list.pending_todos().len();

    println!("{}", "=".repeat(40).dimmed());
    println!(
        "Total: {} | Completed: {} | Pending: {}",
        total.to_string().bold(),
        completed.to_string().green(),
        pending.to_string().yellow()
    );
}

pub fn add_todo(todo_list: &mut TodoList, title: String, description: Option<String>) {
    let id = todo_list.add_todo(title.clone(), description);
    println!(
        "{} Todo '{}' added with ID {}",
        "✓".green().bold(),
        title.bold(),
        id.to_string().cyan()
    );
}

pub fn complete_todo(todo_list: &mut TodoList, id: u32) {
    match todo_list.get_todo_mut(id) {
        Some(todo) => {
            if todo.completed {
                println!("{} Todo is already completed!", "!".yellow());
            } else {
                todo.mark_completed();
                println!(
                    "{} Todo '{}' marked as completed!",
                    "✓".green().bold(),
                    todo.title.bold()
                );
            }
        }
        None => {
            println!("{} Todo with ID {} not found!", "✗".red(), id);
        }
    }
}

pub fn incomplete_todo(todo_list: &mut TodoList, id: u32) {
    match todo_list.get_todo_mut(id) {
        Some(todo) => {
            if !todo.completed {
                println!("{} Todo is already incomplete!", "!".yellow());
            } else {
                todo.mark_incomplete();
                println!(
                    "{} Todo '{}' marked as incomplete!",
                    "○".yellow().bold(),
                    todo.title.bold()
                );
            }
        }
        None => {
            println!("{} Todo with ID {} not found!", "✗".red(), id);
        }
    }
}

pub fn remove_todo(todo_list: &mut TodoList, id: u32) {
    match todo_list.remove_todo(id) {
        Some(todo) => {
            println!(
                "{} Todo '{}' removed!",
                "✗".red().bold(),
                todo.title.bold()
            );
        }
        None => {
            println!("{} Todo with ID {} not found!", "✗".red(), id);
        }
    }
}

pub fn show_todo(todo_list: &TodoList, id: u32) {
    match todo_list.get_todo(id) {
        Some(todo) => {
            println!("{}", "Todo Details".bold().underline());
            println!();
            print_todo(todo);
        }
        None => {
            println!("{} Todo with ID {} not found!", "✗".red(), id);
        }
    }
}
