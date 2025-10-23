struct Character {
    name: String,
    health: i32,
    attack: i32,
    defense: i32,
}

impl Character {
    // Constructor for character
    fn new(name: &str, health: i32, attack: i32, defense: i32) -> Self {
        Self {
            name: name.to_string(),
            health,
            attack,
            defense,
        }
    }

    // Attack another character
    fn attack(&self, defender: &mut Character) {
        let damage = (self.attack - defender.defense).max(1);
        defender.health -= damage;
        println!(
            "{} attacks {} for {} damage!",
            self.name, defender.name, damage
        );
    }

    // Check if character is alive
    fn is_alive(&self) -> bool {
        self.health > 0
    }
}

fn main() {
    // Create two characters
    let mut hero = Character::new("Hero", 30, 8, 3);
    let mut monster = Character::new("Monster", 25, 6, 2);

    // Battle loop
    let mut turn = 1;
    while hero.is_alive() && monster.is_alive() {
        println!("-- Turn {} --", turn);
        if turn % 2 == 1 {
            hero.attack(&mut monster);
        } else {
            monster.attack(&mut hero);
        }
        println!("{}: {} HP | {}: {} HP", hero.name, hero.health, monster.name, monster.health);
        turn += 1;
    }

    // End game result
    if hero.is_alive() {
        println!("The Hero wins!");
    } else {
        println!("The Monster wins!");
    }
}
