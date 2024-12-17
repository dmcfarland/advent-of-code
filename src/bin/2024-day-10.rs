use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./src/resources/day-10-input.txt")
        .expect("Should have been able to read the file");
    let result = part1(&input);
    println!("Results {:?}", result);
}

fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let grid = lines
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut trails: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] != '.' {
                trails
                    .entry(grid[x][y].to_digit(10).unwrap())
                    .or_insert_with(Vec::new)
                    .push((x as u32, y as u32));
            }
        }
    }

    println!("Trails: {:?}", trails);
    let mut trailheads_count = 0;
    for trailhead in trails[&0].clone() {
        let mut routes: Vec<Vec<(u32, u32)>> = Vec::new();
        routes.push(vec![trailhead]);
        for n in 1..10 {
            let last = routes.last().unwrap();
            let next: Vec<(u32, u32)> = trails[&n]
                .iter()
                .filter(|s| {
                    last.iter().any(|f| {
                        (s.0.abs_diff(f.0) == 1 && s.1.abs_diff(f.1) == 0)
                            || (s.1.abs_diff(f.1) == 1 && s.0.abs_diff(f.0) == 0)
                    })
                })
                .cloned()
                .collect();
            routes.push(next);
        }

        let mut unique_tracker: Vec<Vec<(u32, u32, u32)>> = Vec::new();
        unique_tracker.push(vec![(trailhead.0, trailhead.1, 1)]);
        println!(">>{:?}", trailhead);
        for n in 1..10 {
            let last = unique_tracker.last().unwrap();
            let current = routes[n].clone();
            let mut a: Vec<(u32, u32, u32)> = Vec::new();
            for l in last {
                if current.len() > 1 {
                    for c in &current {
                        if (l.0.abs_diff(c.0) == 1 && l.1.abs_diff(c.1) == 0)
                            || (l.1.abs_diff(c.1) == 1 && l.0.abs_diff(c.0) == 0)
                        {
                            a.push(value);
                        }
                    }
                } else {
                    a.push(value);
                }
            }
        }

        trailheads_count += 1;
    }

    return trailheads_count as u32;
}

#[test]
fn test_part1_b() {
    let input = r#"012345
123456
234567
345678
4.6789
56789."#;
    let result = part1(&input);
    println!("Counts {:?}", result);
    assert_eq!(result, 36);
}

// #[test]
// fn test_part1_b() {
//     let input = r#"89010123
// 78121874
// 87430965
// 96549874
// 45678903
// 32019012
// 01329801
// 10456732"#;
//     let result = part1(&input);
//     println!("Counts {:?}", result);
//     assert_eq!(result, 36);
// }

// #[test]
// fn test_part1_a() {
//     let input = r#"..90..9
// ...1.98
// ...2..7
// 6543456
// 765.987
// 876....
// 987...."#;
//     let result = part1(&input);
//     println!("Counts {:?}", result);
//     assert_eq!(result, 4);
// }

// #[test]
// fn test_part1_b() {
//     let input = r#"10..9..
// 2...8..
// 3...7..
// 4567654
// ...8..3
// ...9..2
// .....01"#;
//     let result = part1(&input);
//     println!("Counts {:?}", result);
//     assert_eq!(result, 3);
// }

// 89010123
// 78121874
// 87430965
// 96549874
// 45678903
// 32019012
// 01329801
// 10456732

/*
t(0, 2) - n1 - l[(0, 2)]
Next: [(0, 3), (1, 2)]
t(0, 2) - n2 - l[(0, 3), (1, 2)]
Next: [(1, 3)]
t(0, 2) - n3 - l[(1, 3)]
Next: [(2, 3)]
t(0, 2) - n4 - l[(2, 3)]
Next: [(2, 2), (3, 3)]
t(0, 2) - n5 - l[(2, 2), (3, 3)]
Next: [(3, 2)]
t(0, 2) - n6 - l[(3, 2)]
Next: [(3, 1), (4, 2)]
t(0, 2) - n7 - l[(3, 1), (4, 2)]
Next: [(2, 1), (4, 3)]
t(0, 2) - n8 - l[(2, 1), (4, 3)]
Next: [(1, 1), (2, 0), (4, 4)]
t(0, 2) - n9 - l[(1, 1), (2, 0), (4, 4)]
Next: [(0, 1), (3, 0), (3, 4), (4, 5), (5, 4)]
(0, 2) 3
*/
