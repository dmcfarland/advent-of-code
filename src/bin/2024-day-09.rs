use advent_of_code::*;
use std::collections::HashMap;

fn main() {
    let input = read_input(2024, 9);
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2: {}", part2_result);
}

fn part2(input: &str) -> u64 {
    let disk_map = input.lines().next().unwrap_or("");
    let mut is_file = true;
    let mut file_id = 0;
    let mut pointers: Vec<isize> = Vec::new();
    let mut file_sizes: HashMap<usize, u32> = HashMap::new();
    let mut space_sizes: Vec<(usize, u32)> = Vec::new();
    for e in disk_map.chars().into_iter() {
        let blocks = e.to_digit(10).unwrap();
        if is_file {
            file_sizes.insert(file_id as usize, blocks);
        } else {
            space_sizes.push((pointers.len() as usize, blocks));
        }
        for _ in 0..blocks {
            if is_file {
                pointers.push(file_id);
            } else {
                pointers.push(-1);
            }
        }
        if is_file {
            file_id += 1;
        }
        is_file = !is_file;
    }

    let pointer_count = pointers.len();
    let mut right = pointer_count - 1;
    let mut file_id_watermark = usize::MAX;

    while 0 < right {
        if pointers[right] != -1 {
            let f_id = pointers[right as usize] as usize;
            let file_size = file_sizes[&f_id];
            if f_id < file_id_watermark {
                file_id_watermark = f_id;
                // println!("{} id:{} size: {}", right, f_id, file_size);
                let space_p = space_sizes.iter().position(|&(_, size)| size >= file_size);
                if let Some(next_space_val) = space_p {
                    if right > space_sizes[next_space_val].0 {
                        // println!("> {:?}", space_sizes[next_space_val]);
                        for i in 0..file_size {
                            pointers
                                .swap(space_sizes[next_space_val].0 as usize + i as usize, right);
                            right -= 1;
                        }
                        right += 1;
                        if file_size < space_sizes[next_space_val].1 as u32 {
                            let pos = space_sizes[next_space_val].0 as u32 + file_size;
                            let size = space_sizes[next_space_val].1 as u32 - file_size;
                            // println!("XXXX {:?}", (pos as usize, size));
                            space_sizes.remove(next_space_val);
                            space_sizes.insert(next_space_val, (pos as usize, size));
                        } else {
                            space_sizes.remove(next_space_val);
                        }
                    }
                }
            }
        }
        right -= 1;
    }

    let mut checksum: u64 = 0;
    for i in 0..pointer_count {
        if pointers[i] != -1 {
            checksum += pointers[i] as u64 * i as u64;
        }
    }
    return checksum;
}

fn part1(input: &str) -> u64 {
    let disk_map = input.lines().next().unwrap_or("");
    let mut is_file = true;
    let mut file_id = 0;
    let mut pointers: Vec<isize> = Vec::new();
    for e in disk_map.chars().into_iter() {
        let blocks = e.to_digit(10).unwrap();
        // print!("b> {}", blocks);
        for _ in 0..blocks {
            if is_file {
                pointers.push(file_id);
            } else {
                pointers.push(-1);
            }
        }
        if is_file {
            file_id += 1;
        }
        is_file = !is_file;
    }

    let pointer_count = pointers.len();

    for i in 0..pointer_count {
        if pointers[i] == -1 {
            for j in (i..pointer_count).rev() {
                if pointers[j] != -1 {
                    pointers.swap(i, j);
                    break;
                }
            }
        }
    }

    let mut checksum: u64 = 0;
    for i in 0..pointer_count {
        if pointers[i] != -1 {
            checksum += pointers[i] as u64 * i as u64;
        }
    }
    return checksum;
}

#[test]
fn test_part1_sample() {
    let input = "2333133121414131402";
    let result = part1(&input);
    println!("Result {:?}", result);
    assert_eq!(result, 1928);
}

#[test]
fn test_part2_sample() {
    let input = "2333133121414131402994422";
    let result = part2(&input);
    println!("Result {:?}", result);
    assert_eq!(result, 2858);
}
