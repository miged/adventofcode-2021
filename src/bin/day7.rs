use std::cmp::Ordering;

pub fn main() {
    let input = parse_file();
    println!("D7P1 result: {}", part1(input.clone()));
    println!("D7P2 result: {}", part2(input));
}

fn parse_file() -> Vec<i32> {
    include_str!("../inputs/7.txt")
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

fn part1(input: Vec<i32>) -> i32 {
    let max: usize = *input.iter().max().unwrap() as usize;
    let mut fuel_used: Vec<i32> = vec![0; max];

    for pos in 0..max {
        for i in &input {
            fuel_used[pos] += (i - pos as i32).abs() as i32;
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

fn part2(input: Vec<i32>) -> i32 {
    let max: usize = *input.iter().max().unwrap() as usize;
    let mut fuel_used: Vec<i32> = vec![0; max];

    for pos in 0..max {
        for i in &input {
            for distance in 0..(i - pos as i32).abs() {
                fuel_used[pos] += 1 + distance;
            }
        }
    }

    let min_pos = fuel_used
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index)
        .unwrap();

    dbg!(min_pos);
    fuel_used[min_pos]
}

#[test]
fn test_p1() {
    assert_eq!(part1(parse_file()), 336721);
}

#[test]
fn test_p2() {
    assert_eq!(part2(parse_file()), 91638945);
}
