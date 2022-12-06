use lines::get_lines;

const INPUT_FILE: &str = "inputs/day_5.txt";

fn main() {
    let mut lines = get_lines(INPUT_FILE);
    let mut crates = String::new();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.is_empty() {
            break;
        }
        crates.push_str(&format!("{}\n", line));
    }
    let mut cargo_bay = CargoBay::new(crates);

    lines.flatten().for_each(|line| {
        cargo_bay.move_cargo(MoveInstruction::new(line));
    });

    println!(
        "After reorganizing the cargo bay, the elves see: {}",
        cargo_bay.peek()
    );
}

#[derive(Debug)]
struct MoveInstruction {
    from: usize,
    to: usize,
    amount: usize,
}
impl MoveInstruction {
    pub fn new(line: String) -> MoveInstruction {
        // "move X from Y to Z"
        let mut split = line.split(' ');
        split.next();
        let amount = split.next().unwrap().parse::<usize>().unwrap();
        split.next();
        let from = split.next().unwrap().parse::<usize>().unwrap() - 1;
        split.next();
        let to = split.next().unwrap().parse::<usize>().unwrap() - 1;

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
