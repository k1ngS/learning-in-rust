use serde::{Deserialize, Serialize};
use crate::player::Player;
use crate::inventory::Inventory;
use crate::quest::Quest;
use crate::town::Town;
use crate::skills::Skill;
use crate::combat_log::CombatLog;
use crate::dialogue::{DialogueEngine, DialogueStep};

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub player: Player,
    pub inventory: Inventory,
    pub quests: Vec<Quest>,
    pub current_town: Town,
    pub skills: Vec<Skill>,
    pub combat_log: CombatLog,
    pub dialogue_engine: DialogueEngine,
}

impl GameState {
    pub fn new() -> Self {
        let mut state = Self {
            player: Player::new("Hero", 100, 10, 5),
            inventory: Inventory::new(),
            quests: vec![],
            current_town: Town::new("Starting Village"),
            skills: vec![],
            combat_log: CombatLog::new(10),
            dialogue_engine: DialogueEngine::new(vec![
                DialogueStep {
                    text: "Welcome to the RPG World! Are you ready for adventure?".to_string(),
                    options: vec!["Yes!".to_string(), "Tell me more".to_string()],
                    next: vec![None, None],
                }
            ]),
        };

        // Add initial combat log entries
        state.combat_log.log("⚔️ Welcome to your adventure!");
        state.combat_log.log("🏘️ You have entered the Starting Village.");
        state.combat_log.log("📦 Your inventory has been stocked with starter items.");

        // Add initial items to inventory
        state.inventory.add_item("Health Potion", 3);
        state.inventory.add_item("Sword", 1);
        state.inventory.add_item("Gold Coin", 100);

        // Add initial quests
        state.quests.push(Quest::new(
            "Find the Lost Artifact",
            "Search the ancient ruins for a mysterious artifact"
        ));
        state.quests.push(Quest::new(
            "Defeat the Goblin Chief",
            "Clear the goblin camp and defeat their leader"
        ));
        state.quests.push(Quest::new(
            "Gather Herbs",
            "Collect 10 medicinal herbs from the forest"
        ));
        state.quests.push(Quest::new(
            "Train with the Master",
            "Complete combat training at the barracks"
        ));

        // Add initial skills
        state.skills.push(Skill {
            name: "Power Strike".to_string(),
            skill_type: crate::skills::SkillType::Attack,
            power: 15,
            required_level: 1,
        });
        state.skills.push(Skill {
            name: "Shield Block".to_string(),
            skill_type: crate::skills::SkillType::Defense,
            power: 10,
            required_level: 1,
        });

        // Build initial town buildings
        state.current_town.build(crate::town::BuildingType::Farm, "Old Mill Farm");
        state.current_town.build(crate::town::BuildingType::Market, "Central Market");

        state
    }
}
