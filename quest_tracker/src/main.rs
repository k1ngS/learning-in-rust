// TODO: Add a new features
// User can choose to start or complete quests
// Add a reward system for completed quests and requirements for starting quests
// Save and load quest progress from a file

// Enum to repressent different quest statuses
enum QuestStatus {
    NotStarted,
    InProgress,
    Completed,
}

// Struct to represent a quest
struct Quest {
    name: String,
    description: String,
    status: QuestStatus,
}

impl Quest {
    // Constructor
    fn new(title: &str, description: &str) -> Self {
        Self {
            name: title.to_string(),
            description: description.to_string(),
            status: QuestStatus::NotStarted,
        }
    }

    // Start the quest
    fn start(&mut self) {
        if let QuestStatus::NotStarted = self.status {
            self.status = QuestStatus::InProgress;
        }
    }

    // Complete the quest
    fn complete(&mut self) {
        if let QuestStatus::InProgress = self.status {
            self.status = QuestStatus::Completed;
        }
    }

    // Display quest details
    fn display(&self) {
        match self.status {
            QuestStatus::NotStarted => println!("Quest: {} - Not Started\nDescription: {}", self.name, self.description),
            QuestStatus::InProgress => println!("Quest: {} - In Progress\nDescription: {}", self.name, self.description),
            QuestStatus::Completed => println!("Quest: {} - Completed\nDescription: {}", self.name, self.description),
        }
    }
}

fn main() {
    // List of quests
    let mut quests = vec![
        Quest::new("Find the Lost Sword", "Retrieve the legendary sword from the ancient ruins."),
        Quest::new("Rescue the Villagers", "Save the villagers captured by bandits."),
        Quest::new("Defeat the Dragon", "Slay the dragon terrorizing the kingdom."),
        Quest::new("Collect Herbs", "Gather medicinal herbs for the village healer."),
        Quest::new("Explore the Cave", "Investigate the mysterious cave near the village.")
    ];

    // Start and complete some quests
    quests[0].start();
    quests[0].complete();
    quests[1].start();
    quests[3].start();

    // Display all quests
    println!("Quest Status Tracker:");
    for quest in &quests {
        quest.display();
    }
}
