pub fn main() {
    let input = parse_file();
    println!("D9P1 result: {}", part1(input));
    // println!("D9P2 result: {}", part2(&input));
}

type Grid<'a> = Vec<&'a [u8]>;

fn parse_file() -> Grid<'static> {
    include_bytes!("../inputs/9.txt")
        .split(|&b| b == b'\n')
        .collect::<Vec<_>>()
}

fn part1(grid: Grid) -> isize {
    let mut vents = 0;
    let neighbors = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    for (y, line) in grid.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if neighbors.iter().all(|&(xx, yy)| {
                grid.get(y.overflowing_add(yy as usize).0)
                    .and_then(|l| l.get(x.overflowing_add(xx as usize).0))
                    .map(|n| cell < n)
                    .unwrap_or(true)
            }) {
                vents += (cell - b'0') as usize + 1;
            }
        }
    }
    vents as isize
}

// fn part2(_input: &[String]) -> isize {
//     todo!();
// }

#[test]
fn test_p1() {
    assert_eq!(part1(parse_file()), 452);
}

// #[test]
// fn test_p2() {
//     assert_eq!(part2(&parse_file()), 0);
// }
