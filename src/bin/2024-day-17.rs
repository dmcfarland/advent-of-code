use advent_of_code::*;
use regex::Regex;

fn main() {
    let input = read_input(2024, 17);
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2: {}", part2_result);
}

fn part1(input: &str) -> String {
    let re =
        Regex::new(r"(?m)Register A: (\d*)\nRegister B: (\d*)\nRegister C: (\d*)\n\nProgram: (.*)")
            .unwrap();

    let (_, [reg_a, reg_b, reg_c, prog]) = re.captures(input).unwrap().extract();

    let (reg_a, reg_b, reg_c, prog) = (
        reg_a.parse::<u64>().unwrap(),
        reg_b.parse::<u64>().unwrap(),
        reg_c.parse::<u64>().unwrap(),
        prog.split(',')
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<u64>>(),
    );

    run_program(reg_a, reg_b, reg_c, &prog)
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn run_program(mut reg_a: u64, mut reg_b: u64, mut reg_c: u64, prog: &Vec<u64>) -> Vec<u64> {
    let mut output: Vec<u64> = Vec::new();

    let mut next = 0;
    while next < prog.len() - 1 {
        let operator = prog[next];
        let operand = prog[next + 1];

        let combi = match operand {
            0..=3 => operand,
            4 => reg_a,
            5 => reg_b,
            6 => reg_c,
            _ => 0,
        };

        // 0 adv a = a / cm^2
        // 1 bxl b=b xor L
        // 2 bst b=cm % 8
        // 3 jnz if a > 0 jump by L
        // 4 bxc b = b xor c
        // 5 out print cm % 8
        // 6 bdv a = b / cm^2
        // 7 bdv a = c / cm^2
        match operator {
            0 => reg_a /= 2_u64.pow(combi.try_into().unwrap()),
            1 => reg_b ^= operand,
            2 => reg_b = combi % 8,
            3 => {
                if reg_a != 0 {
                    next = operand as usize;
                    continue;
                }
            }
            4 => reg_b ^= reg_c,
            5 => output.push(combi % 8),
            6 => reg_b = reg_a / 2_u64.pow(combi.try_into().unwrap()),
            7 => reg_c = reg_a / 2_u64.pow(combi.try_into().unwrap()),
            _ => (),
        }
        next += 2
    }

    output
}

fn part2(input: &str) -> u64 {
    let re =
        Regex::new(r"(?m)Register A: (\d*)\nRegister B: (\d*)\nRegister C: (\d*)\n\nProgram: (.*)")
            .unwrap();

    let (_, [_, _, _, prog_string]) = re.captures(input).unwrap().extract();

    let prog = prog_string
        .split(',')
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut result = Vec::new();

    // Guess that this will be below quine seed.
    let mut i = 10_u64.pow((prog_string.len() / 2) as u32 - 1);
    let mut skip = i / 1000;

    let mut overlap = 0;
    let mut record_overlap = 0;
    while overlap < prog.len() {
        result = run_program(i, 0, 0, &prog);
        overlap = matching(&result, &prog);
        if overlap > record_overlap {
            skip = (skip / 100).max(1);
        }
        record_overlap = overlap;
        // println!(
        //     "{} = {:?} len:{} match:{} skip:{}",
        //     i,
        //     result,
        //     result.len() == prog.len(),
        //     overlap,
        //     skip,
        // );
        i += skip;
    }

    if result == prog {
        return i - 1;
    }
    0
}

fn matching<T: PartialEq>(vec1: &Vec<T>, vec2: &Vec<T>) -> usize {
    if vec1.len() != vec2.len() {
        return 0;
    }

    let mut count = 0;
    for i in (0..vec1.len()).rev() {
        if vec1[i] == vec2[i] {
            count += 1;
        } else {
            break;
        }
    }
    count
}

#[test]
fn test_part1_sample() {
    let input = r#"Register A: 729
    Register B: 0
    Register C: 0

    Program: 0,1,5,4,3,0"#;

    let input = left_trim_lines(input);
    let result = part1(&input);
    assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
}

#[test]
fn test_part2_sample() {
    let input = r#"Register A: 0
    Register B: 0
    Register C: 0

    Program: 0,3,5,4,3,0"#;

    let input = left_trim_lines(input);
    let result = part2(&input);
    assert_eq!(result, 117440);
}
