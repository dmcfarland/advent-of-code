use advent_of_code::*;
use std::collections::HashSet;

fn main() {
    let input = read_input(2024, 12);
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2: {}", part2_result);
}

fn part2(input: &str) -> u64 {
    todo!()
}

fn potential_next_locations(
    grid: &Vec<Vec<char>>,
    current: &(usize, usize),
) -> Vec<(usize, usize)> {
    let moves: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    let next_locations = moves
        .iter()
        .flat_map(|m| {
            let next = ((current.0 as isize + m.0), (current.1 as isize + m.1));
            if next.0 >= 0
                && next.0 < grid.len() as isize
                && next.1 >= 0
                && next.1 < grid[0].len() as isize
            {
                Some((next.0 as usize, next.1 as usize))
            } else {
                None
            }
        })
        .collect();
    return next_locations;
}

fn part1(input: &str) -> u64 {
    let grid = as_grid_of_char(input);
    let mut processed: Vec<(usize, usize)> = Vec::new();

    let mut cost = 0_u64;

    grid.iter().enumerate().for_each(|(r, row)| {
        row.iter().enumerate().for_each(|(c, cell)| {
            let current_location = (r, c);
            if !processed.contains(&current_location) {
                let mut block: Vec<(usize, usize)> = Vec::new();
                let mut current_scan: Vec<(usize, usize)> = vec![(r, c)];
                while !current_scan.iter().all(|s| block.contains(s)) {
                    block.append(current_scan.clone().as_mut());
                    processed.append(current_scan.clone().as_mut());
                    current_scan = current_scan
                        .iter()
                        .flat_map(|csi| {
                            potential_next_locations(&grid, &(csi.0, csi.1))
                                .iter()
                                .filter_map(|pnl| {
                                    if !processed.contains(&pnl) && grid[pnl.0][pnl.1] == *cell {
                                        Some(*pnl)
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<HashSet<_>>()
                        .into_iter()
                        .collect();
                }

                let perimeter: usize = block
                    .iter()
                    .map(|cell| {
                        let touches = potential_next_locations(&grid, &(cell.0, cell.1))
                            .iter()
                            .filter(|pt| block.contains(pt))
                            .count();
                        4 - touches
                    })
                    .sum();

                println!(
                    "Cell: {}, perimeter: {}, block_len:{}, blocks: {:?}",
                    cell,
                    perimeter,
                    block.len(),
                    block
                );
                cost += (perimeter * block.len()) as u64;
            }
        });
    });

    return cost;
}

#[test]
fn test_part1_sample1() {
    let input = r#"AAAA
    BBCD
    BBCC
    EEEC"#;
    let result = part1(&left_trim_lines(input));
    assert_eq!(result, 140);
}

// #[test]
// fn test_part1_sample2() {
//     let input = r#"OOOOO
//     OXOXO
//     OOOOO
//     OXOXO
//     OOOOO"#;
//     let result = part1(&left_trim_lines(input));
//     assert_eq!(result, 772);
// }

// #[test]
// fn test_part1_sample3() {
//     let input = r#"RRRRIICCFF
//     RRRRIICCCF
//     VVRRRCCFFF
//     VVRCCCJFFF
//     VVVVCJJCFE
//     VVIVCCJJEE
//     VVIIICJJEE
//     MIIIIIJJEE
//     MIIISIJEEE
//     MMMISSJEEE"#;
//     let result = part1(&left_trim_lines(input));
//     assert_eq!(result, 1930);
// }
