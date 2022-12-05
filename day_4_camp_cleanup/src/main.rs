use lines::get_lines;

const INPUT_FILE: &str = "inputs/day_4.txt";

fn main() {
    let lines = get_lines(INPUT_FILE);
    let has_overlap_count = lines.flatten().fold(0, |has_overlap_count, line| {
        let mut split = line.split(',');
        if let (Some(range_1), Some(range_2)) = (split.next(), split.next()) {
            let (r1_low, r1_high) = parse_range(range_1);
            let (r2_low, r2_high) = parse_range(range_2);
            if r1_low <= r2_low && r1_high >= r2_low || r2_low <= r1_low && r2_high >= r1_low {
                return has_overlap_count + 1;
            }
        }
        has_overlap_count
    });

    println!("Number of range pairs with overlap: {}", has_overlap_count);
}

fn parse_range(range: &str) -> (i32, i32) {
    let mut split = range.split('-');
    let low = split.next().unwrap().parse::<i32>().unwrap();
    let high = split.next().unwrap().parse::<i32>().unwrap();
    (low, high)
}
