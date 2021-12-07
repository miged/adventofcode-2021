use std::cmp::Ordering;

pub fn main() {
    let input = parse_file();
    println!("D7P1 result: {}", solve(input.clone(), true));
    println!("D7P2 result: {}", solve(input, false));
}

fn parse_file() -> Vec<i32> {
    include_str!("../inputs/7.txt")
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

fn solve(crabs: Vec<i32>, constant_consumption: bool) -> i32 {
    let max: usize = *crabs.iter().max().unwrap() as usize;
    let mut fuel_used: Vec<i32> = vec![0; max];

    for pos in 0..max {
        for crab in &crabs {
            let distance = (crab - pos as i32).abs();
            match constant_consumption {
                true => fuel_used[pos] += distance,
                false => fuel_used[pos] += distance * (distance + 1) / 2,
            }
        }
    }

    let min_pos = fuel_used
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index)
        .unwrap();

    fuel_used[min_pos]
}

#[test]
fn test_p1() {
    assert_eq!(solve(parse_file(), true), 336721);
}

#[test]
fn test_p2() {
    assert_eq!(solve(parse_file(), false), 91638945);
}
