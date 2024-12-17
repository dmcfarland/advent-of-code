use advent_of_code::*;

fn main() {
    let input = read_input(2024, 15);
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&part2_double_map(&input));
    println!("Part 2: {}", part2_result);
}

fn part1(input: &str) -> usize {
    let grid = input
        .lines()
        .filter(|l| l.starts_with("#"))
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let moves = input
        .lines()
        .filter(|l| !l.starts_with("#"))
        .flat_map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<char>>();

    let mut robot_location = (0, 0);
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '@' {
                robot_location = (r, c);
            }
        }
    }
    let mut overlay = grid.clone();

    for attempt_move in moves {
        if attempt_move == '<' {
            let x = robot_location.0;
            let mut space = 0;
            for y in (0..robot_location.1).rev() {
                if overlay[x][y] == '#' {
                    break;
                } else if overlay[x][y] == '.' {
                    space = y;
                    break;
                }
            }

            if space != 0 {
                for i in space..robot_location.1 {
                    overlay[x][i] = overlay[x][i + 1]
                }
                overlay[x][robot_location.1] = '.';
                robot_location.1 -= 1;
            }
        } else if attempt_move == '>' {
            let x = robot_location.0;
            let mut space = 0;
            for y in robot_location.1..overlay[x].len() - 1 {
                if overlay[x][y] == '#' {
                    break;
                } else if overlay[x][y] == '.' {
                    space = y;
                    break;
                }
            }

            if space != 0 {
                for i in (robot_location.1..space + 1).rev() {
                    overlay[x][i] = overlay[x][i - 1]
                }
                overlay[x][robot_location.1] = '.';
                robot_location.1 += 1;
            }
        } else if attempt_move == '^' {
            let y = robot_location.1;
            let mut space = 0;
            for x in (0..robot_location.0).rev() {
                if overlay[x][y] == '#' {
                    break;
                } else if overlay[x][y] == '.' {
                    space = x;
                    break;
                }
            }

            if space != 0 {
                for i in space..robot_location.0 {
                    overlay[i][y] = overlay[i + 1][y]
                }
                overlay[robot_location.0][y] = '.';
                robot_location.0 -= 1;
            }
        } else if attempt_move == 'v' {
            let y = robot_location.1;
            let mut space = 0;
            for x in robot_location.0..overlay.len() - 1 {
                if overlay[x][y] == '#' {
                    break;
                } else if overlay[x][y] == '.' {
                    space = x;
                    break;
                }
            }

            if space != 0 {
                for i in (robot_location.0..space + 1).rev() {
                    overlay[i][y] = overlay[i - 1][y]
                }
                overlay[robot_location.0][y] = '.';
                robot_location.0 += 1;
            }
        }
    }

    let mut sum = 0;
    for r in 0..overlay.len() {
        for c in 0..overlay[0].len() {
            if overlay[r][c] == 'O' {
                sum += r * 100 + c;
            }
        }
    }

    sum
}

fn part2_double_map(input: &str) -> String {
    let mut doubled = String::from(input);
    doubled = doubled.replace("#", "##");
    doubled = doubled.replace("O", "[]");
    doubled = doubled.replace(".", "..");
    doubled = doubled.replace("@", "@.");
    doubled
}

