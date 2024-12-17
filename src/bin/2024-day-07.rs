use advent_of_code::*;

fn main() {
    let input = read_input(2024, 7);
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2: {}", part2_result);
}

fn part1(input: &str) -> u64 {
    return part_x(input).0;
}

fn part2(input: &str) -> u64 {
    return part_x(input).1;
}

fn part_x(input: &str) -> (u64, u64) {
    let mut result: u64 = 0;
    let mut sum: u64 = 0;
    let lines = input.lines();
    for line in lines {
        let (test_value, numbers) = line.split_once(": ").unwrap();

        // println!("{:?} {:?}", test_value, numbers);
        let test = test_value.parse::<u64>().unwrap();
        let values = numbers
            .split(" ")
            .into_iter()
            .map(|num| num.parse::<u64>())
            .collect::<Result<Vec<u64>, _>>()
            .expect("Should have been able to parse the numbers");
        // println!("{:?}", values);

        let max_operations = 1 << values.len() - 1;

        'outer: for op1 in 0..max_operations {
            for op2 in 0..max_operations {
                let mut total: u64 = values[0] as u64;
                // print!("{} ", values[0]);

                for i in 1..values.len() {
                    let f = 1 << (i - 1) as u16;

                    // print!("[{:#b} {:#b} = {:#b} {:#b}]", f, op1, f & op1, f & op2);
                    if f & op1 == f {
                        if i < values.len() {
                            // print!("* ");
                        }
                        total *= values[i] as u64;
                    } else if f & op2 == f {
                        if i < values.len() {
                            // print!("|| ");
                        }
                        total = ([total.to_string(), values[i].to_string()].join(""))
                            .parse::<u64>()
                            .unwrap();
                    } else {
                        if i < values.len() {
                            // print!("+ ");
                        }
                        total += values[i] as u64;
                    }

                    // print!("{} ", values[i]);
                }
                // println!("= {}", total);
                if test == total {
                    result += 1;
                    sum += test as u64;
                    break 'outer;
                }
            }
        }
    }

    return (result, sum);
}

#[test]
fn test_part1() {
    let input = r#"190: 10 19
    3267: 81 40 27
    83: 17 5
    156: 15 6
    7290: 6 8 6 15
    161011: 16 10 13
    192: 17 8 14
    21037: 9 7 18 13
    292: 11 6 16 20"#;
    let result = part_x(&input);
    println!("Counts {:?}", result);
    assert_eq!(result.0, 6);
    assert_eq!(result.1, 11387);
}
