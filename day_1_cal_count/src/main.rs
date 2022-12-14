use lines::get_lines;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INPUT_FILE: &str = "inputs/day_1.txt";
const TOP_K_ELVES: usize = 3;

#[derive(Default)]
pub struct Calories {
    pub current: i32,
    pub heap: BinaryHeap<Reverse<i32>>,
}

fn main() {
    let lines = get_lines(INPUT_FILE);

    let cals = lines.flatten().fold(
        Calories {
            current: 0,
            heap: BinaryHeap::new(),
        },
        |cals, line| {
            if line.is_empty() {
                let mut heap = cals.heap.clone();
                heap.push(Reverse(cals.current));
                while heap.len() > TOP_K_ELVES {
                    heap.pop();
                }
                Calories { current: 0, heap }
            } else {
                Calories {
                    current: cals.current + line.parse::<i32>().unwrap(),
                    heap: cals.heap,
                }
            }
        },
    );

    println!(
        "Max calories: {}",
        cals.heap.iter().fold(0, |sum, max| {
            let Reverse(x) = max;
            sum + x
        })
    );
}
