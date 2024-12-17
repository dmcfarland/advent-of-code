use regex::Regex;

// Did have to resort to subreddit to determine tree would appear when all robots where in unique locations
// Puzzle would have started with this and worked backwards to generate my input

use advent_of_code::*;

fn main() {
    let input = read_input(2024, 14);
    let part1_result = part1(&input, 100, 101, 103);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&input, 10000, 101, 103);
    println!("Part 2: {}", part2_result);
}

#[derive(Debug)]
struct Robot {
    x: u32,
    y: u32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn move_location(&mut self, width: u32, height: u32) {
        let mut new_x: i32 = self.x as i32 + self.vx;
        if new_x < 0 {
            new_x = width as i32 + new_x;
        } else if new_x >= width as i32 {
            new_x = new_x - width as i32;
        }
        self.x = new_x as u32;

        let mut new_y: i32 = self.y as i32 + self.vy;
        if new_y < 0 {
            new_y = height as i32 + new_y;
        } else if new_y >= height as i32 {
            new_y = new_y - height as i32;
        }
        self.y = new_y as u32;
    }
}

fn part1(input: &str, elapsed_seconds: u32, width: u32, height: u32) -> u64 {
    let re = Regex::new(r"(?m)^p=(\d*),(\d*) v=(-?\d*),(-?\d*)$").unwrap();
    let mut robots: Vec<Robot> = re
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [x, y, vx, vy])| Robot {
            x: x.parse::<u32>().unwrap(),
            y: y.parse::<u32>().unwrap(),
            vx: vx.parse::<i32>().unwrap(),
            vy: vy.parse::<i32>().unwrap(),
        })
        .collect();

    for _ in 0..elapsed_seconds {
        robots
            .iter_mut()
            .for_each(|r| r.move_location(width, height));
    }

    let mut quadrants = (0, 0, 0, 0);
    for robot in robots {
        if robot.x < width / 2 && robot.y < height / 2 {
            quadrants.0 += 1;
        }
        if robot.x > width / 2 && robot.y < height / 2 {
            quadrants.1 += 1;
        }
        if robot.x < width / 2 && robot.y > height / 2 {
            quadrants.2 += 1;
        }
        if robot.x > width / 2 && robot.y > height / 2 {
            quadrants.3 += 1;
        }
    }

    println!("{:?}", quadrants);

    return quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3;
}

fn part2(input: &str, elapsed_seconds: u32, width: u32, height: u32) -> u64 {
    let re = Regex::new(r"(?m)^p=(\d*),(\d*) v=(-?\d*),(-?\d*)$").unwrap();
    let mut robots: Vec<Robot> = re
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [x, y, vx, vy])| Robot {
            x: x.parse::<u32>().unwrap(),
            y: y.parse::<u32>().unwrap(),
            vx: vx.parse::<i32>().unwrap(),
            vy: vy.parse::<i32>().unwrap(),
        })
        .collect();

    for i in 0..elapsed_seconds {
        robots
            .iter_mut()
            .for_each(|r| r.move_location(width, height));

        if is_tree(&robots, width, height) {
            return i as u64 + 1;
        }
    }

    return 0;
}

fn is_tree(robots: &Vec<Robot>, width: u32, height: u32) -> bool {
    let mut overlay = vec![vec![0; height as usize]; width as usize];
    let mut x = true;
    for robot in robots {
        overlay[robot.x as usize][robot.y as usize] += 1;
        x = x && overlay[robot.x as usize][robot.y as usize] <= 1;
    }
    if x {
        return true;
    }

    for y in 0..height {
        for x in 0..width / 2 {
            if overlay[x as usize][y as usize] != 0
                && overlay[x as usize][y as usize]
                    == overlay[width as usize - x as usize - 1][y as usize]
            {
                return false;
            }
        }
    }
    return true;
}

#[test]
fn test_part1_sample() {
    let input = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;
    let result = part1(&input, 100, 11, 7);
    println!("Counts {:?}", result);
    assert_eq!(result, 12);
}
