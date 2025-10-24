# 🦀 Rust Learning Journey

Welcome to my Rust learning repository! This collection represents my progressive journey through the Rust programming language, featuring hands-on projects that explore core concepts and practical applications.

## 📚 Learning Path

This repository follows a structured learning approach, starting with basic concepts and progressing to more complex applications:

### 🌱 **Fundamentals** (Day 1)
- **`main.rs`** - Hello World starting point
- **`variables/`** - Variable declaration, mutability, shadowing, and constants
- **`data_types/`** - Integer types, tuples, and type annotations
- **`guess_game/`** - Classic number guessing game with input handling and error management

### 🔧 **CLI Applications** (Day 2+)
- **`todo-cli-01/`** - Command-line todo application (Not started yet)
- **`character_name_generator/`** - RPG character name generator

### 🎮 **Game Development Projects** (Day 2-3)
- **`battle_simulator/`** - Turn-based combat system with character stats
- **`inventory_manager/`** - Item management system for games
- **`quest_tracker/`** - Quest system with completion tracking
- **`town_builder/`** - Town construction and resource management
- **`skill_system/`** - Character abilities and skill progression
- **`dialogue_engine/`** - Interactive conversation system
- **`combat_log/`** - Event logging and tracking system
- **`save_load_system/`** - Game state persistence with JSON serialization

### 🏰 **Capstone Project**
- **`mini_rpg_world/`** - Complete text-based RPG integrating all learned concepts

## 🎯 Key Learning Objectives

### Core Rust Concepts
- ✅ **Ownership & Borrowing** - Memory safety without garbage collection
- ✅ **Pattern Matching** - Using `match` expressions and error handling
- ✅ **Structs & Implementation** - Creating custom types and methods
- ✅ **Modules & Crates** - Code organization and external dependencies
- ✅ **Error Handling** - `Result` and `Option` types
- ✅ **Iterators & Closures** - Functional programming concepts
- ✅ **Traits** - Shared behavior and polymorphism

### Practical Skills
- 🔧 **CLI Development** - Building command-line applications
- 📁 **File I/O** - Reading from and writing to files
- 🌐 **JSON Serialization** - Data persistence with Serde
- 🎮 **Game Logic** - State management and user interaction
- 📦 **Project Structure** - Organizing larger Rust projects
- 🚀 **Cargo Workflow** - Building, testing, and managing dependencies

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Git

### Running Projects

Each project is self-contained with its own `Cargo.toml`:

```bash
# Clone the repository
git clone <your-repo-url>
cd learning

# Run any individual project
cd <project-name>
cargo run

# Build all projects
find . -name "Cargo.toml" -execdir cargo build \;
```

### Featured Projects to Try

1. **Start Simple**: `guess_game/` - Classic beginner project
2. **Learn CLI**: `todo-cli-01/` - Practical command-line tool
3. **Game Development**: `battle_simulator/` - Turn-based combat
4. **Full Experience**: `mini_rpg_world/` - Complete RPG with all features

## 📁 Project Structure

```
learning/
├── 📄 main.rs                     # Hello World
├── 🔤 variables/                  # Variable basics
├── 📊 data_types/                 # Type system
├── 🎲 guess_game/                 # Number guessing game
├── ✅ todo-cli-01/                # CLI todo app
├── 🎭 character_name_generator/   # Name generator
├── ⚔️  battle_simulator/          # Combat system
├── 🎒 inventory_manager/          # Item management
├── 📜 quest_tracker/              # Quest system
├── 🏘️  town_builder/              # Town building
├── 🔮 skill_system/               # Abilities & skills
├── 💬 dialogue_engine/            # Conversation system
├── 📋 combat_log/                 # Event logging
├── 💾 save_load_system/           # State persistence
└── 🎮 mini_rpg_world/             # Complete RPG
```

## 🛠️ Technologies & Dependencies

- **Language**: Rust 2021 Edition
- **External Crates**:
  - `rand` - Random number generation
  - `serde` + `serde_json` - Serialization
  - Standard library extensively used

## 📈 Learning Progress

- **Day 1**: Basic syntax, variables, data types, first program
- **Day 2**: CLI applications, game mechanics, data structures
- **Day 3**: Advanced features, modular design, complete project integration

## 🎓 Learning Resources

This repository was built following:
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
- Hands-on project-based learning approach

## 📜 License

This project is open source and available under the [MIT License](LICENSE).

---

**Happy Coding! 🦀**