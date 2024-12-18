use advent_of_code::*;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let input = read_input(2024, 18);
    let part1_result = part1(&input, 70, 1024);
    println!("Part 1: {}", part1_result);

    // let part2_result = part2(&input);
    // println!("Part 2: {}", part2_result);
}

fn part2(input: &str) -> u64 {
    todo!()
}

fn part1(input: &str, grid_size: usize, num_bytes: usize) -> usize {
    let mut grid = vec![vec!['.'; grid_size + 1]; grid_size + 1];
    let re = Regex::new(r"(\d*),(\d*)").unwrap();
    let bad_bytes: Vec<(usize, usize)> = re
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [x, y])| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .collect();

    for i in 0..num_bytes {
        let b = bad_bytes[i];

        grid[b.1][b.0] = '#';
    }

    print_grid(&grid);

    let moves = vec![(0, 1), (0, -1_i32), (1, 0), (-1_i32, 0)];
    let mut overlay_grid = vec![vec!['.'; grid_size + 1]; grid_size + 1];

    let mut cur = vec![(0, 0)];
    let mut prev: Vec<(usize, usize)> = Vec::new();
    let mut steps = 0;
    while let Some(&last) = cur.last() {
        if last == (grid_size, grid_size) {
            break;
        }
        steps += 1;

        cur = cur
            .iter()
            .flat_map(|f| {
                let r = moves.iter().filter_map(|m| {
                    let new_y = f.1 as i32 + m.1;
                    let new_x = f.0 as i32 + m.0;

                    if new_y >= 0
                        && new_y <= grid_size as i32
                        && new_x >= 0
                        && new_x <= grid_size as i32
                    {
                        let next = (new_x as usize, new_y as usize);
                        if grid[next.1][next.0] != '#' && !prev.contains(&next) {
                            println!("{:?} => {:?}", f.clone(), next);
                            Some(next)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                });
                r
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        prev.append(cur.clone().as_mut());
    }

    for p in prev {
        overlay_grid[p.1][p.0] = 'O';
    }
    print_grid(&overlay_grid);

    return steps;
}

#[test]
fn test_part1_sample() {
    let input = r#"5,4
    4,2
    4,5
    3,0
    2,1
    6,3
    2,4
    1,5
    0,6
    3,3
    2,6
    5,1
    1,2
    5,5
    2,5
    6,5
    1,4
    0,4
    6,4
    1,1
    6,1
    1,0
    0,5
    1,6
    2,0"#;

    let input = left_trim_lines(input);
    let result = part1(&input, 6, 12);
    assert_eq!(result, 22);
}
