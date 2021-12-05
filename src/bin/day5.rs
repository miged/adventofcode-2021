pub fn main() {
    let mut lines = parse_file();
    println!("D5P1 result: {}", part1(&lines));
    println!("D5P2 result: {}", part2(&mut lines));
}

#[derive(Debug)]
struct Line {
    start: (usize, usize),
    end: (usize, usize),
}

fn parse_file() -> Vec<Line> {
    let contents: Vec<String> = include_str!("../inputs/5.txt")
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
            start: (tup1[0], tup1[1]),
            end: (tup2[0], tup2[1]),
        });
    }

    entries
}

fn part1(lines: &[Line]) -> usize {
    let mut grid: Vec<Vec<usize>> = vec![vec![0; 999]; 999];

    for line in lines {
        // skip diagonals
        if line.start.0 != line.end.0 && line.start.1 != line.end.1 {
            continue;
        }
        if line.start.0 == line.end.0 {
            // vertical line
            for i in line.start.1.min(line.end.1)..line.start.1.max(line.end.1) + 1 {
                grid[i][line.start.0] += 1;
            }
        } else {
            // horizontal line
            for i in line.start.0.min(line.end.0)..line.start.0.max(line.end.0) + 1 {
                grid[line.start.1][i] += 1;
            }
        }
    }

    // count overlapping points
    grid.iter().flatten().filter(|x| **x > 1).count()
}

fn part2(lines: &mut [Line]) -> usize {
    let mut grid: Vec<Vec<usize>> = vec![vec![0; 999]; 999];

    for line in lines {
        if line.start.0 != line.end.0 && line.start.1 != line.end.1 {
            // diagonal line
            for _ in line.start.1.min(line.end.1)..line.start.1.max(line.end.1) + 1 {
                grid[line.start.1][line.start.0] += 1;
                if line.start.0 < line.end.0 {
                    line.start.0 += 1;
                } else {
                    line.start.0 -= 1;
                }

                if line.start.1 < line.end.1 {
                    line.start.1 += 1;
                } else {
                    line.start.1 -= 1;
                }
            }
        } else if line.start.0 == line.end.0 {
            // vertical line
            for i in line.start.1.min(line.end.1)..line.start.1.max(line.end.1) + 1 {
                grid[i][line.start.0] += 1;
            }
        } else {
            // horizontal line
            for i in line.start.0.min(line.end.0)..line.start.0.max(line.end.0) + 1 {
                grid[line.start.1][i] += 1;
            }
        }
    }

    // count overlapping points
    grid.iter().flatten().filter(|x| **x > 1).count()
}

#[test]
fn test_p1() {
    assert_eq!(part1(&parse_file()), 7644);
}

#[test]
fn test_p2() {
    assert_eq!(part2(&mut parse_file()), 18627);
}
