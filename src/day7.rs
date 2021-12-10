pub fn parse(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(",")
        .map(|v| v.parse().expect("parse failed"))
        .collect()
}

type CostFn = fn(&i32, &i32) -> i32;

fn linear_cost_fn(a: &i32, b: &i32) -> i32 {
    (a - b).abs()
}

fn increasing_cost_fn(a: &i32, b: &i32) -> i32 {
    // sum 1+2+3...+ (a-b)::abs()
    let n = (a - b).abs();
    n * (n + 1) / 2
}

pub fn find_least_cost_position(mut crabs: Vec<i32>, cost_fn: CostFn) -> (i32, i32) {
    // Value = sum abs(position - X)
    // Find X such that we minimize Value. I think this is a convex function since it is sum of abs.

    let cost =
        |crabs: &[i32], position: &i32| crabs.iter().fold(0, |acc, i| acc + cost_fn(i, position));

    crabs.sort();

    // We probably can stop the moment value stops going down.
    let min = crabs
        .iter()
        .fold((i32::MAX, -1), |(min_cost, min_crab), crab| {
            let c = cost(&crabs, crab);
            if c < min_cost {
                return (c, *crab);
            } else {
                return (min_cost, min_crab);
            }
        });

    min
    // This is a convex function so we can do binary search on it?
    // I was wrong. A convex function does not help here.
    //
    // let mut start: usize = 0;
    // let mut end: usize = crabs.len();
    //
    // while end - start > 1 {
    //     let pivot: usize = (start + (end - start) / 2).try_into().unwrap();
    //     let left_cost = cost(&crabs[..pivot], crabs[pivot]);
    //     let right_cost = cost(&crabs[pivot..], crabs[pivot]);
    //     println!("{}, [{}], {}", left_cost, pivot, right_cost);

    //     if left_cost == right_cost {
    //         return crabs[pivot];
    //     } else if left_cost < right_cost {
    //         start = pivot;
    //     } else if left_cost > right_cost {
    //         end = pivot;
    //     }
    // }
    // return crabs[start];
}

fn part1(input: &str) -> (i32, i32) {
    find_least_cost_position(parse(input), linear_cost_fn)
}

fn part2(input: &str) -> (i32, i32) {
    find_least_cost_position(parse(input), increasing_cost_fn)
}

pub fn run() {
    let input = std::fs::read_to_string("./src/input/day7.txt").expect("Missing input file");
    let min1 = part1(&input);
    println!(
        "day7/1 - optimal crab position: {}, required fuel:{}",
        min1.1, min1.0
    );

    let min2 = part2(&input);

    println!(
        "day7/2 - optimal crab position: {}, required fuel:{}",
        min2.1, min2.0
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(part1(&input).1, 2);
    }
}
