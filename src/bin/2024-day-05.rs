use advent_of_code::*;
use std::cmp::Ordering;
use std::collections::HashMap;

// Star 1 - 00:47:19
// Star 2 - 01:06:37

fn main() {
    let input = read_input(2024, 5);
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2: {}", part2_result);
}

fn part1(input: &str) -> u16 {
    return part_x(input).0;
}

fn part2(input: &str) -> u16 {
    return part_x(input).1;
}

fn part_x(input: &str) -> (u16, u16) {
    let lines = input.lines();
    let mut order: HashMap<u16, Vec<u16>> = HashMap::new();
    let mut counter: u16 = 0;
    let mut invalid_counter: u16 = 0;

    for line in lines {
        if line.contains('|') {
            if let Some((before, after)) = line.split_once('|') {
                let value = before.trim().parse::<u16>().unwrap();
                let key = after.trim().parse::<u16>().unwrap();
                order.entry(key).or_insert_with(Vec::new).push(value);
            }
        } else {
            let mut ok = true;
            if let Ok(numbers) = line
                .trim()
                .split(',')
                .into_iter()
                .map(|v| v.parse::<u16>())
                .collect::<Result<Vec<u16>, _>>()
            {
                for i in 0..numbers.len() {
                    for j in i..numbers.len() {
                        if let Some(values) = order.get(&numbers[i]) {
                            if values.contains(&numbers[j]) {
                                ok = false;
                            }
                        }
                    }
                }
                if ok {
                    counter += numbers[numbers.len() / 2];
                } else {
                    let mut m = numbers.clone();
                    m.sort_by(|a, b| {
                        if let Some(values) = order.get(a) {
                            if values.contains(b) {
                                Ordering::Greater
                            } else {
                                Ordering::Less
                            }
                        } else {
                            Ordering::Less
                        }
                    });
                    invalid_counter += m[m.len() / 2];
                }
            }
        }
    }

    return (counter, invalid_counter);
}

#[test]
fn test_part1_sample() {
    let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    let result = part_x(input);
    assert_eq!(result.0, 143);
    assert_eq!(result.1, 123);
}
