use advent_of_code::*;

fn main() {
    let input = read_input(2024, 6);
    let part1_result = part1(&as_grid(&input));
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&as_grid(&input));
    println!("Part 2: {}", part2_result);
}

#[derive(Clone, Debug)]
struct Grid {
    matrix: Vec<Vec<char>>,
    rows: usize,
    columns: usize,
    start_pos: (usize, usize),
}

fn as_grid(input: &str) -> Grid {
    let lines = input.lines();
    let grid = lines
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let row_count = grid.len();
    let column_count = grid[0].len();
    let mut initial: (usize, usize) = (0, 0);

    for r in 0..row_count {
        for c in 0..column_count {
            if grid[r][c] == '^' {
                initial = (r, c);
            }
        }
    }

    return Grid {
        matrix: grid,
        rows: row_count,
        columns: column_count,
        start_pos: initial,
    };
}

fn part1(grid: &Grid) -> u16 {
    return part_x(&grid).0;
}

fn part_x(grid: &Grid) -> (u16, u16) {
    let mut overlay_grid = vec![vec![' '; grid.columns]; grid.rows];

    let mut r = grid.start_pos.0 as i16;
    let mut c = grid.start_pos.1 as i16;
    let mut steps = 0;
    let mut incrementor = 0;
    let mut direction = Direction::UP;
    while r < grid.rows as i16 && r >= 0 && c < grid.columns as i16 && c >= 0 && incrementor < 10000
    {
        let x = r as usize;
        let y = c as usize;

        let switch = grid.matrix[x][y] == '#';
        if switch {
            overlay_grid[x][y] = '#';
            match direction {
                Direction::UP => r += 1,
                Direction::RIGHT => c -= 1,
                Direction::DOWN => r -= 1,
                Direction::LEFT => c += 1,
            }
            direction = match direction {
                Direction::UP => Direction::RIGHT,
                Direction::RIGHT => Direction::DOWN,
                Direction::DOWN => Direction::LEFT,
                Direction::LEFT => Direction::UP,
            };
            overlay_grid[r as usize][c as usize] = '+';
        } else {
            incrementor += 1;
            if overlay_grid[x][y] == ' ' {
                overlay_grid[x][y] = match direction {
                    Direction::UP => '|',
                    Direction::RIGHT => '-',
                    Direction::DOWN => '|',
                    Direction::LEFT => '-',
                };
                steps += 1;
            }
        }

        match direction {
            Direction::UP => r -= 1,
            Direction::RIGHT => c += 1,
            Direction::DOWN => r += 1,
            Direction::LEFT => c -= 1,
        }
    }
    return (steps, incrementor);
}

fn part2(grid: &Grid) -> u16 {
    let mut loops = 0;
    for r in 0..grid.rows {
        for c in 0..grid.columns {
            let mut t_grid = grid.clone();
            t_grid.matrix[r][c] = '#';
            let results = part_x(&t_grid).1;
            // Assumes in loop once we hit 10000
            if results == 10000 {
                loops += 1;
            }
        }
    }

    return loops;
}

#[derive(Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        let grid = as_grid(&input);
        let result = part1(&grid);
        println!("Counts {:?}", result);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_part2() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        let grid = as_grid(&input);
        let result = part2(&grid);
        println!("Counts {:?}", result);
        assert_eq!(result, 6);
    }
}
