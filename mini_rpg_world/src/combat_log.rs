use serde::{Deserialize, Serialize};
use crate::game_state::GameState;
use crate::player::Player;
use crate::read_input;

#[derive(Serialize, Deserialize, Debug)]
pub struct CombatLog {
    pub entries: Vec<String>,
    pub max_entries: usize,
}

impl CombatLog {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }
    pub fn log(&mut self, entry: &str) {
        if self.entries.len() == self.max_entries {
            self.entries.remove(0);
        }
        self.entries.push(entry.to_string());
    }
    pub fn display(&self) {
        println!("--- Combat Log ---");
        for entry in &self.entries {
            println!("{}", entry);
        }
        println!("------------------");
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

// COMBAT SYSTEM
pub fn combat_menu(state: &mut GameState) {
    println!("\n⚔️ === COMBAT ARENA === ⚔️");
    println!("A wild enemy appears!");

    let mut enemy = Player::new("Goblin Warrior", 50, 8, 3);

    loop {
        println!("\n{} HP: {} | {} HP: {}",
                 state.player.name, state.player.health,
                 enemy.name, enemy.health);

        println!("\n1. Attack");
        println!("2. Use Skill");
        println!("3. Use Potion");
        println!("4. Run Away");

        match read_input().as_str() {
            "1" => {
                state.player.attack(&mut enemy);
                state.combat_log.log(&format!("{} attacked {}", state.player.name, enemy.name));

                if !enemy.is_alive() {
                    println!("\n🎉 Victory! {} defeated!", enemy.name);
                    state.combat_log.log(&format!("Defeated {}", enemy.name));
                    state.current_town.gold += 25;
                    println!("💰 Earned 25 gold!");
                    break;
                }

                // Enemy turn
                enemy.attack(&mut state.player);
                state.combat_log.log(&format!("{} attacked {}", enemy.name, state.player.name));

                if !state.player.is_alive() {
                    println!("\n💀 Game Over! You were defeated...");
                    state.player.health = 100; // Respawn
                    state.combat_log.log("Player was defeated and respawned");
                    break;
                }
            },
            "2" => {
                if !state.skills.is_empty() {
                    state.skills[0].use_skill(&mut state.player, &mut state.combat_log);
                    let damage = state.skills[0].power as i32;
                    enemy.health -= damage;
                    println!("💥 Dealt {} damage with skill!", damage);

                    if !enemy.is_alive() {
                        println!("\n🎉 Victory! {} defeated!", enemy.name);
                        state.combat_log.log(&format!("Defeated {} with skill", enemy.name));
                        state.current_town.gold += 25;
                        println!("💰 Earned 25 gold!");
                        break;
                    }
                } else {
                    println!("No skills available!");
                }
            },
            "3" => {
                if let Some(item) = state.inventory.items.iter_mut().find(|i| i.name == "Health Potion" && i.quantity > 0) {
                    item.quantity -= 1;
                    state.player.health = (state.player.health + 30).min(100);
                    println!("💚 Healed for 30 HP!");
                }
            },
            "4" => {
                println!("🏃 You fled from battle!");
                state.combat_log.log("Fled from combat");
                break;
            },
            _ => println!("Invalid option"),
        }
    }
}
