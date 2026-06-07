use advent_of_code::*;

fn main() {
    let input = read_input(2024, 20);
    let part1_result = part1(&input);
    println!("Part 1: {:?}", part1_result);

    // let part2_result = part2(&input);
    // println!("Part 2: {}", part2_result);
}

fn part1(input: &str) -> u32 {
    let grid = as_grid_of_char(input);

    let start = find_in_grid(&grid, 'S');

    let mut tracks = vec![vec![start]];

    'l: loop {
        let mut track = vec![tracks.last()];

        track.iter().flat_map(|s|

        );

        break 'l;
    }



    return 0;
}

const SAMPLE_1: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

#[test]
fn part1_test() {
    let input = left_trim_lines(SAMPLE_1);
    assert_eq!(part1(&input), 6);
}
