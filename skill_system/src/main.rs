// Enum to represent different types
#[derive(Debug)]
enum SkillType {
    Attack,
    Defense,
    Support,
}

// Struct to represent a skill with a name and type
struct Skill {
    name: String,
    skill_type: SkillType,
    power: u32,
    required_level: u32,
}

// Struct to represent a character with skills
struct Character {
    name: String,
    level: u32,
    skills: Vec<Skill>,
}

impl Character {
    // Constructor
    fn new(name: &str, level: u32) -> Self {
        Self {
            name: name.to_string(),
            level,
            skills: Vec::new(),
        }
    }

    // Method to learn a new skill
    fn learn_skill(&mut self, skill: Skill) {
        if self.level >= skill.required_level {
            self.skills.push(skill);
            println!("{} learned a new skill!", self.name);
        } else {
            println!("{} is not high enough level to learn {}.", self.name, skill.name);
        }
    }

    // Method to use a skill
    fn use_skill(&self, skill_name: &str) {
        match self.skills.iter().find(|skill| skill.name == skill_name) {
            Some(skill) => {
                println!(
                    "{} uses {} ({:?}) with power {}!",
                    self.name, skill.name, skill.skill_type, skill.power
                );
            }
            None => {
                println!("{} does not know the skill {}.", self.name, skill_name);
            }
        }
    }

    // Method to list all learned skills
    fn list_skills(&self) {
        println!("{} skills available.", self.skills.len());
        for skill in &self.skills {
            println!(
                "- {} ({:?}): Power {}, Required Level {}",
                skill.name, skill.skill_type, skill.power, skill.required_level
            );
        }
    }
}

fn main() {
    // Sample skills
    let attack = Skill {
        name: "Sword Slash".to_string(),
        skill_type: SkillType::Attack,
        power: 50,
        required_level: 1,
    };
    let heal = Skill {
        name: "Heal".to_string(),
        skill_type: SkillType::Support,
        power: 30,
        required_level: 2,
    };
    let fireball = Skill {
        name: "Fireball".to_string(),
        skill_type: SkillType::Attack,
        power: 70,
        required_level: 3,
    };

    // Create a character
    let mut hero = Character::new("Hero", 2);

    // Learn skills
    hero.learn_skill(attack);
    hero.learn_skill(heal);
    hero.learn_skill(fireball); // Should fail due to level

    // List learned skills
    hero.list_skills();

    // Use skills
    hero.use_skill("Sword Slash");
    hero.use_skill("Fireball"); // Should fail as not learned
}
