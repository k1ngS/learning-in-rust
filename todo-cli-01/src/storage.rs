use crate::todo::TodoList;
use serde_json;
use std::fs;
use std::io;
use std::path::Path;

const TODO_FILE: &str = "todos.json";

pub fn load_todos() -> io::Result<TodoList> {
    if !Path::new(TODO_FILE).exists() {
        return Ok(TodoList::new());
    }

    let contents = fs::read_to_string(TODO_FILE)?;
    let todo_list: TodoList = serde_json::from_str(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
    Ok(todo_list)
}

pub fn save_todos(todo_list: &TodoList) -> io::Result<()> {
    let contents = serde_json::to_string_pretty(todo_list)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
    fs::write(TODO_FILE, contents)?;
    Ok(())
}
