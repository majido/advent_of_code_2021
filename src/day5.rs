use std::{cmp::max, cmp::min, collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord(u32, u32);

#[derive(Debug)]
struct Segment {
    start: Coord,
    end: Coord,
}

type Vents = Vec<Segment>;

impl FromStr for Coord {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<_> = s
            .trim()
            .split(',')
            .map(|v| v.parse::<u32>().expect("invalid value in command"))
            .collect();

        Ok(Coord(values[0], values[1]))
    }
}

impl Segment {
    // produce a list of all points
    pub fn points(&self) -> Vec<Coord> {
        if self.start.0 == self.end.0 {
            // horizontal line
            let x = self.start.0;
            let span = std::ops::Range {
                start: min(self.start.1, self.end.1),
                end: max(self.start.1, self.end.1) + 1,
            };
            span.map(|y| Coord(x, y)).collect()
        } else if self.start.1 == self.end.1 {
            // vertical line
            let y = self.start.1;
            let span = std::ops::Range {
                start: min(self.start.0, self.end.0),
                end: max(self.start.0, self.end.0) + 1,
            };
            span.map(|x| Coord(x, y)).collect()
        } else {
            // diagonal
            // comment out for part 1
            let mut x_delta: i64 = self.end.0 as i64 - self.start.0 as i64;
            x_delta /= x_delta.abs();
            let mut y_delta: i64 = self.end.1 as i64 - self.start.1 as i64;
            y_delta /= y_delta.abs();

            let mut coords: Vec<Coord> = vec![];
            let mut x: i64 = self.start.0 as i64;
            let mut y: i64 = self.start.1 as i64;

            while x != self.end.0 as i64 && y != self.end.1 as i64 {
                coords.push(Coord(x as u32, y as u32));
                x += x_delta;
                y += y_delta;
            }
            coords.push(self.end.clone());

            coords
        }
    }
}

fn parse_input(input: &str) -> Vents {
    input
        .lines()
        .map(|line| {
            //0,9 -> 5,9
            let mut coords = line.split("->");
            let start: Coord = coords
                .next()
                .unwrap()
                .parse()
                .expect("failed to parse start");
            let end: Coord = coords.next().unwrap().parse().expect("failed to parse end");

            Segment { start, end }
        })
        .collect()
}

fn find_dangerous(vents: &Vents) -> Vec<Coord> {
    // If this was just horizontal and vertical lines we could use a sorted
    // vector and then go from lowest to highest and count overlaps.
    // But I suspect they are going to expands to other segments so this approach
    // is harder to extend to slanted lines.

    // A memory hungry approch:
    // 1. Create a hash map of all points and count them as we see them
    let mut map: HashMap<Coord, u8> = HashMap::new();

    for vent in vents.iter() {
        for point in vent.points() {
            let count = map.entry(point).or_insert(0);
            *count += 1;
        }
    }

    // 2. Count all dots that have >2
    let at_least_two: Vec<Coord> = map
        .iter()
        .filter_map(|(key, count)| (*count >= 2).then(|| key.clone()))
        .collect();

    at_least_two
}

fn part_one(input: &str) -> usize {
    let vents = parse_input(input);

    let dangerous = find_dangerous(&vents);

    dangerous.len()
}

pub fn run() {
    let input = std::fs::read_to_string("./src/input/day5.txt").expect("Missing input file");
    println!("day5 - p1 - dangerous vents: {}", part_one(&input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        assert_eq!(part_one(&input), 12);
    }
}
