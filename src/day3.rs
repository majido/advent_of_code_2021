use std::fs;

fn power_consumption(input: &str) -> (u32, u32) {
    // Each bit in the gamma rate can be determined by finding the most common bit
    // in the corresponding position of all numbers in the diagnostic report. For
    // example, given the following diagnostic report:
    let powers: Vec<&str> = input.lines().collect();

    let max_char = powers[0].len();

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for i in 0..max_char {
        let number_of_ones = powers
            .iter()
            .filter(|&v| v.chars().nth(i).eq(&Some('1')))
            .count();

        let bit: u32 = match number_of_ones > powers.len() / 2 {
            true => 1,
            false => 0,
        };
        // Shift gamma and add latest bit
        gamma = gamma << 1;
        gamma = gamma ^ bit;

        // The epsilon rate is calculated in a similar way; rather than use the most
        // common bit, the least common bit from each position is used.
        epsilon = epsilon << 1;
        epsilon = epsilon ^ (!bit & 1);
    }
    println!("gamma: {:#b}, epsilon: {:#b}", gamma, epsilon);

    (gamma, epsilon)
}

pub fn run() {
    let input = fs::read_to_string("./src/input/day3.txt").expect("Missing input file");
    let result1 = power_consumption(&input);
    println!(
        "part 1 - power_consumption: {:?}, magnitude: {}",
        result1,
        result1.0 * result1.1
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        assert_eq!(power_consumption(&input), (22, 9));
    }
}
