use serde::{Deserialize, Serialize};
use crate::combat_log::CombatLog;
use crate::game_state::GameState;
use crate::read_input;

#[derive(Serialize, Deserialize, Debug)]
pub enum QuestStatus {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quest {
    pub name: String,
    pub description: String,
    pub status: QuestStatus,
}

impl Quest {
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            name: title.to_string(),
            description: description.to_string(),
            status: QuestStatus::NotStarted,
        }
    }
    pub fn start(&mut self) {
        if let QuestStatus::NotStarted = self.status {
            self.status = QuestStatus::InProgress;
        }
    }
    pub fn complete(&mut self) {
        if let QuestStatus::InProgress = self.status {
            self.status = QuestStatus::Completed;
        }
    }

    pub fn display(&self) {
        let status = match self.status {
            QuestStatus::NotStarted => "⭕ Not Started",
            QuestStatus::InProgress => "🔄 In Progress",
            QuestStatus::Completed => "✅ Completed",
        };
        println!("[{}] {}", status, self.name);
        println!("   📜 {}", self.description);
    }
}

// Quest Menu
pub fn quest_menu(state: &mut GameState) {
    loop {
        println!("\n=== Quest Menu ===");
        for (i, quest) in state.quests.iter().enumerate() {
            print!("{}. ", i + 1);
            quest.display();
        }
        println!("0. Back");

        match read_input().as_str() {
            "0" => break,
            n => {
                if let Ok(idx) = n.parse::<usize>() {
                    if idx > 0 && idx <= state.quests.len() {
                        quest_action(&mut state.quests[idx - 1], &mut state.combat_log);
                    }
                }
            }
        }
    }
}

fn quest_action(quest: &mut Quest, log: &mut CombatLog) {
    println!("\n1. Start Quest\n2. Complete Quest\n0. Back");
    match read_input().as_str() {
        "1" => {
            quest.start();
            log.log(&format!("Started quest: {}", quest.name));
            println!("✅ Quest started!");
        },
        "2" => {
            quest.complete();
            log.log(&format!("Completed quest: {}", quest.name));
            println!("🎉 Quest completed!");
        },
        _ => {}
    }
}
