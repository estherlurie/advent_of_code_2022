use lines::get_lines;
use std::collections::{HashMap, HashSet};

const INPUT_FILE: &str = "inputs/day_3.txt";

fn main() {
    let mut lines = get_lines(INPUT_FILE);

    let mut groups: Vec<Group> = Vec::new();

    while let (Some(Ok(bag_1)), Some(Ok(bag_2)), Some(Ok(bag_3))) =
        (lines.next(), lines.next(), lines.next())
    {
        groups.push(Group::new(bag_1, bag_2, bag_3));
    }

    let priorities = priority_map();

    let total = groups.iter().fold(0, |sum, group| {
        sum + *priorities.get(&group.badge()).unwrap()
    });

    println!("The total priority of all rucksacks is {}", total);
}

#[derive(Debug)]
struct Group {
    bag_1: String,
    bag_2: String,
    bag_3: String,
}
impl Group {
    pub fn new(bag_1: String, bag_2: String, bag_3: String) -> Group {
        Group {
            bag_1,
            bag_2,
            bag_3,
        }
    }

    pub fn badge(&self) -> char {
        let set: HashSet<char> = self.bag_1.chars().collect();
        let set: HashSet<char> = self.bag_2.chars().filter(|c| set.contains(c)).collect();
        let set: HashSet<char> = self.bag_3.chars().filter(|c| set.contains(c)).collect();
        set.into_iter().next().unwrap()
    }
}

fn priority_map() -> HashMap<char, usize> {
    let mut lookup = HashMap::new();
    (b'a'..=b'z')
        .map(char::from)
        .enumerate()
        .for_each(|(idx, c)| {
            lookup.insert(c, idx + 1);
        });
    (b'A'..=b'Z')
        .map(char::from)
        .enumerate()
        .for_each(|(idx, c)| {
            lookup.insert(c, idx + 27);
        });
    lookup
}
