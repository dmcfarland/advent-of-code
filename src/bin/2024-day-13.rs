use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/resources/day-13-input.txt")
        .expect("Should have been able to read the file");
    let result = part1(&input);
    println!("Results {:?}", result);
}

fn part1(input: &str) -> u32 {
    let re = Regex::new(
        r"(?m)^Button A: X\+(\d*), Y\+(\d*)\nButton B: X\+(\d*), Y\+(\d*)\nPrize: X=(\d*), Y=(\d*)",
    )
    .unwrap();

    re.captures(input).unwrap()

    return 0;
}

#[test]
fn test_part1() {
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
    let result = part1(&input);
    println!("Counts {:?}", result);
    assert_eq!(result, 480);
}
