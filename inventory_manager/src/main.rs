// Item struct
struct Item {
    name: String,
    quantity: u32,
}
// Inventory struct
struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    // Create a new, empty inventory
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    // Add an item to the inventory
    fn add_item(&mut self, name: &str, qty: u32) {
        for item in self.items.iter_mut() {
            if item.name == name {
                item.quantity += qty;
                return;
            }
        }
        self.items.push(Item {
            name: name.to_string(),
            quantity: qty,
        });
    }

    // Remove an item from the inventory
    fn remove_item(&mut self, name: &str, qty: u32) {
        // retain_mut is equals to filter in javascript
        self.items.retain_mut(|item| {
            if item.name == name {
                if item.quantity > qty {
                    item.quantity -= qty;
                    true
                } else {
                    false // remove if quantity is zero or less
                }
            } else {
                true // keep other items
            }
        });
    }

    // List all items in the inventory
    fn list_items(&self) {
        println!("Your inventory contains:");
        for item in &self.items {
            println!("{} x{}", item.name, item.quantity);
        }
    }
}

fn main() {
    let mut inventory = Inventory::new();

    // Add items to the inventory
    inventory.add_item("Health Potion", 10);
    inventory.add_item("Mana Potion", 5);
    inventory.add_item("Sword", 1);

    // List items in the inventory
    inventory.list_items();

    // Remove some items from the inventory
    inventory.remove_item("Health Potion", 3);
    inventory.remove_item("Mana Potion", 5);
    println!("After removing some items:");
    inventory.list_items();
}