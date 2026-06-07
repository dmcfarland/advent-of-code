use std::collections::HashSet;

use advent_of_code::*;

fn main() {
    let input = read_input(2024, 22);
    let part1_result = part1(&input);
    println!("Part 1: {:?}", part1_result);
}

fn part1(input: &str) -> usize {
    let mut groups: Vec<HashSet<&str>> = Vec::new();

    for line in input.lines() {
        let boxes: Vec<_> = line.split('-').collect();
        if let Some(group) = groups
            .iter_mut()
            .find(|g| g.contains(boxes[0]) || g.contains(boxes[1]))
        {
            group.insert(boxes[0]);
            group.insert(boxes[1]);
        } else {
            let group: HashSet<_> = HashSet::from_iter(boxes);
            groups.push(group);
        }
    }
    println!("Sets: {:?}", groups);

    println!(
        "TSets: {:?}",
        groups
            .iter()
            .filter(|p| p.len() > 2 && p.iter().any(|p1| p1.starts_with("t")))
            .count()
    );

    return groups
        .iter()
        .filter(|p| p.len() > 2 && p.iter().any(|p1| p1.starts_with("t")))
        .count();
}

const SAMPLE_1: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

#[test]
fn part1_test() {
    let input = left_trim_lines(SAMPLE_1);
    assert_eq!(part1(&input), 7);
}
