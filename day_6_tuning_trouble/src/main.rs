use lines::get_lines;
use std::collections::VecDeque;

const INPUT_FILE: &str = "inputs/day_6.txt";

fn main() {
    let lines = get_lines(INPUT_FILE);

    let mut buf = VecDeque::new();
    for (idx, c) in lines.flatten().next().unwrap().chars().enumerate() {
        if buf.len() == 14 {
            buf.pop_front();
            buf.push_back(c);
            let mut has_duplicate = false;
            for i in 0..buf.len() {
                for j in i + 1..buf.len() {
                    if buf[i] == buf[j] {
                        has_duplicate = true;
                        break;
                    }
                }
            }
            if !has_duplicate {
                println!("Found the message marker at index: {}", idx + 1);
                break;
            }
        } else {
            buf.push_back(c);
        }
    }
}
