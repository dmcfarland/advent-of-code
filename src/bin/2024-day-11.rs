use advent_of_code::*;
use std::collections::HashMap;

fn main() {
    let input = read_input(2024, 11);
    let part1_result = part1(&input, 25);
    println!("Part 1: {}", part1_result);

    // let part2_result = part2(&input);
    // println!("Part 2: {}", part2_result);
}

fn part1(input: &str, blinks: u32) -> u64 {
    let stones = input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .map(|v| v.parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()
        .unwrap();

    let mut lookup: HashMap<u64, Vec<u64>> = HashMap::new();
    for i in 0..10 {
        lookup.insert(i, build_lookup(vec![i], 40));
    }

    count_stones(stones, blinks, lookup)
}

fn build_lookup(initial_stones: Vec<u64>, blinks: u32) -> Vec<u64> {
    let mut stonez: Vec<u64> = initial_stones.clone();
    let mut num_stones: Vec<u64> = Vec::new();
    for _ in 0..blinks {
        stonez = stonez.iter().flat_map(|&f| apply_rule(f)).collect();
        num_stones.push(stonez.len() as u64);
        // println!("{:?} || {} -- {}", stonez, stonez.len(), i);
    }
    println!("lookup: {:?}", initial_stones);
    num_stones
}

fn count_stones(initial_stones: Vec<u64>, blinks: u32, lookup: HashMap<u64, Vec<u64>>) -> u64 {
    let mut stonez: Vec<u64> = initial_stones.clone();
    let mut total = 0;
    for i in 0..blinks {
        stonez = stonez
            .iter()
            .flat_map(|&f| {
                if let Some(r) = lookup.get(&f) {
                    println!("r{:?}", r);
                    total += r[(blinks - i - 1) as usize];
                    Vec::new()
                } else {
                    apply_rule(f)
                }
            })
            .collect();
        // println!("{:?} || {} -- {} ({})", stonez, stonez.len(), i, total);
        println!("blink: {}", i);
    }
    total + (stonez.len() as u64)
}

fn apply_rule(f: u64) -> Vec<u64> {
    if f == 0 {
        vec![1]
    } else {
        let num = f;
        let mut digits = 0;
        let mut temp = f;

        while temp > 0 {
            digits += 1;
            temp /= 10;
        }

        if digits % 2 == 0 && digits > 0 {
            let div = 10_u64.pow((digits / 2) as u32);
            let right = num % div;
            let left = num / div;
            vec![left, right]
        } else {
            vec![f * 2024]
        }
    }
}

#[test]
fn test_part1_b() {
    let input = r#"125 17"#;
    let result = part1(input, 25);
    println!("Counts {:?}", result);
    assert_eq!(result, 55312);
}
