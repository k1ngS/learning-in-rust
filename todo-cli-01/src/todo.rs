use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Todo {
    pub fn new(id: u32, title: String, description: Option<String>) -> Self {
        Todo {
            id,
            title,
            description,
            completed: false,
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    pub fn mark_completed(&mut self) {
        self.completed = true;
        self.completed_at = Some(Utc::now());
    }

    pub fn mark_incomplete(&mut self) {
        self.completed = false;
        self.completed_at = None;
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TodoList {
    pub todos: Vec<Todo>,
    pub next_id: u32,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_todo(&mut self, title: String, description: Option<String>) -> u32 {
        let id = self.next_id;
        let todo = Todo::new(id, title, description);
        self.todos.push(todo);
        self.next_id += 1;
        id
    }

    pub fn get_todo(&self, id: u32) -> Option<&Todo> {
        self.todos.iter().find(|todo| todo.id == id)
    }

    pub fn get_todo_mut(&mut self, id: u32) -> Option<&mut Todo> {
        self.todos.iter_mut().find(|todo| todo.id == id)
    }

    pub fn remove_todo(&mut self, id: u32) -> Option<Todo> {
        if let Some(pos) = self.todos.iter().position(|todo| todo.id == id) {
            Some(self.todos.remove(pos))
        } else {
            None
        }
    }

    pub fn list_todos(&self) -> &Vec<Todo> {
        &self.todos
    }

    pub fn completed_todos(&self) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| todo.completed).collect()
    }

    pub fn pending_todos(&self) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| !todo.completed).collect()
    }
}
