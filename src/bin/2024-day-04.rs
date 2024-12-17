use advent_of_code::*;

// Star 1 - 01:50:00 - My code 2614, Guessed 2618, unsure of missing 4
// Star 2 - 02:10:00

fn main() {
    let input = read_input(2024, 4);
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2: {}", part2_result);
}

fn part1(input: &str) -> u16 {
    let grid = as_grid_of_char(input);
    let row_count = grid.len();
    let column_count = grid[0].len();

    let mut count = 0;
    let mut overlay_grid = vec![vec!['.'; row_count]; column_count];

    for r in 0..row_count {
        for c in 0..column_count {
            if (c + 3) < column_count {
                let horizontal: String =
                    [grid[r][c], grid[r][c + 1], grid[r][c + 2], grid[r][c + 3]]
                        .iter()
                        .collect();

                // println!("H - {},{} {:?}", r, c, horizontal);
                if horizontal == "XMAS" || horizontal == "SAMX" {
                    count += 1;
                    overlay_grid[r][c] = grid[r][c];
                    overlay_grid[r][c + 1] = grid[r][c + 1];
                    overlay_grid[r][c + 2] = grid[r][c + 2];
                    overlay_grid[r][c + 3] = grid[r][c + 3];
                }
            }

            if (r + 3) < row_count {
                let vertical: String = [grid[r][c], grid[r + 1][c], grid[r + 2][c], grid[r + 3][c]]
                    .iter()
                    .collect();

                // println!("V - {},{} {:?}", r, c, vertical);
                if vertical == "XMAS" || vertical == "SAMX" {
                    count += 1;
                    overlay_grid[r][c] = grid[r][c];
                    overlay_grid[r + 1][c] = grid[r + 1][c];
                    overlay_grid[r + 2][c] = grid[r + 2][c];
                    overlay_grid[r + 3][c] = grid[r + 3][c];
                }
            }

            if (r + 3) < row_count && (c + 3) < column_count {
                let diagonal: String = [
                    grid[r][c],
                    grid[r + 1][c + 1],
                    grid[r + 2][c + 2],
                    grid[r + 3][c + 3],
                ]
                .iter()
                .collect();

                // println!("Dx - {},{} {:?}", r, c, diagonal);
                if diagonal == "XMAS" || diagonal == "SAMX" {
                    count += 1;
                    overlay_grid[r][c] = grid[r][c];
                    overlay_grid[r + 1][c + 1] = grid[r + 1][c + 1];
                    overlay_grid[r + 2][c + 2] = grid[r + 2][c + 2];
                    overlay_grid[r + 3][c + 3] = grid[r + 3][c + 3];
                }
            }

            if (r as i16 - 3) >= 0 && (c as i16 - 3) >= 0 {
                let diagonal: String = [
                    grid[r][c],
                    grid[r - 1][c - 1],
                    grid[r - 2][c - 2],
                    grid[r - 3][c - 3],
                ]
                .iter()
                .collect();
                // println!("Dy - {},{} {:?}", r, c, diagonal);
                if diagonal == "XMAS" || diagonal == "SAMX" {
                    count += 1;
                    overlay_grid[r][c] = grid[r][c];
                    overlay_grid[r - 1][c - 1] = grid[r - 1][c - 1];
                    overlay_grid[r - 2][c - 2] = grid[r - 2][c - 2];
                    overlay_grid[r - 3][c - 3] = grid[r - 3][c - 3];
                }
            }
        }
    }

    // print_grid(&overlay_grid);

    count
}

fn part2(input: &str) -> u16 {
    let grid = as_grid_of_char(input);
    let row_count = grid.len();
    let column_count = grid[0].len();

    let mut count = 0;
    let mut overlay_grid = vec![vec!['.'; row_count]; column_count];

    for r in 1..row_count - 1 {
        for c in 1..column_count - 1 {
            let m: String = [
                grid[r - 1][c - 1],
                grid[r - 1][c + 1],
                grid[r][c],
                grid[r + 1][c - 1],
                grid[r + 1][c + 1],
            ]
            .iter()
            .collect();
            if m == "SMASM" || m == "MSAMS" || m == "SSAMM" || m == "MMASS" {
                count += 1;
                overlay_grid[r - 1][c - 1] = grid[r - 1][c - 1];
                overlay_grid[r - 1][c + 1] = grid[r - 1][c + 1];
                overlay_grid[r][c] = grid[r][c];
                overlay_grid[r + 1][c - 1] = grid[r + 1][c - 1];
                overlay_grid[r + 1][c + 1] = grid[r + 1][c + 1];
            }
        }
    }
    count
}

#[test]
fn should_count_xmas() {
    let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    let result = part1(input);
    assert_eq!(result, 18);
}

#[test]
fn should_count_mas_x() {
    let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    let result = part2(input);
    assert_eq!(result, 9);
}
