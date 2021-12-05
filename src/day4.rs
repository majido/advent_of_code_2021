#[derive(Debug)]

struct BingoLine {
    values: Vec<u32>,
    matched: Vec<u32>,
}

impl BingoLine {
    pub fn new(values: &[u32]) -> BingoLine {
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

    pub fn value(&self) -> u64 {
        self.matched.iter().fold(0, |sum, v| sum + v) as u64
    }

    pub fn is_bingo(&self) -> bool {
        self.values.len() == self.matched.len()
    }
}

#[derive(Debug)]
struct BingoBoard {
    lines: Vec<BingoLine>,
}

impl BingoBoard {
    pub fn new(board_lines: &[&str]) -> BingoBoard {
        dbg!(board_lines);
        // parse rows
        let rows: Vec<Vec<u32>> = board_lines
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
        let num_columns = rows[0].len();
        let mut columns: Vec<Vec<u32>> = Vec::new();
        for i in 0..num_columns {
            let mut column = Vec::new();
            for row in rows.iter() {
                column.push(row[i]);
            }
            columns.push(column);
        }
        let lines: Vec<BingoLine> = rows
            .iter()
            .chain(columns.iter())
            .map(|values: &Vec<u32>| BingoLine::new(&values))
            .collect();

        BingoBoard { lines }
    }

    pub fn observe(&mut self, value: u32) -> bool {
        let result: Vec<bool> = self
            .lines
            .iter_mut()
            .map(|line: &mut BingoLine| line.observe(value))
            .collect();

        result.iter().any(|v| *v)
    }
}

fn wining_board_score(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    dbg!(&lines);

    let calls: Vec<u32> = lines[0]
        .split(",")
        .map(|v| v.parse::<u32>().expect("error parsing calls"))
        .collect();

    let mut boards: Vec<BingoBoard> = lines[1..]
        .split(|line| line.is_empty())
        .filter_map(|lines| match lines.len() {
            0 => None,
            _ => Some(BingoBoard::new(lines)),
        })
        .collect();

    for call in calls.iter() {
        for board in boards.iter_mut() {
            // Figure our if observe should have take &u32 instead.
            let call_value: u32 = *call;
            if board.observe(call_value) {
                // Bingo
                println!("Bingo {:?}", board);
            }
        }
    }
    0
}

pub fn run() {
    // let input = fs::read_to_string("./src/input/day4.txt").expect("Missing input file");
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

        assert_eq!(wining_board_score(&input), 4512);
    }
}
