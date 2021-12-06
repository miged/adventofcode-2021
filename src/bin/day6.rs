use std::collections::HashMap;

pub fn main() {
    let input = parse_file();
    println!("D6P1 result: {}", solve(input.clone(), 80));
    // println!("D6P2 result: {}", solve(input, 256));
}

fn parse_file() -> Vec<i8> {
    include_str!("../inputs/6.txt")
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

fn solve(mut fishes: Vec<i8>, days: i16) -> usize {
    for i in 0..days {
        println!("Iteration {}: {}", i, fishes.len());
        // decrease timer
        fishes = fishes.iter().map(|&n| n - 1).collect();

        // produce new fish
        let num_of_births: usize = fishes.iter().filter(|&&n| n == -1).count();
        fishes.resize(fishes.len() + num_of_births, 8);

        // reset timer
        fishes = fishes.iter().map(|&n| if n == -1 {6} else {n}).collect();
    }
    fishes.len()
}

#[test]
fn test_p1() {
    assert_eq!(solve(parse_file(), 80), 372984);
}
