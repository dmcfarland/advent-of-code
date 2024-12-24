use advent_of_code::*;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = read_input(2024, 24);
    let part1_result = part1(&input);
    println!("Part 1: {:?}", part1_result);
}

fn part1(input: &str) -> usize {
    let input_re = Regex::new(r"(\w{3}): (1|0)").unwrap();
    let gate_re = Regex::new(r"(\w{3}) (AND|OR|XOR) (\w{3}) -> (\w{3})").unwrap();

    let mut inputs: HashMap<&str, bool> = input_re
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [input, v])| (input, if v == "0" { false } else { true }))
        .collect();

    let gates: Vec<(&str, &str, &str, &str)> = gate_re
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [a1, op, a2, t])| (a1, op, a2, t))
        .collect();

    let mut all_calculated = false;
    while !all_calculated {
        all_calculated = true;
        for gate in &gates {
            if !inputs.contains_key(&gate.3) {
                all_calculated = false;
                if let (Some(&i1), Some(&i2)) = (inputs.get(&gate.0), inputs.get(&gate.2)) {
                    let result = match gate.1 {
                        "AND" => i1 & i2,
                        "OR" => i1 | i2,
                        "XOR" => i1 ^ i2,
                        _ => panic!("Unknown gate operation"),
                    };
                    inputs.insert(gate.3, result);
                }
            }
        }
    }

    let mut z: Vec<(&str, bool)> = inputs
        .iter()
        .filter(|e| e.0.starts_with("z"))
        .map(|(s, b)| (*s, *b))
        .collect();

    z.sort_by(|a, b| a.0.cmp(b.0));
    z.reverse();

    let mut total = 0;
    for (_, value) in &z {
        total *= 2;
        total += match value {
            true => 1,
            false => 0,
        };
    }

    println!("ZInputs: {} {:?}", total, z);

    return total;
}

const SAMPLE_1: &str = "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

#[test]
fn part1_sample1() {
    let input = left_trim_lines(SAMPLE_1);
    assert_eq!(part1(&input), 4);
}

const SAMPLE_2: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

#[test]
fn part1_sample2() {
    let input = left_trim_lines(SAMPLE_2);
    assert_eq!(part1(&input), 2024);
}
