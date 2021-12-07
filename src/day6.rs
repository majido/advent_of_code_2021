fn parse_input(input: &str) -> Vec<u8> {
    input.trim()
        .split(",")
        .map(|fish| fish.parse().expect("failed to parse fish"))
        .collect()
}

fn run_a_day(population: Vec<u8>) -> Vec<u8> {
    // consume the old poplation and generate a new one
    let mut new_population: Vec<u8> = vec![];
    for fish in population {
        match fish {
            0 => {
                // reset this fish and add a new born
                new_population.push(6u8);
                new_population.push(8u8);
            }
            _ => new_population.push(fish - 1),
        };
    }
    new_population
}

fn part_one(input: &str, days: usize) -> usize {
    let mut population: Vec<u8> = parse_input(input);

    for i in 0..days {
        // run a day simulation
        population = run_a_day(population);
        //println!("{} -> fish:{}", i, population.len());
    }

    population.len()
}

pub fn run() {
    let input = std::fs::read_to_string("./src/input/day6.txt").expect("Missing input file");
    println!(
        "day6 - p1 - number of lanterfish after 80: {}",
        part_one(&input, 80)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "3,4,3,1,2";

        assert_eq!(part_one(&input, 18), 26);
        assert_eq!(part_one(&input, 80), 5934);
        // trick question :), it is going to consume all my ram.
        //assert_eq!(part_one(&input, 256), 26984457539);

    }
}
