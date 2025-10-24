mod player;
mod inventory;
mod skills;
mod quest;
mod town;
mod combat_log;
mod dialogue;
mod save_load;
mod game_state;
mod helper;

use crate::combat_log::combat_menu;
use crate::game_state::GameState;
use crate::helper::read_input;
use crate::inventory::inventory_menu;
use crate::quest::quest_menu;
use crate::skills::skills_menu;
use crate::town::town_menu;

fn main() {
    let mut state = GameState::new();
    println!("\n╔══════════════════════════════════╗");
    println!("║   WELCOME TO MINI RPG WORLD!    ║");
    println!("╚══════════════════════════════════╝");

    loop {
        println!("\n==== Mini RPG World ====");
        println!(
            "1. Player Status\n\
             2. Inventory\n\
             3. Quests\n\
             4. Town\n\
             5. Skills\n\
             6. Combat Arena\n\
             7. Combat Log\n\
             8. Dialogue\n\
             9. Save\n\
             10. Load\n\
             11. Exit"
        );

        match read_input().as_str() {
            "1" => state.player.display(),
            "2" => inventory_menu(&mut state),
            "3" => {
                println!("\n=== Active Quests ===");
                for quest in &state.quests {
                    quest.display();
                }
                quest_menu(&mut state);
            },
            "4" => town_menu(&mut state),
            "5" => skills_menu(&mut state),
            "6" => combat_menu(&mut state),
            "7" => state.combat_log.display(),
            "8" => state.dialogue_engine.run(),
            "9" => {
                save_load::save_game(&state, "save.json");
                println!("💾 Game saved!");
            },
            "10" => {
                if let Some(loaded) = save_load::load_game::<GameState>("save.json") {
                    state = loaded;
                    println!("📂 Game loaded successfully!");
                }
            },
            "11" => {
                println!("Thanks for playing! Goodbye! 👋");
                break;
            },
            _ => println!("Invalid option"),
        }
    }
}
