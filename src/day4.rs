use std::{fs, iter::Iterator};

#[derive(Debug)]
struct BingoLine {
    values: Vec<u32>,
    matched: Vec<u32>,
}

impl BingoLine {
    pub fn new(values: &[u32]) -> BingoLine {
        // TODO: This could be (u32, bool) instead of two vectors.
        BingoLine {
            values: Vec::from(values),
            matched: Vec::new(),
        }
    }

    pub fn observe(&mut self, value: u32) -> bool {
        if self.values.contains(&value) {
            self.matched.push(value);
        }

        self.is_bingo()
    }

    pub fn unmarked_sum(&self) -> u64 {
        let mut unmarked: Vec<u32> = vec![];
        for value in self.values.iter() {
            if !self.matched.contains(value) {
                unmarked.push(*value);
            }
        }
        unmarked.iter().fold(0, |sum, v| sum + v) as u64
    }

    pub fn is_bingo(&self) -> bool {
        self.values.len() == self.matched.len()
    }
}

#[derive(Debug)]
struct BingoBoard {
    rows: Vec<BingoLine>,
    columns: Vec<BingoLine>,
    is_complete: bool,
}

impl BingoBoard {
    pub fn new(board_lines: &[&str]) -> BingoBoard {
        // dbg!(board_lines);
        // parse rows
        let row_vlaues: Vec<Vec<u32>> = board_lines
            .iter()
            .map(|line| {
                let values: Vec<u32> = line
                    .split_ascii_whitespace()
                    .map(|v: &str| v.parse::<u32>().expect("cannot parse u32"))
                    .collect();
                values
            })
            .collect();

        // precompute columns
        let num_columns = row_vlaues[0].len();
        let mut column_values: Vec<Vec<u32>> = Vec::new();
        for i in 0..num_columns {
            let mut column = Vec::new();
            for row in row_vlaues.iter() {
                column.push(row[i]);
            }
            column_values.push(column);
        }
        let rows: Vec<BingoLine> = row_vlaues
            .iter()
            .map(|values: &Vec<u32>| BingoLine::new(&values))
            .collect();

        let columns: Vec<BingoLine> = column_values
            .iter()
            .map(|values: &Vec<u32>| BingoLine::new(&values))
            .collect();

        BingoBoard {
            rows,
            columns,
            is_complete: false,
        }
    }

    pub fn observe(&mut self, value: u32) -> bool {
        let result: Vec<bool> = self
            .rows
            .iter_mut()
            .chain(self.columns.iter_mut())
            .map(|line: &mut BingoLine| line.observe(value))
            .collect();

        self.is_complete = result.iter().any(|v| *v);

        self.is_complete
    }

    pub fn unmarked_sum(&self) -> u64 {
        let sum_unmarked = self
            .rows
            .iter()
            .fold(0, |sum, line: &BingoLine| sum + line.unmarked_sum());

        sum_unmarked
    }

    pub fn is_complete(&self) -> bool {
        self.is_complete
    }
}

fn run_bingo(calls: Vec<u32>, boards: &mut Vec<BingoBoard>) -> Vec<u64> {
    let mut winner_scores: Vec<u64> = vec![];

    for call in calls.iter() {
        for board in boards.iter_mut() {
            // skip boards that have won.
            if board.is_complete() {
                continue;
            }

            // TODO: Figure out if observe should have taken &u32 instead.
            // Bingo
            if board.observe(*call) {
                let winning_score = board.unmarked_sum() * *call as u64;
                winner_scores.push(winning_score);
            }
        }
    }

    winner_scores
}

fn find_first_winner_score(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();

    let (calls, mut boards) = construct_boards(lines);

    let winner_scores = run_bingo(calls, &mut boards);
    winner_scores[0]
}

fn find_last_winner_score(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();

    let (calls, mut boards) = construct_boards(lines);

    let winner_scores = run_bingo(calls, &mut boards);
    let last_score: u64 = *winner_scores.last().unwrap();
    last_score
}

fn construct_boards(lines: Vec<&str>) -> (Vec<u32>, Vec<BingoBoard>) {
    let calls: Vec<u32> = lines[0]
        .split(",")
        .map(|v| v.parse::<u32>().expect("error parsing calls"))
        .collect();

    let boards: Vec<BingoBoard> = lines[1..]
        .split(|line| line.is_empty())
        .filter_map(|lines| match lines.len() {
            0 => None,
            _ => Some(BingoBoard::new(lines)),
        })
        .collect();

    (calls, boards)
}

pub fn run() {
    let input = fs::read_to_string("./src/input/day4.txt").expect("Missing input file");
    println!(
        "day4 - First winner score: {}",
        find_first_winner_score(&input)
    );
    println!(
        "day4 - Last winner score: {}",
        find_last_winner_score(&input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        assert_eq!(find_first_winner_score(&input), 4512);
        assert_eq!(find_last_winner_score(&input), 1924);
    }
}
