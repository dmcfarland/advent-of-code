use advent_of_code::*;

// 458 - too low

fn main() {
    let input = read_input(2025, 1);
    // let part1_result = part1(&input);
    // println!("Part 1: {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2: {}", part2_result);
}

fn part1(input: &str) -> u32 {
    let mut index: i16 = 50;
    let mut password: u32 = 0;

    input.lines().for_each(|line| {
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let inc = chars.as_str().parse::<i16>().unwrap() % 100;

        if dir == 'L' {
            index -= inc;
        } else {
            index += inc;
        }
        if index < 0 {
            index = 100 + index;
        }
        if index > 99 {
            index = index - 100;
        }

        if index == 0 {
            password += 1;
        }

        println!("{:?}{}: {}", dir, inc, index);
    });

    return password;
}

// 6796,6790 - too high
// 5000 - too low
fn part2(input: &str) -> u32 {
    let mut index: i32 = 50;
    let mut password: u32 = 0;
    let mut i = 0;
    input.lines().for_each(|line| {
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let incx = chars.as_str().parse::<i32>().unwrap();
        let inc = incx % 100;

        if dir == 'L' {
            index -= inc;
        } else {
            index += inc;
        };
        if index < 0 {
            index = 100 + index;
            password += 1;
        }
        if index > 99 {
            index = index - 100;
            password += 1;
        }

        password += (incx / 100) as u32;

        println!("{}|{:?}{}: {} ({})", i, dir, incx, index, password);
        i += 1;
    });

    return password;
}

#[test]
fn test_part1_sample() {
    let input = r#"L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82"#;

    let input = left_trim_lines(input);
    let result = part1(&input);
    assert_eq!(result, 3);
}

#[test]
fn test_part2_sample() {
    let input = r#"L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82"#;

    let input = left_trim_lines(input);
    let result = part2(&input);
    assert_eq!(result, 6);
}
