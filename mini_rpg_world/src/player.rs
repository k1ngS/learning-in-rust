use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub health: i32,
    pub attack: i32,
    pub defense: i32,
}

impl Player {
    pub fn new(name: &str, health: i32, attack: i32, defense: i32) -> Self {
        Self {
            name: name.to_string(),
            health,
            attack,
            defense,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn attack(&self, defender: &mut Player) {
        let damage = (self.attack - defender.defense).max(1);
        defender.health -= damage;
        println!("{} attacks {} for {} damage!", self.name, defender.name, damage);
    }

    pub fn display(&self) {
        println!("\n╔════════════════════════════╗");
        println!("║      PLAYER STATUS         ║");
        println!("╠════════════════════════════╣");
        println!("║ Name:    {:17}║", self.name);
        println!("║ Health:  ❤️  {:3}/100         ║", self.health);
        println!("║ Attack:  ⚔️  {:3}             ║", self.attack);
        println!("║ Defense: 🛡️  {:3}             ║", self.defense);
        println!("╚════════════════════════════╝");
    }
}
