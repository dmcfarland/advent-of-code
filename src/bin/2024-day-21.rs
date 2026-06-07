use advent_of_code::*;

fn main() {
    let input = read_input(2024, 21);
    let part1_result = part1(&input);
    println!("Part 1: {:?}", part1_result);

    // let part2_result = part2(&input);
    // println!("Part 2: {}", part2_result);
}

const NUMERICAL_KEYPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];
const DIRECTIONAL_KEYPAD: [[char; 3]; 2] = [[' ', '^', 'A'], ['<', 'v', '>']];

fn part1(input: &str) -> u32 {
    let codes = input.lines();

    for code in codes {
        // let end_pos: (usize, usize) = (2, 3);
        let rev_code = code.chars().rev().skip(1);
        let mut prev_loc: (usize, usize) = (2, 3);
        for ch in rev_code {
            let loc = find_in_grid_x(&NUMERICAL_KEYPAD, ch).unwrap();
            let r_diff = prev_loc.0.abs_diff(loc.0);
            let c_diff = prev_loc.1.abs_diff(loc.1);
            println!(
                "ch:{} prevLoc:{:?} loc:{:?} {} {} ",
                ch, prev_loc, loc, r_diff, c_diff
            );
            prev_loc = loc;
        }
        println!("----");
    }

    return 0;
}

/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
--------------

    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
*/

const SAMPLE_1: &str = "\
029A
980A
179A
456A
379A";

#[test]
fn part1_test() {
    let input = left_trim_lines(SAMPLE_1);
    assert_eq!(part1(&input), 6);
}
