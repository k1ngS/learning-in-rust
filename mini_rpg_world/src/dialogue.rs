use serde::{Deserialize, Serialize};

pub enum DialogueChoice {
    Next,
    Option(usize),
    End,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DialogueStep {
    pub text: String,
    pub options: Vec<String>,
    pub next: Vec<Option<usize>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DialogueEngine {
    pub steps: Vec<DialogueStep>,
    pub current: usize,
}

impl DialogueEngine {
    pub fn new(steps: Vec<DialogueStep>) -> Self {
        Self { steps, current: 0 }
    }
    pub fn run(&mut self) {
        if self.steps.is_empty() {
            println!("No dialogue available.");
            return;
        }
        loop {
            let step = &self.steps[self.current];
            println!("\n{}", step.text);
            // Adicione lógica para opções e fluxo de diálogo
            break;
        }
    }
}
