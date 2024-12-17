use advent_of_code::*;

// Time: 32:47 - Looked up syntax for collecting and zipping

fn main() {
    let input = read_input(2024, 1);
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2: {}", part2_result);
}

fn extract(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    input.lines().for_each(|line| {
        let elements = line
            .split("   ")
            .map(|num| num.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()
            .expect("Should have been able to parse the numbers");
        left_list.push(elements[0]);
        right_list.push(elements[1]);
    });

    left_list.sort();
    right_list.sort();
    (left_list, right_list)
}

fn part1(input: &str) -> u32 {
    let (left_list, right_list) = extract(input);
    let mut total = 0;

    for (left, right) in left_list.iter().zip(right_list.iter()) {
        total += left.abs_diff(*right);
    }

    total
}

fn part2(input: &str) -> u32 {
    let (left_list, right_list) = extract(input);
    let mut total = 0;

    for left in left_list.iter() {
        let right_count = right_list.iter().filter(|x| *x == left).count();
        total += left * right_count as u32;
    }

    total
}

#[test]
fn test_part1_sample() {
    let input = r#"3   4
    4   3
    2   5
    1   3
    3   9
    3   3"#;

    let input = left_trim_lines(input);
    let result = part1(&input);
    assert_eq!(result, 11);
}

#[test]
fn test_part2_sample() {
    let input = r#"3   4
    4   3
    2   5
    1   3
    3   9
    3   3"#;

    let input = left_trim_lines(input);
    let result = part2(&input);
    assert_eq!(result, 31);
}
