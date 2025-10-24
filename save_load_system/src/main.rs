use serde::{Serialize, Deserialize};
use std::fs;

// Example game data struct
#[derive(Serialize, Deserialize, Debug)]
struct GameData {
    player_name: String,
    level: u32,
    hp: u32,
    items: Vec<String>,
    gold: u32,
}

fn save_game(data: &GameData, filename: &str) {
    let json = serde_json::to_string_pretty(data).expect("Failed to serialize game data");
    fs::write(filename, json).expect("Failed to write game data to file");
    println!("Wrote game data to '{}'", filename);
}

fn load_game(filename: &str) -> Option<GameData> {
    match fs::read_to_string(filename) {
        Ok(content) => {
            let data: GameData = serde_json::from_str(&content).expect("Failed to deserialize game data");
            println!("Loaded game data from '{}'", filename);
            Some(data)
        }
        Err(_) => {
            println!("No save file found at '{}'", filename);
            None
        }
    }
}

fn main() {
    // Example data
    let data = GameData {
        player_name: "Hero".to_string(),
        level: 5,
        hp: 100,
        items: vec!["Sword".to_string(), "Shield".to_string()],
        gold: 250,
    };

    // Save
    save_game(&data, "savegame.json");

    // Load
    if let Some(loaded_data) = load_game("savegame.json") {
        println!("Loaded Data: {:?}", loaded_data);
    }
}
