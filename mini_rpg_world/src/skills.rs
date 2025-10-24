use serde::{Deserialize, Serialize};
use crate::game_state::GameState;
use crate::read_input;

#[derive(Serialize, Deserialize, Debug)]
pub enum SkillType {
    Attack,
    Defense,
    Support,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Skill {
    pub name: String,
    pub skill_type: SkillType,
    pub power: u32,
    pub required_level: u32,
}

impl Skill {
    pub fn use_skill(&self, player: &mut super::player::Player, log: &mut super::combat_log::CombatLog) {
        log.log(&format!("{} uses {} [{:?}]", player.name, self.name, self.skill_type));
        if let SkillType::Attack = self.skill_type {
            println!("Attack skill used! (power {})", self.power);
        }
    }

    pub fn display(&self) {
        let icon = match self.skill_type {
            SkillType::Attack => "⚔️",
            SkillType::Defense => "🛡️",
            SkillType::Support => "💚",
        };
        println!("{} {} (Power: {}, Req. Level: {})",
                 icon, self.name, self.power, self.required_level);
    }
}

// Skills Menu
pub fn skills_menu(state: &mut GameState) {
    loop {
        println!("\n=== Skills Menu ===");
        if state.skills.is_empty() {
            println!("No skills available.");
            return;
        }

        for (i, skill) in state.skills.iter().enumerate() {
            print!("{}. ", i + 1);
            skill.display();
        }
        println!("0. Back");

        match read_input().as_str() {
            "0" => break,
            n => {
                if let Ok(idx) = n.parse::<usize>() {
                    if idx > 0 && idx <= state.skills.len() {
                        let skill = &state.skills[idx - 1];
                        skill.use_skill(&mut state.player, &mut state.combat_log);
                        println!("✨ Used skill: {}", skill.name);
                    }
                }
            }
        }
    }
}