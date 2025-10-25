// Enum to represent building types
#[derive(Clone, Copy)]
enum BuildingType {
    House,
    Farm,
    Barracks,
    Market,
    Temple,
}

// Struct to represent a building
struct Building {
    building_type: BuildingType,
    name: String,
}

// Struct to represent the town resources & buildings
struct Town {
    name: String,
    gold: u32,
    wood: u32,
    food: u32,
    buildings: Vec<Building>,
}

impl Town {
    // Create a new town
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            gold: 100,
            wood: 100,
            food: 100,
            buildings: Vec::new(),
        }
    }

    // Build a new building, spending resources
    fn build(&mut self, building_type: BuildingType, building_name: &str) {
        // Resource costs for each building type
        let (gold_cost, wood_cost, food_cost) = match building_type {
            BuildingType::House => (10, 15, 0),
            BuildingType::Farm => (15, 10, 20),
            BuildingType::Barracks => (30, 40, 10),
            BuildingType::Market => (25, 20, 5),
            BuildingType::Temple => (50, 30, 15),
        };

        if self.gold >= gold_cost && self.wood >= wood_cost && self.food >= food_cost {
            self.gold -= gold_cost;
            self.wood -= wood_cost;
            self.food -= food_cost;
            self.buildings.push(Building {
                building_type: building_type.clone(),
                name: building_name.to_string(),
            });
            println!("Built a {} named '{}'.", Self::building_type_name(&building_type), building_name);
        } else {
            println!("Not enough resources to build a {}.", Self::building_type_name(&building_type));
        }
    }

    // List all buildings in the town
    fn list_buildings(&self) {
        println!("Buildings in {}:", self.name);
        for building in &self.buildings {
            println!("- {}: {}", Self::building_type_name(&building.building_type), building.name);
        }
    }

    // Helper function to get building type name
    fn building_type_name(building_type: &BuildingType) -> &'static str {
        match building_type {
            BuildingType::House => "House",
            BuildingType::Farm => "Farm",
            BuildingType::Barracks => "Barracks",
            BuildingType::Market => "Market",
            BuildingType::Temple => "Temple",
        }
    }

    // Display current resources
    fn display_resources(&self) {
        println!(
            "Resources - Gold: {}, Wood: {}, Food: {}",
            self.gold, self.wood, self.food
        );
    }
}

fn main() {
    let mut town = Town::new("Japanese Village");

    // Build a city step by step
    town.display_resources();
    town.build(BuildingType::House, "Sakura House");
    town.build(BuildingType::Farm, "Rice Farm");
    town.build(BuildingType::Barracks, "Samurai Barracks");
    town.build(BuildingType::Market, "Ninja Market");
    town.build(BuildingType::Temple, "Zen Temple");
    town.display_resources();

    town.list_buildings();
}
