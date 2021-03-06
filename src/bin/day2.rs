pub fn main() {
    let input = parse_file();
    println!("D2P1 result: {}", part1(&input));
    println!("D2P2 result: {}", part2(&input));
}

fn parse_file() -> Vec<String> {
    include_str!("../inputs/2.txt")
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect()
}

fn part1(directions: &[String]) -> i32 {
    let (mut x, mut y) = (0, 0);

    for d in directions
        .iter()
        .map(|d| d.split(' ').collect::<Vec<&str>>())
    {
        let unit: i32 = d[1].parse().unwrap();
        match d[0] {
            "forward" => x += unit,
            "up" => y -= unit,
            "down" => y += unit,
            _ => (),
        }
    }
    x * y
}

fn part2(directions: &[String]) -> i32 {
    let (mut x, mut y, mut aim) = (0, 0, 0);

    for d in directions
        .iter()
        .map(|d| d.split(' ').collect::<Vec<&str>>())
    {
        let unit = d[1].parse::<i32>().unwrap();
        match d[0] {
            "forward" => {
                x += unit;
                y += unit * aim;
            }
            "up" => aim -= unit,
            "down" => aim += unit,
            _ => (),
        }
    }
    x * y
}

#[test]
fn test_p1() {
    assert_eq!(part1(&parse_file()), 1989265);
}

#[test]
fn test_p2() {
    assert_eq!(part2(&parse_file()), 2089174012);
}
