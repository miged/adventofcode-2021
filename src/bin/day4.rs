pub fn main() {
    let (draws, boards) = parse_file();
    println!("D4P1 result: {}", part1(&draws, boards.clone()));
    println!("D4P2 result: {}", part2(&draws, boards));
}

fn parse_file() -> (Vec<i32>, Vec<Vec<Vec<i32>>>) {
    let contents: String = include_str!("../inputs/4.txt").into();

    // parse file contents
    let mut contents: Vec<String> = contents
        .lines()
        .filter_map(|line| line.parse::<String>().ok())
        .collect();
    contents.push("".to_string());

    // parse draw numbers
    let draws: Vec<i32> = contents[0]
        .split(',')
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    // parse bingo boards
    let mut board: Vec<Vec<i32>> = Vec::new();
    let mut boards: Vec<Vec<Vec<i32>>> = Vec::new();
    for row in 2..contents.len() {
        if !contents[row].is_empty() {
            board.push(
                contents[row]
                    .split_ascii_whitespace()
                    .map(|i| i.parse::<i32>().unwrap())
                    .collect(),
            );
        } else {
            boards.push(board);
            board = Vec::new();
        }
    }

    (draws, boards)
}

fn part1(draws: &[i32], mut boards: Vec<Vec<Vec<i32>>>) -> i32 {
    let mut score = 0;

    'outer: for draw in draws {
        // iterate over boards
        for (board_number, mut board) in boards.clone().iter().enumerate() {
            // iterate over board numbers
            for (row, row_el) in board.iter().enumerate() {
                for (column, el) in row_el.iter().enumerate() {
                    if el == draw {
                        boards[board_number][row][column] = -1;
                    }
                }
            }
            board = &boards[board_number];

            // evaluate board
            for (row, _) in board.iter().enumerate() {
                // if all numbers in row are marked
                if board[row] == vec![-1; 5] || rotate(board.clone())[row] == vec![-1; 5] {
                    // get final score
                    score = board.iter().flatten().filter(|x| **x != -1).sum::<i32>() * draw;
                    break 'outer;
                }
            }
        }
    }

    score
}

fn part2(draws: &[i32], mut boards: Vec<Vec<Vec<i32>>>) -> i32 {
    let mut score = 0;
    let mut boards_won: Vec<usize> = Vec::new();

    'outer: for draw in draws {
        // iterate over boards
        for (board_number, mut board) in boards.clone().iter().enumerate() {
            // iterate over board numbers
            for (row, row_el) in board.iter().enumerate() {
                for (column, el) in row_el.iter().enumerate() {
                    if el == draw {
                        boards[board_number][row][column] = -1;
                    }
                }
            }
            board = &boards[board_number];

            // evaluate board
            if !boards_won.contains(&board_number) {
                'inner: for (row, _) in board.iter().enumerate() {
                    // if all numbers in row are marked
                    if board[row] == vec![-1; 5] || rotate(board.clone())[row] == vec![-1; 5] {
                        boards_won.push(board_number);
                        if (boards.len() - boards_won.len()) == 0 {
                            // get final score
                            score = board.iter().flatten().filter(|x| **x != -1).sum::<i32>() * draw;
                            break 'outer;
                        }
                        break 'inner;
                    }
                }
            }
        }
    }

    score
}

fn rotate(arr: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut rotated: Vec<Vec<i32>> = arr.clone();
    for (row, _) in arr.iter().enumerate() {
        for (col, _) in arr[row].iter().enumerate() {
            rotated[col][row] = arr[row][col];
        }
    }
    rotated
}

#[test]
fn test_p1() {
    let (draws, boards) = parse_file();
    assert_eq!(part1(&draws, boards), 10374);
}

#[test]
fn test_p2() {
    let (draws, boards) = parse_file();
    assert_eq!(part2(&draws, boards), 24742);
}
