use std::fs;

pub fn read_input(year: u16, day: u8) -> String {
    
    fs::read_to_string(format!("./input/{year}-day-{:02}.txt", day))
        .expect("Should have been able to read the file")
}

pub fn left_trim_lines(input: &str) -> String {
    input
        .lines()
        .map(|line| line.trim_start())
        .collect::<Vec<&str>>()
        .join("\n")
}

pub fn as_lines(input: &str) -> Vec<&str> {
    input.lines().map(|l| l.trim()).collect()
}

pub fn as_rows_of_u32(input: &str) -> Vec<Vec<u32>> {
    let trimmed_input = left_trim_lines(input);

    trimmed_input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|num| num.parse::<u32>())
                .collect::<Result<Vec<u32>, _>>()
                .expect("Should have been able to parse the numbers")
        })
        .collect()
}

pub fn as_grid_of_char(input: &str) -> Vec<Vec<char>> {
    let trimmed_input = left_trim_lines(input);
    let lines = trimmed_input.lines();
    lines
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

pub fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid.iter() {
        let x: String = row.iter().collect();
        println!("{}", x);
    }
}

pub fn find_in_grid(grid: &Vec<Vec<char>>, character: char) -> Option<(usize, usize)> {
    for (i, r) in grid.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c == character {
                return Some((j, i));
            }
        }
    }
    None
}
