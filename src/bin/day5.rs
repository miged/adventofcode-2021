use std::cmp::{max, min};

pub fn main() {
    let input = parse_file();
    println!("D5P1 result: {}", part1(input));
    // println!("D5P2 result: {}", part2(&input));
}

#[derive(Debug)]
struct Line {
    from: (usize, usize),
    to: (usize, usize),
}

fn parse_file() -> Vec<Line> {
    let contents: String = include_str!("../inputs/5.txt").into();

    // parse file contents
    let contents: Vec<String> = contents
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    let mut entries: Vec<Line> = Vec::new();
    for row in contents {
        let split: Vec<String> = row
            .split_ascii_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();
        let tup1: Vec<usize> = split[0].split(',').map(|i| i.parse().unwrap()).collect();
        let tup2: Vec<usize> = split[2].split(',').map(|i| i.parse().unwrap()).collect();
        entries.push(Line {
            from: (tup1[0], tup1[1]),
            to: (tup2[0], tup2[1]),
        });
    }

    entries
}

fn part1(lines: Vec<Line>) -> usize {
    let mut grid: Vec<Vec<usize>> = vec![vec![0; 999]; 999];

    // fill grid with lines
    for line in lines {
        // skip diagonals
        if line.from.0 != line.to.0 && line.from.1 != line.to.1 {
            continue;
        }
        if line.from.0 == line.to.0 {
            // vertical line
            for i in min(line.from.1, line.to.1)..max(line.from.1, line.to.1) + 1 {
                grid[i][line.from.0] += 1;
            }
        } else {
            // horizontal line
            for i in min(line.from.0, line.to.0)..max(line.from.0, line.to.0) + 1 {
                grid[line.from.1][i] += 1;
            }
        }
    }

    // count overlapping points
    grid.iter().flatten().filter(|x| **x > 1).count()
}

// fn part2(input: &[String]) -> isize {
//     todo!();
// }

#[test]
fn test_p1() {
    assert_eq!(part1(parse_file()), 7644);
}

// #[test]
// fn test_p2() {
//     assert_eq!(part2(&parse_file()), 0);
// }

// for i in 0..grid.len() {
//     println!("{:?}", grid[i]);
// }
