use std::io;
use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::game_state::GameState;
use crate::read_input;

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn add_item(&mut self, name: &str, qty: u32) {
        for item in self.items.iter_mut() {
            if item.name == name {
                item.quantity += qty;
                return;
            }
        }
        self.items.push(Item {
            name: name.to_string(),
            quantity: qty,
        });
    }
    pub fn show_items(&self) {
        println!("Inventory:");
        for item in &self.items {
            println!("{} x{}", item.name, item.quantity);
        }
    }
}

// Inventory Menu
pub fn inventory_menu(state: &mut GameState) {
    loop {
        state.inventory.show_items();
        println!("\n=== Inventory Menu ===");
        println!("1. Add Item");
        println!("2. Use Health Potion");
        println!("0. Back");

        match read_input().as_str() {
            "0" => break,
            "1" => {
                print!("Item name: ");
                io::stdout().flush().unwrap();
                let name = read_input();
                print!("Quantity: ");
                io::stdout().flush().unwrap();
                if let Ok(qty) = read_input().parse::<u32>() {
                    state.inventory.add_item(&name, qty);
                    state.combat_log.log(&format!("Added {} x{} to inventory", name, qty));
                    println!("✅ Item added!");
                }
            },
            "2" => {
                if state.inventory.items.iter().any(|item| item.name == "Health Potion" && item.quantity > 0) {
                    if let Some(item) = state.inventory.items.iter_mut().find(|i| i.name == "Health Potion") {
                        item.quantity -= 1;
                        state.player.health = (state.player.health + 30).min(100);
                        state.combat_log.log("Used Health Potion (+30 HP)");
                        println!("💚 Health restored! Current HP: {}", state.player.health);
                    }
                } else {
                    println!("❌ No Health Potions available!");
                }
            },
            _ => println!("Invalid option"),
        }
    }
}
