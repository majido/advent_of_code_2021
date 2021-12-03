// https://adventofcode.com/2021/day/1
use std::fs;

pub fn run() {
    let input = fs::read_to_string("./src/input/day1.txt").expect("Missing input file");
    let depths: Vec<i32> = input
        .split_ascii_whitespace()
        .map(|line: &str| line.parse::<i32>().unwrap())
        .collect();

    let grouped_depths: Vec<i32> = depths.windows(3).map(|w| w[0] + w[1] + w[2]).collect();

    println!("number of increased: {}", count_increases(&depths));
    println!(
        "number of increased (group of 3): {}",
        count_increases(&grouped_depths)
    );
}

// part 1: count the number of times a depth measurement increases from the
// previous measurement
fn count_increases(depths: &[i32]) -> i32 {
    let mut number_of_increases: i32 = 0;
    for d in depths.windows(2) {
        if d[1] > d[0] {
            number_of_increases += 1;
        }
    }
    number_of_increases
}
