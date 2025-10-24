use serde::{Serialize, Deserialize};
use std::fs;

pub fn  save_game<T: Serialize>(data: &T, filename: &str) {
    let json = serde_json::to_string_pretty(data).expect("Failed to serialize");
    fs::write(filename, json).expect("Failed to write file");
    println!("Game saved to {}", filename);
}

pub fn load_game<T: for<'de> Deserialize<'de>>(filename: &str) -> Option<T> {
    match fs::read_to_string(filename) {
        Ok(content) => {
            let data = serde_json::from_str(&content).expect("Failed to deserialize");
            Some(data)
        }
        Err(_) => None,
    }
}
