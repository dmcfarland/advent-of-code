use advent_of_code::*;
use std::collections::HashMap;

fn main() {
    let input = read_input(2024, 10);
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);

    // TODO:
    // let part2_result = part2(&input);
    // println!("Part 2: {}", part2_result);
}

fn part1(input: &str) -> u32 {
    let grid = as_grid_of_char(input);
    let mut trails: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] != '.' {
                trails
                    .entry(grid[x][y].to_digit(10).unwrap())
                    .or_default()
                    .push((x as u32, y as u32));
            }
        }
    }

    println!("Trails: {:?}", trails);
    let mut trailheads_count = 0;
    for trailhead in trails[&0].clone() {
        let mut routes: Vec<Vec<(u32, u32)>> = Vec::new();
        routes.push(vec![trailhead]);
        for n in 1..10 {
            let last = routes.last().unwrap();
            let next: Vec<(u32, u32)> = trails[&n]
                .iter()
                .filter(|s| {
                    last.iter().any(|f| {
                        (s.0.abs_diff(f.0) == 1 && s.1.abs_diff(f.1) == 0)
                            || (s.1.abs_diff(f.1) == 1 && s.0.abs_diff(f.0) == 0)
                    })
                })
                .cloned()
                .collect();
            // routes.push(next);
            // }
            if n == 9 {
                println!("{:?} {}", trailhead, next.len());
                trailheads_count += next.len();
            }
            routes.push(next);
        }
    }

    trailheads_count as u32
}

#[test]
fn test_part1_sample() {
    let input = r#"89010123
    78121874
    87430965
    96549874
    45678903
    32019012
    01329801
    10456732"#;
    let result = part1(input);
    println!("Counts {:?}", result);
    assert_eq!(result, 36);
}
