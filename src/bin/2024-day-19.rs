use advent_of_code::*;

fn main() {
    let input = read_input(2024, 19);
    let part1_result = part1(&input);
    println!("Part 1: {:?}", part1_result);

    // let part2_result = part2(&input);
    // println!("Part 2: {}", part2_result);
}

fn part1(input: &str) -> u64 {
    let mut towels: Vec<String> = input
        .lines()
        .next()
        .unwrap_or("")
        .split(", ")
        .map(String::from)
        .collect();

    towels.sort_by(|a, b| b.len().cmp(&a.len()));
    println!("Towels: {:?}", towels);

    let mut patterns: Vec<String> = Vec::new();
    for line in input.lines().skip(2) {
        patterns.push(line.to_string());
    }

    let possible = patterns
        .iter()
        .filter(|pattern| {
            let mut m = pattern.to_string();
            for i in 0..towels.len() {
                m = pattern.to_string();
                // let i = 0;
                let tt = towels.iter().skip(i);
                for towel in tt {
                    let om = m.clone();
                    m = m.replace(towel, "");
                    if om != m {
                        println!(
                            "pattern: {:?} towel: {:?} m: {:?} => {:?}",
                            pattern, towel, om, m
                        );
                    }
                    if m.is_empty() {
                        break;
                    }
                }
                if m.is_empty() {
                    break;
                }
            }
            println!("pattern: {:?} m: {:?}", pattern, m);
            m == ""
        })
        .count();

    // println!("{:?}", towels);
    // println!("{:?}", patterns);
    // println!("{}", possible);

    return possible as u64;
}

#[test]
fn test_part1_sample() {
    let input = r#"r, wr, b, g, bwu, rb, gb, br

    brwrr
    bggr
    gbbr
    rrbgbr
    ubwu
    bwurrg
    brgr
    bbrgwb"#;

    let input = left_trim_lines(input);
    let result = part1(&input);
    assert_eq!(result, 6);
}

const EXAMPLE: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
