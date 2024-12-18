use advent_of_code::*;
use std::collections::HashMap;

fn main() {
    let input = read_input(2024, 11);
    let part1_result = partx(&input, 25);
    println!("Part 1: {}", part1_result);

    let part1_result = partx(&input, 75);
    println!("Part 2: {}", part1_result);
}

fn partx(input: &str, blinks: u32) -> u64 {
    let stones = input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .map(|v| v.parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()
        .unwrap();
    count_stones(stones, blinks)
}

fn count_stones(initial_stones: Vec<u64>, blinks: u32) -> u64 {
    let mut stone_map: HashMap<u64, u64> = HashMap::new();
    initial_stones.iter().for_each(|s| {
        stone_map.entry(*s).and_modify(|c| *c += 1).or_insert(1);
    });

    for i in 0..blinks {
        let mut new_stone_map: HashMap<u64, u64> = HashMap::new();
        for k in stone_map.keys() {
            let new_stones = apply_rule(*k);
            let current_stone_count = stone_map[k];
            for stone in new_stones {
                new_stone_map
                    .entry(stone)
                    .and_modify(|c| *c += current_stone_count)
                    .or_insert(current_stone_count);
            }
        }
        stone_map = new_stone_map;
        // println!("blink: {} | {:?}", i, stone_map.len());
    }

    return stone_map.values().sum();
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
    let result = partx(input, 25);
    println!("Counts {:?}", result);
    assert_eq!(result, 55312);
}
