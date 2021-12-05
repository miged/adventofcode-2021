pub fn main() {
    let (draws, boards) = parse_file();
    println!("D4P1 result: {}", part1(&draws, boards.clone()));
    println!("D4P2 result: {}", part2(&draws, boards));
}

type Board = Vec<Vec<i32>>;

fn parse_file() -> (Vec<i32>, Vec<Board>) {
    let contents: String = include_str!("../inputs/4.txt").into();

    // parse file contents
    let mut contents: Vec<String> = contents
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();
    contents.push("".to_string());

    // parse draw numbers
    let draws: Vec<i32> = contents[0].split(',').map(|i| i.parse().unwrap()).collect();

    // parse bingo boards
    let mut board: Board = Vec::new();
    let mut boards: Vec<Board> = Vec::new();
    for row in contents.iter().skip(2) {
        if !row.is_empty() {
            board.push(
                row.split_ascii_whitespace()
                    .map(|i| i.parse().unwrap())
                    .collect(),
            );
        } else {
            boards.push(board);
            board = Vec::new();
        }
    }

    (draws, boards)
}

fn part1(draws: &[i32], mut boards: Vec<Board>) -> i32 {
    for draw in draws {
        // iterate over boards
        for (board_number, board) in boards.clone().iter().enumerate() {
            boards[board_number] = mark_board(board.clone(), draw);

            // evaluate board
            if check_board_win(&boards[board_number]) {
                return get_score(&boards[board_number]) * draw;
            }
        }
    }
    0
}

fn part2(draws: &[i32], mut boards: Vec<Board>) -> i32 {
    let mut boards_won: Vec<usize> = Vec::new();
    for draw in draws {
        // iterate over boards
        for (board_number, board) in boards.clone().iter().enumerate() {
            if !boards_won.contains(&board_number) {
                boards[board_number] = mark_board(board.clone(), draw);

                // evaluate board
                if check_board_win(&boards[board_number]) {
                    boards_won.push(board_number);
                    if (boards.len() - boards_won.len()) == 0 {
                        return get_score(&boards[board_number]) * draw;
                    }
                }
            }
        }
    }
    0
}

fn mark_board(mut board: Board, draw: &i32) -> Board {
    // iterate over board numbers
    for (row, row_el) in board.clone().iter().enumerate() {
        for (column, el) in row_el.iter().enumerate() {
            if el == draw {
                board[row][column] = -1;
            }
        }
    }
    board.to_vec()
}

fn check_board_win(board: &Board) -> bool {
    let rotated = rotate(board);
    for (row, _) in board.iter().enumerate() {
        // if all numbers in row are marked
        if board[row] == vec![-1; 5] || rotated[row] == vec![-1; 5] {
            return true;
        }
    }
    false
}

fn rotate(arr: &Board) -> Board {
    let mut rotated: Board = arr.to_vec();
    for (row, _) in arr.iter().enumerate() {
        for (col, _) in arr[row].iter().enumerate() {
            rotated[col][row] = arr[row][col];
        }
    }
    rotated
}

fn get_score(board: &Board) -> i32 {
    board.iter().flatten().filter(|x| **x != -1).sum()
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
