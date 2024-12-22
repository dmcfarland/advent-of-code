use advent_of_code::*;

fn main() {
    let input = read_input(2024, 22);
    let part1_result = part1(&input);
    println!("Part 1: {:?}", part1_result);

    // let part2_result = part2(&input);
    // println!("Part 2: {}", part2_result);
}

fn part1(input: &str) -> usize {
    let sns: Vec<usize> = input.lines().map(|l| l.parse::<usize>().unwrap()).collect();

    let mut total = 0_usize;
    for sn in sns {
        let mut secret_number = sn;
        for _ in 0..2000 {
            let e1 = ((secret_number * 64) ^ secret_number) % 16777216;
            let e2 = ((e1 / 32) ^ e1) % 16777216;
            let e3 = ((e2 * 2048) ^ e2) % 16777216;
            secret_number = e3;
        }
        println!("{}: {}", sn, secret_number);
        total += secret_number;
    }

    return total;
}

const SAMPLE_1: &str = "\
1
10
100
2024";

#[test]
fn part1_test() {
    let input = left_trim_lines(SAMPLE_1);
    assert_eq!(part1(&input), 37327623);
}
