use std::collections::HashMap;
use std::collections::HashSet;

use advent_of_code::*;

fn main() {
    let input = read_input(2024, 22);
    let part1_result = part1(&input);
    println!("Part 1: {:?}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2: {}", part2_result);
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

fn part2(input: &str) -> usize {
    let sns: Vec<usize> = input.lines().map(|l| l.parse::<usize>().unwrap()).collect();

    let x: Vec<HashMap<[i8; 4], u16>> = sns
        .iter()
        .map(|sn| {
            let mut first_sequence: HashMap<[i8; 4], u16> = HashMap::new();
            let mut secret_number = sn.clone();
            let mut seq = Vec::new();
            seq.push(sn % 10);
            for _ in 0..2000 {
                let e1 = ((secret_number * 64) ^ secret_number) % 16777216;
                let e2 = ((e1 / 32) ^ e1) % 16777216;
                let e3 = ((e2 * 2048) ^ e2) % 16777216;
                secret_number = e3;
                seq.push(secret_number % 10);
            }

            for w in seq.windows(5) {
                let key = [
                    w[1] as i8 - w[0] as i8,
                    w[2] as i8 - w[1] as i8,
                    w[3] as i8 - w[2] as i8,
                    w[4] as i8 - w[3] as i8,
                ];
                if !first_sequence.contains_key(&key) {
                    first_sequence.insert(key, w[4] as u16);
                }
            }
            first_sequence
        })
        .collect();

    let mut max: (usize, [i8; 4]) = (0, [0, 0, 0, 0]);
    let keys: HashSet<[i8; 4]> = x.iter().flat_map(|f| f.keys()).cloned().collect();

    for e in keys.iter() {
        let sum: usize = x
            .iter()
            .map(|f| {
                if f.contains_key(e) {
                    f[e] as usize
                } else {
                    0_usize
                }
            })
            .sum();
        if sum > max.0 {
            max = (sum, *e);
        }
    }

    println!("max: {:?}", max);

    return max.0;
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

const SAMPLE_2: &str = "\
1
2
3
2024";

#[test]
fn part2_test() {
    let input = left_trim_lines(SAMPLE_2);
    assert_eq!(part2(&input), 23);
}
