use advent_of_code::*;
use std::collections::HashMap;

fn main() {
    let input = read_input(2024, 8);
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2: {}", part2_result);
}

type AntinodesFunction = fn(&mut Vec<Vec<char>>, (usize, usize), (usize, usize)) -> usize;

fn part1(input: &str) -> usize {
    count_antinodes(input, record_antinodes_part1, false)
}

fn part2(input: &str) -> usize {
    count_antinodes(input, record_antinodes_part2, true)
}

fn count_antinodes(
    input: &str,
    record_antinodes: AntinodesFunction,
    include_current: bool,
) -> usize {
    let grid = as_grid_of_char(input);

    let mut antenna_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    let row_count = grid.len();
    let column_count = grid[0].len();

    let mut overlay_grid = vec![vec!['.'; row_count]; column_count];

    for r in 0..row_count {
        for c in 0..column_count {
            if grid[r][c] != '.' {
                antenna_map
                    .entry(grid[r][c])
                    .or_default()
                    .push((r, c));
            }
        }
    }

    let mut antinodes = 0;
    for (_, antennas) in antenna_map.iter() {
        for i in 0..antennas.iter().len() {
            for j in (i + 1)..antennas.iter().len() {
                let f = antennas[i];
                let s = antennas[j];
                antinodes += record_antinodes(&mut overlay_grid, f, s);
            }
            if include_current && overlay_grid[antennas[i].0][antennas[i].1] != '#' {
                antinodes += 1;
                overlay_grid[antennas[i].0][antennas[i].1] = '#';
            }
        }
    }

    antinodes
}

fn record_antinodes_part1(
    grid: &mut Vec<Vec<char>>,
    f: (usize, usize),
    s: (usize, usize),
) -> usize {
    let mut antinodes = 0;
    let ant1 = (
        (f.0 as i32) + ((f.0 as i32) - (s.0 as i32)),
        (f.1 as i32) + ((f.1 as i32) - (s.1 as i32)),
    );
    let ant2 = (
        (s.0 as i32) - ((f.0 as i32) - (s.0 as i32)),
        (s.1 as i32) - ((f.1 as i32) - (s.1 as i32)),
    );

    if ant1.0 >= 0 && ant1.0 < grid.len() as i32 && ant1.1 >= 0 && ant1.1 < grid[0].len() as i32 {
        if grid[ant1.0 as usize][ant1.1 as usize] != '#' {
            antinodes += 1;
        }
        grid[ant1.0 as usize][ant1.1 as usize] = '#';
    }
    if ant2.0 >= 0 && ant2.0 < grid.len() as i32 && ant2.1 >= 0 && ant2.1 < grid[0].len() as i32 {
        if grid[ant2.0 as usize][ant2.1 as usize] != '#' {
            antinodes += 1;
        }
        grid[ant2.0 as usize][ant2.1 as usize] = '#';
    }

    antinodes
}

fn record_antinodes_part2(
    grid: &mut Vec<Vec<char>>,
    f: (usize, usize),
    s: (usize, usize),
) -> usize {
    let mut antinodes = 0;
    let r_diff = (f.0 as i32) - (s.0 as i32);
    let l_diff = (f.1 as i32) - (s.1 as i32);

    for i in 1..100 {
        let ant1 = ((f.0 as i32) + (r_diff * i), (f.1 as i32) + (l_diff * i));
        let ant2 = ((s.0 as i32) - (r_diff * i), (s.1 as i32) - (l_diff * i));

        if ant1.0 >= 0 && ant1.0 < grid.len() as i32 && ant1.1 >= 0 && ant1.1 < grid[0].len() as i32
        {
            if grid[ant1.0 as usize][ant1.1 as usize] != '#' {
                antinodes += 1;
            }
            grid[ant1.0 as usize][ant1.1 as usize] = '#';
        }
        if ant2.0 >= 0 && ant2.0 < grid.len() as i32 && ant2.1 >= 0 && ant2.1 < grid[0].len() as i32
        {
            if grid[ant2.0 as usize][ant2.1 as usize] != '#' {
                antinodes += 1;
            }
            grid[ant2.0 as usize][ant2.1 as usize] = '#';
        }
    }

    antinodes
}

#[test]
fn test_part1_sample() {
    let input = r#"............
    ........0...
    .....0......
    .......0....
    ....0.......
    ......A.....
    ............
    ............
    ........A...
    .........A..
    ............
    ............"#;
    let result = part1(input);
    assert_eq!(result, 14);
}

#[test]
fn test_part2_sample() {
    let input = r#"............
    ........0...
    .....0......
    .......0....
    ....0.......
    ......A.....
    ............
    ............
    ........A...
    .........A..
    ............
    ............"#;
    let result = part2(input);
    assert_eq!(result, 34);
}
