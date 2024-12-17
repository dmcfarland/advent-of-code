use advent_of_code::*;

fn main() {
    let input = read_input(2024, 7);
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2: {}", part2_result);
}

fn part1(input: &str) -> u64 {
    let mut result: u64 = 0;
    let lines = input.lines();
    for line in lines {
        let (test_value, numbers) = line.split_once(": ").unwrap();

        // println!("{:?} {:?}", test_value, numbers);
        let test = test_value.parse::<u64>().unwrap();
        let values = numbers
            .split(" ")
            .map(|num| num.parse::<u64>())
            .collect::<Result<Vec<u64>, _>>()
            .expect("Should have been able to parse the numbers");
        // println!("{:?}", values);

        let max_operations = 1 << (values.len() - 1);

        for op in 0..max_operations {
            let mut total: u64 = values[0];
            print!("{} ", values[0]);

            for i in 1..values.len() {
                let f = 1 << (i - 1) as u16;
                let s = op as u16;

                print!("[{:#b} {:#b} = {:#b}]", f, s, f & s);
                if f & s == f {
                    if i < values.len() {
                        print!("* ");
                    }
                    total *= values[i];
                } else {
                    if i < values.len() {
                        print!("+ ");
                    }
                    total += values[i];
                }

                print!("{} ", values[i]);
            }
            println!("= {}", total);
            if test == total {
                result += 1;
                break;
            }
        }
    }

    result
}

fn part2(input: &str) -> u64 {
    let mut sum: u64 = 0;
    let lines = input.lines();
    for line in lines {
        let (test_value, numbers) = line.split_once(": ").unwrap();

        // println!("{:?} {:?}", test_value, numbers);
        let test = test_value.parse::<u64>().unwrap();
        let values = numbers
            .split(" ")
            .map(|num| num.parse::<u64>())
            .collect::<Result<Vec<u64>, _>>()
            .expect("Should have been able to parse the numbers");
        // println!("{:?}", values);

        let max_operations = 1 << (values.len() - 1);

        'outer: for op1 in 0..max_operations {
            for op2 in 0..max_operations {
                let mut total: u64 = values[0];
                // print!("{} ", values[0]);

                for i in 1..values.len() {
                    let f = 1 << (i - 1) as u16;

                    // print!("[{:#b} {:#b} = {:#b} {:#b}]", f, op1, f & op1, f & op2);
                    if f & op1 == f {
                        if i < values.len() {
                            // print!("* ");
                        }
                        total *= values[i];
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
                        total += values[i];
                    }

                    // print!("{} ", values[i]);
                }
                // println!("= {}", total);
                if test == total {
                    sum += test;
                    break 'outer;
                }
            }
        }
    }

    sum
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
    let result = part1(input);
    println!("Counts {:?}", result);
    assert_eq!(result, 3749);
}

#[test]
fn test_part2() {
    let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
    let result = part2(input);
    println!("Counts {:?}", result);
    assert_eq!(result, 11387);
}
