use std::fs;
use std::iter::Iterator;
use std::str::FromStr;

enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.trim().split(' ').collect();

        let value = match split[1].parse::<u32>() {
            Ok(v) => v,
            Err(_) => return Err("invalid value in command"),
        };

        let command = match split[0] {
            "forward" => Ok(Command::Forward(value)),
            "down" => Ok(Command::Down(value)),
            "up" => Ok(Command::Up(value)),
            _ => Err("unexpected command"),
        };

        command
    }
}

pub fn process_commands(input: &str) -> (u32, u32) {
    let commands: Vec<Command> = input
        .lines()
        .map(|line: &str| Command::from_str(line).unwrap())
        .collect();

    let final_location = commands.iter().fold((0, 0), |loc, command| match command {
        Command::Forward(i) => (loc.0 + i, loc.1),
        Command::Down(i) => (loc.0, loc.1 + i),
        Command::Up(i) => (loc.0, loc.1 - i),
    });

    final_location
}

pub fn run() {
    let input = fs::read_to_string("./src/input/day2.txt").expect("Missing input file");
    let result = process_commands(&input);
    println!(
        "final location: {:?}, magnitude: {}",
        result,
        result.0 * result.1
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";

        assert_eq!(process_commands(&input), (15, 10));
    }
}
