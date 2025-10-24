# 🎮 Mini RPG World

A modular text-based RPG built in Rust as a learning project.

## 🌟 Features

- ⚔️ **Player System** - Character stats and progression
- 🎒 **Inventory Management** - Collect and manage items
- 📜 **Quest Tracking** - Start and complete quests
- 🏘️ **Town Building** - Construct buildings and manage resources
- 🔮 **Skill System** - Learn and use abilities
- 📖 **Dialogue Engine** - Interactive conversations
- 💾 **Save/Load** - Persistent game state
- 📋 **Combat Log** - Track all game events

## 🚀 Quick Start

1. Clone the repository:
   ```bash
   git clone
   cd mini-rpg-world
   ```
   
2. Build the project:
   ```bash
   cargo build
   ```
   
3. Run the game:
   ```bash
    cargo run
    ```

## 🎯 How to Play

1. Choose options from the main menu (1-10)
2. Manage your character, inventory, and quests
3. Build your town and develop skills
4. Save your progress anytime (option 8)
5. Load your save file (option 9)

## 📚 Learning Outcomes

This project demonstrates:
- Rust ownership and borrowing
- Modular architecture
- State management
- Serialization with Serde
- File I/O operations
- User interaction loops

## 🛠️ Technologies

- **Language:** Rust
- **Serialization:** Serde + serde_json
- **Build System:** Cargo

## 📦 Project Structure

- `src/` - Source code
  - `player.rs` - Player management
  - `inventory.rs` - Inventory system
  - `quests.rs` - Quest tracking
  - `town.rs` - Town building
  - `skills.rs` - Skill system
  - `dialogue.rs` - Dialogue engine
  - `combat_log.rs` - Combat logging
  - `main.rs` - Entry point
- `save.json` - Example save file
- `Cargo.toml` - Project configuration

## 🎓 Author

Created as a Rust learning project by @k1ngS

## 📄 License

MIT License