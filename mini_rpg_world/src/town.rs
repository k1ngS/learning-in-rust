use std::io;
use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::combat_log::CombatLog;
use crate::game_state::GameState;
use crate::read_input;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum BuildingType {
    House,
    Farm,
    Barracks,
    Market,
    Temple,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Building {
    pub building_type: BuildingType,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Town {
    pub name: String,
    pub gold: u32,
    pub wood: u32,
    pub food: u32,
    pub buildings: Vec<Building>,
}

impl Town {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            gold: 100,
            wood: 100,
            food: 100,
            buildings: Vec::new(),
        }
    }
    pub fn build(&mut self, building_type: BuildingType, building_name: &str) {
        self.buildings.push(Building {
            building_type,
            name: building_name.to_string(),
        });
    }
    pub fn show_buildings(&self) {
        println!("Buildings in {}:", self.name);
        for b in &self.buildings {
            println!("- {:?}: {}", b.building_type, b.name);
        }
    }

    pub fn display_resources(&self) {
        println!("\n=== {} ===", self.name);
        println!("💰 Gold: {} | 🪵 Wood: {} | 🌾 Food: {}",
                 self.gold, self.wood, self.food);
    }
}

// Town Builder Menu
pub fn town_menu(state: &mut GameState) {
    loop {
        state.current_town.display_resources();
        state.current_town.show_buildings();

        println!("\n=== Town Menu ===");
        println!("1. Build Farm (Cost: 50 gold, 20 wood)");
        println!("2. Build Barracks (Cost: 100 gold, 50 wood)");
        println!("3. Build Market (Cost: 75 gold, 30 wood)");
        println!("4. Build Temple (Cost: 150 gold, 40 wood)");
        println!("5. Build House (Cost: 40 gold, 25 wood)");
        println!("0. Back");

        match read_input().as_str() {
            "0" => break,
            "1" => build_and_pay(&mut state.current_town, &mut state.combat_log, BuildingType::Farm, 50, 20),
            "2" => build_and_pay(&mut state.current_town, &mut state.combat_log, BuildingType::Barracks, 100, 50),
            "3" => build_and_pay(&mut state.current_town, &mut state.combat_log, BuildingType::Market, 75, 30),
            "4" => build_and_pay(&mut state.current_town, &mut state.combat_log, BuildingType::Temple, 150, 40),
            "5" => build_and_pay(&mut state.current_town, &mut state.combat_log, BuildingType::House, 40, 25),
            _ => println!("Invalid option"),
        }
    }
}

pub fn build_and_pay(town: &mut crate::town::Town, log: &mut CombatLog, building_type: BuildingType, gold_cost: u32, wood_cost: u32) {
    if town.gold >= gold_cost && town.wood >= wood_cost {
        print!("Enter building name: ");
        io::stdout().flush().unwrap();
        let name = read_input();
        town.gold -= gold_cost;
        town.wood -= wood_cost;
        town.build(building_type, &name);
        log.log(&format!("Built a {:?} for {} gold and {} wood", building_type, gold_cost, wood_cost));
    } else {
        println!("❌ Not enough resources!");
    }
}