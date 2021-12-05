pub fn main() {
    let mut lines = parse_file();
    println!("D5P1 result: {}", solve(&mut lines, true));
    println!("D5P2 result: {}", solve(&mut lines, false));
}

#[derive(Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

fn parse_file() -> Vec<Line> {
    let input: &str = include_str!("../inputs/5.txt");
    let mut entries: Vec<Line> = Vec::new();

    for line in input.lines() {
        let (a, b) = line.split_once(" -> ").unwrap();
        let (x1, y1) = a
            .split_once(",")
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .unwrap();
        let (x2, y2) = b
            .split_once(",")
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .unwrap();
        entries.push(Line {
            start: (x1, y1),
            end: (x2, y2),
        });
    }

    entries
}

fn solve(lines: &mut [Line], ignore_diagonals: bool) -> usize {
    let mut grid: Vec<Vec<i32>> = vec![vec![0; 999]; 999];
    for line in lines {
        if line.start.0 != line.end.0 && line.start.1 != line.end.1 {
            // diagonal line
            if !ignore_diagonals {
                for _ in line.start.1.min(line.end.1)..line.start.1.max(line.end.1) + 1 {
                    grid[line.start.1 as usize][line.start.0 as usize] += 1;
                    line.start.0 += (line.end.0 - line.start.0).signum();
                    line.start.1 += (line.end.1 - line.start.1).signum();
                }
            }
        } else if line.start.0 == line.end.0 {
            // vertical line
            for i in line.start.1.min(line.end.1)..line.start.1.max(line.end.1) + 1 {
                grid[i as usize][line.start.0 as usize] += 1;
            }
        } else {
            // horizontal line
            for i in line.start.0.min(line.end.0)..line.start.0.max(line.end.0) + 1 {
                grid[line.start.1 as usize][i as usize] += 1;
            }
        }
    }

    // count overlapping points
    grid.iter().flatten().filter(|&&n| n > 1).count()
}

#[test]
fn test_p1() {
    assert_eq!(solve(&mut parse_file(), true), 7644);
}

#[test]
fn test_p2() {
    assert_eq!(solve(&mut parse_file(), false), 18627);
}
