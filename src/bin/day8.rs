pub fn main() {
    let input = parse_file();
    println!("D8P1 result: {}", part1(&input));
    // println!("D8P2 result: {}", part2(&input));
}

type Entry = (Vec<String>, Vec<String>);

fn parse_file() -> Vec<Entry> {
    let contents: Vec<String> = include_str!("../inputs/8.txt")
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    let mut entries: Vec<Entry> = Vec::new();
    for line in contents.iter() {
        let line = line.split_once('|').unwrap();
        let signals: Vec<String> = line
            .0
            .split_ascii_whitespace()
            .filter_map(|line| line.parse().ok())
            .collect();
        let output: Vec<String> = line
            .1
            .split_ascii_whitespace()
            .filter_map(|line| line.parse().ok())
            .collect();
        entries.push((signals, output));
    }

    entries
}

fn part1(entries: &[Entry]) -> isize {
    let mut numbers: Vec<isize> = vec![0; 10];

    for entry in entries {
        for output in &entry.1 {
            match output.len() {
                2 => numbers[1] += 1,
                4 => numbers[4] += 1,
                3 => numbers[7] += 1,
                7 => numbers[8] += 1,
                _ => (),
            }
        }
    }

    numbers.iter().sum()
}

fn part2(_input: &[String]) -> isize {
    todo!();
}

#[test]
fn test_p1() {
    assert_eq!(part1(&parse_file()), 375);
}

// #[test]
// fn test_p2() {
//     assert_eq!(part2(&parse_file()), 0);
// }
