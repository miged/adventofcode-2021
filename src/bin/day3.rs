pub fn main() {
    let input = parse_file();
    println!("D3P1 result: {}", part1(&input));
    println!("D3P2 result: {}", part2(&input));
}

fn parse_file() -> Vec<String> {
    let contents: String = include_str!("../inputs/3.txt").into();

    // parse file contents
    contents
        .lines()
        .filter_map(|line| line.parse::<String>().ok())
        .collect::<Vec<String>>()
}

fn part1(input: &[String]) -> isize {
    let (common_bits, least_common_bits) = get_common_bits(input.to_vec());

    // convert to binary string
    let common_binary = common_bits
        .iter()
        .map(|x| x.to_string())
        .collect::<String>();
    let least_binary = least_common_bits
        .iter()
        .map(|x| x.to_string())
        .collect::<String>();

    let gamma_rate = isize::from_str_radix(&common_binary, 2).unwrap();
    let epsilon_rate = isize::from_str_radix(&least_binary, 2).unwrap();
    gamma_rate * epsilon_rate
}

fn part2(input: &[String]) -> isize {
    let mut ox_rating: Vec<String> = input.to_vec();
    let mut co_rating: Vec<String> = input.to_vec();

    for c in 0..input[0].len() {
        if ox_rating.len() > 1 {
            let (common_bits, _) = get_common_bits(ox_rating.to_vec());
            ox_rating = ox_rating
                .iter()
                .filter(|x| x.chars().nth(c).unwrap().to_digit(10).unwrap() == common_bits[c])
                .cloned()
                .collect();
        }

        if co_rating.len() > 1 {
            let (_, least_common_bits) = get_common_bits(co_rating.to_vec());
            co_rating = co_rating
                .iter()
                .filter(|x| x.chars().nth(c).unwrap().to_digit(10).unwrap() == least_common_bits[c])
                .cloned()
                .collect();
        }
    }

    let ox_rating = isize::from_str_radix(&ox_rating[0], 2).unwrap();
    let co_rating = isize::from_str_radix(&co_rating[0], 2).unwrap();
    ox_rating * co_rating
}

fn get_common_bits(input: Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let mut common_bits: Vec<u32> = vec![];
    let mut least_common_bits: Vec<u32> = vec![];

    for c in 0..input[0].len() {
        // get bits into columns
        let mut column: Vec<u32> = vec![];
        input
            .iter()
            .for_each(|s| column.push(s.chars().nth(c).unwrap().to_digit(10).unwrap()));

        let zeroes = column.iter().filter(|&n| *n == 0).count();
        if zeroes > (column.len() / 2) {
            // most common: 0
            common_bits.push(0);
            least_common_bits.push(1);
        } else {
            // most common: 1
            common_bits.push(1);
            least_common_bits.push(0);
        }
    }

    (common_bits, least_common_bits)
}

#[test]
fn test_p1() {
    assert_eq!(part1(&parse_file()), 4001724);
}

#[test]
fn test_p2() {
    assert_eq!(part2(&parse_file()), 587895);
}
