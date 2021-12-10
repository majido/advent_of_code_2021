pub fn parse(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(",")
        .map(|v| v.parse().expect("parse failed"))
        .collect()
}

pub fn find_least_cost_position(mut crabs: Vec<i32>) -> (i32, i32) {
    // Value = sum abs(position - X)
    // Find X such that we minimize Value. I think this is a convex function since it is sum of abs.

    let cost =
        |crabs: &[i32], position: i32| crabs.iter().fold(0, |acc, i| acc + (i - position).abs());

    crabs.sort();

    // We probably can stop the moment value stops going down.
    let min = crabs
        .iter()
        .fold((i32::MAX, -1), |(min_cost, min_crab), crab| {
            let c = cost(&crabs, *crab);
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

pub fn run() {
    let input = std::fs::read_to_string("./src/input/day7.txt").expect("Missing input file");
    let min = find_least_cost_position(parse(&input));
    println!(
        "day7/1 - optimal crab position: {}, required fuel{}",
        min.1, min.0
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(find_least_cost_position(parse(&input)).1, 2);
    }
}
