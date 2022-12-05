use lines::get_lines;

const INPUT_FILE: &str = "inputs/day_2.txt";
fn main() {
    let lines = get_lines(INPUT_FILE);

    let total = lines.flatten().fold(0, |score, line| {
        let (me, opponent) = parse_line(line);
        score + me.play(opponent)
    });

    println!("At the end, I scored: {}", total);
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn play(&self, opponent: Move) -> i32 {
        let rock = 1;
        let paper = 2;
        let scissors = 3;
        let lose = 0;
        let draw = 3;
        let win = 6;

        match (self, opponent) {
            (Move::Rock, Move::Rock) => rock + draw,
            (Move::Rock, Move::Paper) => rock + lose,
            (Move::Rock, Move::Scissors) => rock + win,
            (Move::Paper, Move::Rock) => paper + win,
            (Move::Paper, Move::Paper) => paper + draw,
            (Move::Paper, Move::Scissors) => paper + lose,
            (Move::Scissors, Move::Rock) => scissors + lose,
            (Move::Scissors, Move::Paper) => scissors + win,
            (Move::Scissors, Move::Scissors) => scissors + draw,
        }
    }
}

fn parse_line(line: String) -> (Move, Move) {
    let mut split = line.split(' ');
    let opponent = match split.next().unwrap() {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        _ => panic!("Expected A, B, or C"),
    };
    let me = match (split.next().unwrap(), &opponent) {
        ("X", Move::Rock) => Move::Scissors,
        ("X", Move::Paper) => Move::Rock,
        ("X", Move::Scissors) => Move::Paper,
        ("Y", Move::Rock) => Move::Rock,
        ("Y", Move::Paper) => Move::Paper,
        ("Y", Move::Scissors) => Move::Scissors,
        ("Z", Move::Rock) => Move::Paper,
        ("Z", Move::Paper) => Move::Scissors,
        ("Z", Move::Scissors) => Move::Rock,
        _ => panic!("Expected X, Y, or Z"),
    };

    (me, opponent)
}
