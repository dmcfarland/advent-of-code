use advent_of_code::*;
use regex::Regex;

// Rusty with Linear Algebra, followed https://www.reddit.com/r/adventofcode/comments/1hd7irq/2024_day_13_an_explanation_of_the_mathematics/

fn main() {
    let input = read_input(2024, 13);
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2: {:?}", part2_result);
}

#[derive(Debug)]
struct ClawMachine {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64),
}

impl ClawMachine {
    fn cheapest_combination(&self) -> u64 {
        let a = self.a;
        let b = self.b;
        let prize = self.prize;
        let det = a.0 as i64 * b.1 as i64 - a.1 as i64 * b.0 as i64;

        let xa = ((prize.0 as i64 * b.1 as i64 - prize.1 as i64 * b.0 as i64) as i64 / det) as u64;
        let xb = ((a.0 as i64 * prize.1 as i64 - a.1 as i64 * prize.0 as i64) as i64 / det) as u64;
        println!(
            "{} {} {} {} {} {} {} {}",
            a.0, xa, b.0, xb, a.1, xa, b.1, xb
        );
        if (
            (a.0 as u128 * xa as u128 + b.0 as u128 * xb as u128) as u64,
            (a.1 as u128 * xa as u128 + b.1 as u128 * xb as u128) as u64,
        ) == (prize.0, prize.1)
        {
            let cost = xa * 3 + xb;
            println!("OK Machine: {:?} - cost: {}", self, cost);
            cost
        } else {
            println!("KO Machine: {:?}", self);
            0
        }
    }
}

fn part1(input: &str) -> u64 {
    let re = Regex::new(
        r"(?m)^Button A: X\+(\d*), Y\+(\d*)\nButton B: X\+(\d*), Y\+(\d*)\nPrize: X=(\d*), Y=(\d*)",
    )
    .unwrap();

    let claw_machines =
        re.captures_iter(input)
            .map(|m| m.extract())
            .map(|(_, [ax, ay, bx, by, px, py])| ClawMachine {
                a: (ax.parse().unwrap(), ay.parse().unwrap()),
                b: (bx.parse().unwrap(), by.parse().unwrap()),
                prize: (px.parse().unwrap(), py.parse().unwrap()),
            });

    let mut sum = 0;
    for claw_machine in claw_machines {
        sum += claw_machine.cheapest_combination();
    }

    sum
}

fn part2(input: &str) -> u64 {
    let re = Regex::new(
        r"(?m)^Button A: X\+(\d*), Y\+(\d*)\nButton B: X\+(\d*), Y\+(\d*)\nPrize: X=(\d*), Y=(\d*)",
    )
    .unwrap();

    let claw_machines =
        re.captures_iter(input)
            .map(|m| m.extract())
            .map(|(_, [ax, ay, bx, by, px, py])| ClawMachine {
                a: (ax.parse().unwrap(), ay.parse().unwrap()),
                b: (bx.parse().unwrap(), by.parse().unwrap()),
                prize: (
                    px.parse::<u64>().unwrap() + 10000000000000,
                    py.parse::<u64>().unwrap() + 10000000000000,
                ),
            });

    let mut sum = 0;
    for claw_machine in claw_machines {
        sum += claw_machine.cheapest_combination();
    }

    sum
}

#[test]
fn test_part1() {
    let input = r#"Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400

    Button A: X+26, Y+66
    Button B: X+67, Y+21
    Prize: X=12748, Y=12176

    Button A: X+17, Y+86
    Button B: X+84, Y+37
    Prize: X=7870, Y=6450

    Button A: X+69, Y+23
    Button B: X+27, Y+71
    Prize: X=18641, Y=10279"#;
    let result = part1(&left_trim_lines(input));
    println!("Counts {:?}", result);
    assert_eq!(result, 480);
}

#[test]
fn test_part2() {
    let input = r#"Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400

    Button A: X+26, Y+66
    Button B: X+67, Y+21
    Prize: X=12748, Y=12176

    Button A: X+17, Y+86
    Button B: X+84, Y+37
    Prize: X=7870, Y=6450

    Button A: X+69, Y+23
    Button B: X+27, Y+71
    Prize: X=18641, Y=10279"#;
    let result = part2(&left_trim_lines(input));
    println!("Counts {:?}", result);
    assert_eq!(result, 875318608908);
}
