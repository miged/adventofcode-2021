pub fn main() {
    let input = parse_file();
    println!("D1P1 result: {}", part1(&input));
    println!("D1P2 result: {}", part2(&input));
}

fn parse_file() -> Vec<i32> {
    let contents: String = include_str!("../inputs/1.txt").into();

    // parse file contents
    contents
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect()
}

fn part1(depths: &[i32]) -> usize {
    depths.windows(2).filter(|w| w[0] < w[1]).count()
}

fn part2(depths: &[i32]) -> usize {
    let windows: Vec<i32> = depths.windows(3).map(|w| w.iter().sum()).collect();
    windows.windows(2).filter(|w| w[0] < w[1]).count()
}

#[test]
fn test_p1() {
    assert_eq!(part1(&parse_file()), 1316);
}

#[test]
fn test_p2() {
    assert_eq!(part2(&parse_file()), 1344);
}
