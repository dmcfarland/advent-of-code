use advent_of_code::*;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = read_input(2024, 16);
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2: {}", part2_result);
}

#[derive(Debug, Clone)]
struct Route {
    head: (usize, usize),
    value: char,
    path: Vec<char>,
    steps: u32,
    turns: u32,
    trail: Vec<(usize, usize)>,
}

impl Route {
    fn score(&self) -> u32 {
        self.steps + (self.turns * 1000)
    }
}

fn part1(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let raindeer_location = find_in_grid(&grid, 'E').unwrap();
    let instructions: HashMap<char, (isize, isize)> =
        HashMap::from([('^', (-1, 0)), ('v', (1, 0)), ('<', (0, -1)), ('>', (0, 1))]);
    let inverse = HashMap::from([('<', '>'), ('>', '<'), ('^', 'v'), ('v', '^')]);

    let mut routes: Vec<Route> = Vec::new();
    routes.push(Route {
        head: raindeer_location,
        value: '>',
        path: vec!['S'],
        steps: 0,
        turns: 0,
        trail: vec![raindeer_location],
    });

    while !routes
        .first()
        .map_or(false, |r| grid[r.head.0][r.head.1] == 'E')
    {
        let mut next_routes: Vec<Route> = Vec::new();
        let mut pruned_routes: Vec<Route> = Vec::new();

        for r in routes.into_iter() {
            if pruned_routes
                .iter()
                .find(|p| p.trail.contains(&r.head))
                .is_none()
            {
                pruned_routes.push(r);
            }
        }

        routes = pruned_routes;

        while let Some(route) = routes.pop() {
            if grid[route.head.0][route.head.1] == 'E' {
                next_routes.push(Route {
                    head: route.head,
                    value: route.value,
                    path: route.path,
                    steps: route.steps,
                    turns: route.turns,
                    trail: route.trail,
                });
                continue;
            }
            for instruction in instructions
                .iter()
                .filter(|i| &inverse[i.0] != &route.value)
            {
                let next = (
                    (instruction.1 .0 + route.head.0 as isize) as usize,
                    (instruction.1 .1 + route.head.1 as isize) as usize,
                );
                if grid[next.0][next.1] != '#' {
                    let mut path = route.path.clone();
                    let mut trail = route.trail.clone();
                    trail.push(next);
                    path.push(*instruction.0);
                    let is_turn = route.value != *instruction.0;

                    let latest_route = Route {
                        head: next,
                        value: *instruction.0,
                        path,
                        steps: route.steps + 1,
                        turns: if is_turn {
                            route.turns + 1
                        } else {
                            route.turns
                        },
                        trail,
                    };
                    next_routes.push(latest_route);
                }
            }
        }
        routes.append(&mut next_routes);
        routes.sort_by(|a, b| a.score().cmp(&b.score()));
    }

    if let Some(r) = routes.first() {
        return r.score();
    }

    return 0;
}

fn part2(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let raindeer_location = find_in_grid(&grid, 'E').unwrap();
    let instructions: HashMap<char, (isize, isize)> =
        HashMap::from([('^', (-1, 0)), ('v', (1, 0)), ('<', (0, -1)), ('>', (0, 1))]);
    let inverse = HashMap::from([('<', '>'), ('>', '<'), ('^', 'v'), ('v', '^')]);

    let mut routes: Vec<Route> = Vec::new();
    routes.push(Route {
        head: raindeer_location,
        value: '>',
        path: vec!['S'],
        steps: 0,
        turns: 0,
        trail: vec![raindeer_location],
    });

    print_grid(&grid);

    let mut i = 0;
    while !routes
        .first()
        .map_or(false, |r| grid[r.head.0][r.head.1] == 'E')
    {
        i += 1;

        let mut next_routes: Vec<Route> = Vec::new();
        let mut pruned_routes: Vec<Route> = Vec::new();

        for r in routes.into_iter() {
            if pruned_routes
                .iter()
                .find(|p| p.trail.contains(&r.head) && p.head != r.head)
                .is_none()
            {
                pruned_routes.push(r);
            }
        }

        routes = if pruned_routes.len() > 1000 {
            pruned_routes[0..1000].to_vec()
        } else {
            pruned_routes
        };

        while let Some(route) = routes.pop() {
            if grid[route.head.0][route.head.1] == 'E' {
                next_routes.push(Route {
                    head: route.head,
                    value: route.value,
                    path: route.path,
                    steps: route.steps,
                    turns: route.turns,
                    trail: route.trail,
                });
                continue;
            }
            for instruction in instructions
                .iter()
                .filter(|i| &inverse[i.0] != &route.value)
            {
                let next = (
                    (instruction.1 .0 + route.head.0 as isize) as usize,
                    (instruction.1 .1 + route.head.1 as isize) as usize,
                );
                if grid[next.0][next.1] != '#' {
                    let mut path = route.path.clone();
                    let mut trail = route.trail.clone();
                    trail.push(next);
                    path.push(*instruction.0);
                    let is_turn = route.value != *instruction.0;

                    let latest_route = Route {
                        head: next,
                        value: *instruction.0,
                        path,
                        steps: route.steps + 1,
                        turns: if is_turn {
                            route.turns + 1
                        } else {
                            route.turns
                        },
                        trail,
                    };
                    next_routes.push(latest_route);
                }
            }
        }
        routes.append(&mut next_routes);
        routes.sort_by(|a, b| a.score().cmp(&b.score()));
        println!("Step {:?} | routes: {}", i, routes.len());
    }

    if let Some(r) = routes.first() {
        if grid[r.head.0][r.head.1] == 'E' {
            let mut p: HashSet<(usize, usize)> = HashSet::new();

            for x in routes.iter().filter(|p| p.score() == r.score()) {
                println!("x: {:?}", x);
                x.trail.iter().for_each(|t| {
                    let _ = p.insert(*t);
                });
            }

            let mut overlay = grid.clone();
            for x in &p {
                overlay[x.0][x.1] = 'O';
            }
            print_grid(&overlay);
            println!("Score -- {}", r.score());
            println!("BestSpots: {}", p.len());

            return p.len() as u32;
        }
    }

    print_grid(&grid);

    return 0;
}

#[test]
fn test_part1_sample1() {
    let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
    let result = part1(&input);
    assert_eq!(result, 7036);
}

#[test]
fn test_part1_sample2() {
    let input = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;
    let result = part1(&input);
    assert_eq!(result, 11048);
}

#[test]
fn test_part2_sample1() {
    let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
    let result = part2(&input);

    assert_eq!(result, 45);
}

#[test]
fn test_part2_sample2() {
    let input = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;
    let result = part2(&input);
    assert_eq!(result, 64);
}
