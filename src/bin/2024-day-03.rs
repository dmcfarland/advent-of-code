use advent_of_code::*;
use regex::Regex;

// Looked up regex syntax
// Star 1 - 21:21
// Star 2 - 51:09

fn main() {
    let input = read_input(2024, 3);
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2: {}", part2_result);
}

fn count_segments(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total = 0;
    for (_, [left, right]) in re.captures_iter(input).map(|x| x.extract()) {
        let seg = left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap();
        total += seg;
    }
    return total;
}

fn part1(input: &str) -> u32 {
    return count_segments(input);
}

fn part2(input: &str) -> u32 {
    let re = Regex::new(r"\n").unwrap();
    let input_without_newlines = re.replace_all(input, "");

    let re = Regex::new(r"(?m)don't\(\).*?(do\(\)|$)").unwrap();
    let input_without_donot_blocks = re.replace_all(&input_without_newlines, "").to_string();

    return count_segments(&input_without_donot_blocks);
}

#[test]
fn test_part1_sample() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let result = part1(input);
    assert_eq!(result, 161);
}

#[test]
fn test_part2_sample() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let result = part2(&input);
    assert_eq!(result, 48);
}
