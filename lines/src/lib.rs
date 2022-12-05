use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub fn get_lines(filename: &str) -> Lines<BufReader<File>> {
    let input = File::open(filename).unwrap();
    BufReader::new(input).lines()
}
