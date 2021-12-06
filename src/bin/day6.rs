pub fn main() {
    let fish = parse_file();
    println!("D6P1 result: {}", solve(fish.clone(), 18));
    println!("D6P2 result: {}", solve(fish, 256));
}

fn parse_file() -> Vec<usize> {
    let contents: Vec<i8> = include_str!("../inputs/6.txt")
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let mut fish: Vec<usize> = vec![0; 9];
    for i in contents {
        fish[i as usize] += 1;
    }
    fish
}

fn solve(mut fish: Vec<usize>, days: i16) -> usize {
    for _ in 0..days {
        let new_fish = fish[0];
        fish[0] = 0;
        fish.rotate_left(1);
        fish[6] += new_fish;
        fish[8] += new_fish;
    }
    fish.iter().sum()
}

#[test]
fn test_p1() {
    assert_eq!(solve(parse_file(), 80), 372984);
}
