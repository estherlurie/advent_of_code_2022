use lines::get_lines;
use std::fs::File;
use std::io::{BufReader, Lines};

const INPUT_FILE: &str = "inputs/day_5.txt";

fn main() {
    let mut lines = get_lines(INPUT_FILE);
    let crates = get_diagram(&mut lines);
    let mut cargo_bay = CargoBay::new(crates);
    lines.flatten().for_each(|line| {
        cargo_bay.move_cargo(MoveInstruction::new(line));
    });

    println!(
        "After reorganizing the cargo bay, the elves see: {}",
        cargo_bay.peek()
    );
}

fn get_diagram(lines: &mut Lines<BufReader<File>>) -> String {
    let mut crates = String::new();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.is_empty() {
            break;
        }
        crates.push_str(&format!("{}\n", line));
    }
    crates
}

#[derive(Debug)]
struct MoveInstruction {
    from: usize,
    to: usize,
    amount: usize,
}
impl MoveInstruction {
    pub fn new(line: String) -> MoveInstruction {
        // "move <amount> from <from> to <to>"
        let mut split = line.split(' ');
        let amount = split.nth(1).unwrap().parse::<usize>().unwrap();
        let from = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        MoveInstruction { from, to, amount }
    }
}

#[derive(Debug)]
struct CargoBay {
    hold: Vec<Vec<char>>,
}
impl CargoBay {
    pub fn new(diagram: String) -> CargoBay {
        let size = (diagram.lines().peekable().peek().unwrap().len() + 1) / 4;
        let mut cargo_bay = CargoBay {
            hold: vec![Vec::new(); size],
        };

        diagram.lines().for_each(|line| {
            let mut store = false;
            // each bay is represented by "[A] ", except the final one
            // so, when we have idx % 4 == 0, we know the next char
            // is what we want to store
            line.chars().enumerate().for_each(|(idx, c)| {
                if idx % 4 == 0 {
                    store = true;
                } else if store && c.is_alphabetic() {
                    cargo_bay.store((idx - 1) / 4, c);
                    store = false;
                }
            });
        });

        cargo_bay
    }

    pub fn store(&mut self, hold_idx: usize, cargo: char) {
        self.hold[hold_idx].insert(0, cargo);
    }

    pub fn move_cargo(&mut self, instruction: MoveInstruction) {
        let mut cargo = Vec::new();
        (0..instruction.amount).for_each(|_| {
            cargo.insert(0, self.hold[instruction.from].pop().unwrap());
        });
        self.hold[instruction.to].append(&mut cargo);
    }

    pub fn peek(&self) -> String {
        self.hold.iter().fold(String::new(), |mut word, bay| {
            word.push(*bay.last().unwrap_or(&' '));
            word
        })
    }
}