fn part2(input: &str) -> usize {
    let grid = input
        .lines()
        .filter(|l| l.starts_with("#"))
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let moves = input
        .lines()
        .filter(|l| !l.starts_with("#"))
        .flat_map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<char>>();

    let mut robot_location = (0, 0);
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '@' {
                robot_location = (r, c);
            }
        }
    }
    let mut overlay = grid.clone();

    for attempt_move in moves {
        if attempt_move == '<' {
            let x = robot_location.0;
            let mut space = 0;
            for y in (0..robot_location.1).rev() {
                if overlay[x][y] == '#' {
                    break;
                } else if overlay[x][y] == '.' {
                    space = y;
                    break;
                }
            }
            if space != 0 {
                for i in space..robot_location.1 {
                    overlay[x][i] = overlay[x][i + 1]
                }
                overlay[x][robot_location.1] = '.';
                robot_location.1 -= 1;
            }
        } else if attempt_move == '>' {
            let x = robot_location.0;
            let mut space = 0;
            for y in robot_location.1..overlay[x].len() - 1 {
                if overlay[x][y] == '#' {
                    break;
                } else if overlay[x][y] == '.' {
                    space = y;
                    break;
                }
            }
            if space != 0 {
                for i in (robot_location.1..space + 1).rev() {
                    overlay[x][i] = overlay[x][i - 1]
                }
                overlay[x][robot_location.1] = '.';
                robot_location.1 += 1;
            }
        } else if attempt_move == '^' {
            let x = robot_location.0;
            let y = robot_location.1;

            let mut row_moves: Vec<Vec<usize>> = Vec::new();

            row_moves.push(vec![y]);
            'all: for r in (0..robot_location.0).rev() {
                let last = row_moves.last().unwrap();
                let mut current = Vec::new();
                for &c in last {
                    if overlay[r][c] == '#' {
                        row_moves = Vec::new();
                        break 'all;
                    }
                    if overlay[r][c] == '[' {
                        current.push(c);
                        current.push(c + 1);
                    } else if overlay[r][c] == ']' {
                        current.push(c);
                        current.push(c - 1);
                    }
                }

                current.sort();
                current.dedup();
                row_moves.push(current);
            }

            let row_shifts = row_moves.iter().filter(|p| !p.is_empty()).count();

            if row_shifts > 0 {
                for r in 0..row_moves.len() {
                    for c in &row_moves[row_moves.len() - r - 1] {
                        overlay[r - 1][*c] = overlay[r][*c];
                        overlay[r][*c] = '.';
                    }
                }

                robot_location.0 -= 1;
                row_moves[0].iter().for_each(|c| overlay[x][*c] = '.');
            }
        } else if attempt_move == 'v' {
            let x = robot_location.0;
            let y = robot_location.1;

            let mut row_moves: Vec<Vec<usize>> = Vec::new();

            row_moves.push(vec![y]);
            'all: for r in (robot_location.0 + 1)..overlay.len() {
                let last = row_moves.last().unwrap();
                let mut current = Vec::new();
                for &c in last {
                    if overlay[r][c] == '#' {
                        row_moves = Vec::new();
                        break 'all;
                    }
                    if overlay[r][c] == '[' {
                        current.push(c);
                        current.push(c + 1);
                    } else if overlay[r][c] == ']' {
                        current.push(c);
                        current.push(c - 1);
                    }
                }

                current.sort();
                current.dedup();
                row_moves.push(current);
            }

            let row_shifts = row_moves.iter().filter(|p| !p.is_empty()).count();

            if row_shifts > 0 {
                for rx in 0..row_moves.len() {
                    let r = grid.len() - rx;
                    for c in &row_moves[row_moves.len() - rx - 1] {
                        overlay[r][*c] = overlay[r - 1][*c];
                        overlay[r - 1][*c] = '.';
                    }
                }

                robot_location.0 += 1;
                row_moves[0].iter().for_each(|c| overlay[x][*c] = '.');
            }
        }
    }

    let mut sum = 0;
    for r in 0..overlay.len() {
        for c in 0..overlay[0].len() {
            if overlay[r][c] == '[' {
                sum += r * 100 + c;
            }
        }
    }

    sum
}

#[test]
fn test_part1_sample() {
    let input = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
    let result = part1(input);
    println!("Counts {:?}", result);
    assert_eq!(result, 10092);
}

#[test]
fn test_part2_sample() {
    let input = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
    let result = part2(&part2_double_map(input));
    println!("Counts {:?}", result);
    assert_eq!(result, 9021);
}
