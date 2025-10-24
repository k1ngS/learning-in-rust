// struct to represent the combat log
struct CombatLog {
    entries: Vec<String>,
    max_entries: usize,
}

impl CombatLog {
    // Constructor
    fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }

    // Add a new entry
    fn log(&mut self, entry: &str) {
        if self.entries.len() == self.max_entries {
            self.entries.remove(0); // Remove the oldest entry
        }
        self.entries.push(entry.to_string());
    }

    // Display the combat log
    fn display(&self) {
        println!("--- Combat Log ---");
        for entry in &self.entries {
            println!("{entry}");
        }
        println!("------------------");
    }

    // Clear the combat log
    fn clear(&mut self) {
        self.entries.clear();
    }
}

fn main() {
    let mut log = CombatLog::new(5);

    // Simulate some combat events
    log.log("Hero attacks Goblin for 10 damage.");
    log.log("Goblin attacks Hero for 5 damage.");
    log.log("Hero casts Fireball on Goblin for 20 damage.");
    log.log("Goblin is defeated.");
    log.log("Hero finds a health potion.");

    log.display();

    // Add more events to excced max entries (oldest events disappear)
    log.log("Hero drinks health potion and heals 15 HP.");
    log.log("Hero encounters a Dragon.");

    println!("\nAfter more events:");
    log.display();

    // Clear the log
    log.clear();
    println!("\nCombat log cleared.");
    log.display()
}
