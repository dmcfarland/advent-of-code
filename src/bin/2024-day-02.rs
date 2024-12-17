use advent_of_code::*;

// Looked up how to add tests
// Star 1 - 0:48:57
// Star 2 - 3:20:99

fn main() {
    let input = read_input(2024, 2);
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(as_lines(&input));
    println!("Part 2: {}", part2_result);
}

fn part1(input: &str) -> usize {
    let readings = as_rows_of_u32(input);

    readings
        .iter()
        .filter(|reading| {
            let deltas: Vec<u32> = reading
                .iter()
                .zip(reading.iter().skip(1))
                .map(|(cur, next)| next.abs_diff(*cur))
                .collect();
            let max = deltas.iter().max().unwrap();
            let min = deltas.iter().min().unwrap();
            (reading.iter().is_sorted() || reading.iter().rev().is_sorted())
                && *min >= 1
                && *max <= 3
        })
        .count()
}

fn part2(lines: Vec<&str>) -> u16 {
    let mut safe_count = 0;
    for line in lines {
        let readings = line
            .split(" ")
            .map(|num| num.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()
            .expect("Should have been able to parse the numbers");

        let mut breah_count = 0;

        for i in 0..readings.len() {
            let mut found_breach = false;
            let mut m = readings.clone();
            m.remove(i);
            let base_order = if m[0] > m[1] {
                Direction::Desc
            } else {
                Direction::Asc
            };

            for i in 0..m.len() - 1 {
                if has_breach(m[i], m[i + 1], base_order) {
                    breah_count += 1;
                    found_breach = true;
                    break;
                }
            }
            if !found_breach {
                breah_count = 0;
                break;
            }
        }

        if breah_count <= 1 {
            safe_count += 1;
        }
    }
    safe_count
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Direction {
    Asc,
    Desc,
}

fn has_breach(a: i32, b: i32, base_order: Direction) -> bool {
    if (a - b).abs() > 3 {
        return true;
    }
    if (a - b).abs() < 1 {
        return true;
    }
    if (a > b) && base_order != Direction::Desc {
        return true;
    }
    if (a < b) && base_order != Direction::Asc {
        return true;
    }
    false
}

#[test]
fn test_part1_sample() {
    let input = r#"7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9"#;

    let result = part1(input);
    assert_eq!(result, 2);
}

#[test]
fn test_part2_sample() {
    let input = r#"31 33 31 34 31
    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9"#;

    let lines = as_lines(input);
    let result = part2(lines);
    assert_eq!(result, 4);
}
