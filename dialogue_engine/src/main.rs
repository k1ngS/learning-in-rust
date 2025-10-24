use std::io;

// Different possible choices in a dialogue
enum DialogueChoice {
    Next,
    Option(usize), // For branching dialogues with multiple options
    End,
}

// Dialogue step: can show text, choices and link to next step
struct DialogueStep {
    text: String,
    options: Vec<String>,
    next: Vec<Option<usize>>, // Indices of next steps
}

// Basic dialogue engine struct
struct DialogueEngine {
    steps: Vec<DialogueStep>,
    current: usize,
}

impl DialogueEngine {
    // Constructor
    fn new(steps: Vec<DialogueStep>) -> Self {
        Self { steps, current: 0 }
    }

    fn run(&mut self) {
        loop {
            let step = &self.steps[self.current];
            println!("\n{}", step.text);

            if step.options.is_empty() {
                println!("\nPress Enter to continue...");
                let _ = io::stdin().read_line(&mut String::new());
                self.current = match step.next.get(0).and_then(|opt| *opt) {
                    Some(next_index) => next_index,
                    None => {
                        println!("Dialogue ended.");
                        break;
                    }
                };
            } else {
                for (i, option) in step.options.iter().enumerate() {
                    println!("{}. {}", i + 1, option);
                }
                let choice = get_player_choice(step.options.len());
                match choice {
                    DialogueChoice::Option(n) => {
                        if let Some(Some(next_index)) = step.next.get(n - 1) {
                            self.current = *next_index;
                        } else {
                            println!("Dialogue ended.");
                            break;
                        }
                    }
                    DialogueChoice::End => {
                        println!("Dialogue ended.");
                        break;
                    }
                    DialogueChoice::Next => {}
                }
            }
        }
    }
}

fn get_player_choice(num_options: usize) -> DialogueChoice {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let choice = input.trim().parse::<usize>().unwrap_or(0);
    match choice {
        0 => DialogueChoice::Next,
        n if n >= 1 && n <= num_options => DialogueChoice::Option(n),
        _ => DialogueChoice::End,
    }
}

fn main() {
    let steps = vec![
        DialogueStep {
            text: String::from("Welcome to the dialogue engine! What would you like to do?"),
            options: vec![
                String::from("Start a new adventure"),
                String::from("Load a saved game"),
                String::from("Exit"),
            ],
            next: vec![Some(1), Some(2), None],
        },
        DialogueStep {
            text: String::from("You embark on a new adventure! Good luck!"),
            options: vec![],
            next: vec![None],
        },
        DialogueStep {
            text: String::from("Loading saved game..."),
            options: vec![],
            next: vec![None],
        },
    ];

    let mut engine = DialogueEngine::new(steps);
    engine.run();
}
