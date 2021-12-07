type FishTab = [usize; 9];

fn parse_input(input: &str) -> FishTab {
    let fish: Vec<u8> = input
        .trim()
        .split(",")
        .map(|fish| fish.parse().expect("failed to parse fish"))
        .collect();

    let mut fish_tab: FishTab = [0; 9];
    for f in fish {
        fish_tab[f as usize] += 1;
    }

    fish_tab
}

fn run_a_day(population: FishTab) -> FishTab {
    // consume the old poplation and generate a new one
    let mut new_population: FishTab = [0; 9];

    // reset fish at 0 and add same amount of new borns
    new_population[6] += population[0];
    new_population[8] += population[0];
    for age in 1..population.len() {
        new_population[age - 1] += population[age];
    }
    new_population
}

fn count_fish(input: &str, days: usize) -> usize {
    let mut population: FishTab = parse_input(input);

    for _i in 0..days {
        // run a day simulation
        population = run_a_day(population);
        //println!("{} -> fish:{}", _i, (population.iter().sum::<usize>()));
    }

    population.iter().sum()
}

pub fn run() {
    let input = std::fs::read_to_string("./src/input/day6.txt").expect("Missing input file");
    println!(
        "day6/p1 - number of lanternfish after 80: {}",
        count_fish(&input, 80)
    );
    println!(
        "day6/p2 - number of lanternfish after 256: {}",
        count_fish(&input, 256)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "3,4,3,1,2";
        assert_eq!(count_fish(&input, 18), 26);
        assert_eq!(count_fish(&input, 80), 5934);
        assert_eq!(count_fish(&input, 256), 26984457539);
    }
}
