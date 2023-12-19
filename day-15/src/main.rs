use itertools::Itertools;
use std::collections::HashMap;

fn part_1() -> i32 {
    let hashes = include_str!("./input.txt")
        .split(",")
        .map(|str| str.chars().collect_vec())
        .collect_vec();

    let sum = hashes.iter().fold(0, |mut acc, hash| {
        let mut current = 0;
        hash.iter().for_each(|c| {
            let ascii = *c as u8;
            current += ascii as i32;
            current *= 17;
            current %= 256;
        });
        acc += current;
        acc
    });
    sum
}

fn part_2() -> i32 {
    let mut map: HashMap<i32, Vec<(&[char], i32)>> = HashMap::new();

    let hashes = include_str!("./input.txt")
        .split(",")
        .map(|str| str.chars().collect_vec())
        .collect_vec();

    hashes.iter().for_each(|hash| {
        let delimiter_pos = hash.iter().position(|&c| c == '=' || c == '-').unwrap();
        let (left, right) = hash.split_at(delimiter_pos);
        let box_number = left.iter().fold(0, |mut acc, c| {
            acc += (*c as u8) as i32;
            acc *= 17;
            acc %= 256;
            acc
        });
        match right[0] {
            '=' => {
                let vec = map.get_mut(&box_number);
                let focal_len = right[1].to_digit(10).unwrap() as i32;
                match vec {
                    Some(vec) => {
                        let is_in_map = vec.iter().position(|(hash, _)| hash == &left);
                        match is_in_map {
                            Some(idx) => vec[idx].1 = focal_len,
                            None => vec.push((left, focal_len)),
                        }
                    }
                    None => {
                        let mut vec: Vec<_> = Vec::new();
                        vec.push((left, focal_len));
                        map.insert(box_number, vec);
                    }
                }
            }
            '-' => {
                let vec = map.get_mut(&box_number);
                match vec {
                    Some(vec) => {
                        let idx_to_remove = vec.iter().position(|(hash, _)| hash == &left);
                        match idx_to_remove {
                            Some(idx) => {
                                vec.remove(idx);
                            }
                            None => (),
                        }
                    }
                    None => (),
                }
            }
            _ => panic!("wtf is this"),
        }
    });
    map.into_iter().fold(0, |mut acc, (box_num, value)| {
        acc += value
            .iter()
            .enumerate()
            .fold(0, |mut acc, (i, (_, focal_len))| {
                acc += (box_num + 1) * (i as i32 + 1) * focal_len;
                acc
            });
        acc
    })
}

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

#[test]
fn test_1() {
    assert_eq!(part_1(), 511257);
    assert_eq!(part_2(), 145);
}
