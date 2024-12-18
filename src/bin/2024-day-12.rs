use advent_of_code::*;
use std::{collections::HashSet, fmt::Debug};

fn main() {
    let input = read_input(2024, 12);
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2: {}", part2_result);
}

fn part2(input: &str) -> u64 {
    return partx(input, calculate_edges);
}

fn part1(input: &str) -> u64 {
    return partx(input, calculate_perimeter);
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

fn partx(input: &str, edgesFn: EdgesFunction) -> u64 {
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

                let edges = edgesFn(&grid, &block);

                println!(
                    "Cell: {}, edges: {}, block_len:{}, blocks: {:?}",
                    cell,
                    edges,
                    block.len(),
                    block
                );
                cost += (edges * block.len()) as u64;
            }
        });
    });

    return cost;
}

type EdgesFunction = fn(grid: &Vec<Vec<char>>, &Vec<(usize, usize)>) -> usize;

fn calculate_perimeter(grid: &Vec<Vec<char>>, block: &Vec<(usize, usize)>) -> usize {
    return block
        .iter()
        .map(|cell| {
            let touches = potential_next_locations(&grid, &(cell.0, cell.1))
                .iter()
                .filter(|pt| block.contains(pt))
                .count();
            4 - touches
        })
        .sum();
}

fn calculate_edges(grid: &Vec<Vec<char>>, block: &Vec<(usize, usize)>) -> usize {
    let mut block = block.clone();
    block.sort_by(|a, b| (a.0 + a.1).cmp(&(b.0 + b.1)));
    let first = block.first().unwrap();
    let last = block.last().unwrap();

    let mut full_block: Vec<(usize, usize)> = Vec::new();
    for r in first.0..=last.0 {
        for c in first.1..=last.1 {
            full_block.push((r, c));
        }
    }

    let mut empty_block = full_block.clone();

    empty_block.retain(|f| !block.contains(f));

    let add_edges: usize = empty_block
        .iter()
        .map(|f| {
            let locs = potential_next_locations(&grid, &f);
            let o = locs.iter().filter(|l| block.contains(l)).count();
            println!("{:?} => {}", f, o);
            if o > 1 {
                o
            } else {
                0
            }
        })
        .sum();
    println!("first: {:?} last: {:?} block: {:?}", first, last, block);
    println!("f_block: {:?}", full_block);
    println!("e_block: {:?}", empty_block);

    return 4 + add_edges;
}

#[test]
fn test_part2_sample1() {
    let input = r#"AAAA
    BBCD
    BBCC
    EEEC"#;
    let result = part2(&left_trim_lines(input));
    assert_eq!(result, 80);
}

#[test]
fn test_part2_sample2() {
    let input = r#"OOOOO
    OXOXO
    OOOOO
    OXOXO
    OOOOO"#;
    let result = part2(&left_trim_lines(input));
    assert_eq!(result, 436);
}

// #[test]
// fn test_part2_sample3() {
//     let input = r#"EEEEE
//     EXXXX
//     EEEEE
//     EXXXX
//     EEEEE"#;
//     let result = part2(&left_trim_lines(input));
//     assert_eq!(result, 236);
// }

// #[test]
// fn test_part2_sample4() {
//     let input = r#"AAAAAA
//     AAABBA
//     AAABBA
//     ABBAAA
//     ABBAAA
//     AAAAAA"#;
//     let result = part2(&left_trim_lines(input));
//     assert_eq!(result, 368);
// }

// #[test]
// fn test_part2_sample5() {
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
//     assert_eq!(result, 1206);
// }

#[test]
fn test_part1_sample1() {
    let input = r#"AAAA
    BBCD
    BBCC
    EEEC"#;
    let result = part1(&left_trim_lines(input));
    assert_eq!(result, 140);
}

#[test]
fn test_part1_sample2() {
    let input = r#"OOOOO
    OXOXO
    OOOOO
    OXOXO
    OOOOO"#;
    let result = part1(&left_trim_lines(input));
    assert_eq!(result, 772);
}

#[test]
fn test_part1_sample3() {
    let input = r#"RRRRIICCFF
    RRRRIICCCF
    VVRRRCCFFF
    VVRCCCJFFF
    VVVVCJJCFE
    VVIVCCJJEE
    VVIIICJJEE
    MIIIIIJJEE
    MIIISIJEEE
    MMMISSJEEE"#;
    let result = part1(&left_trim_lines(input));
    assert_eq!(result, 1930);
}
