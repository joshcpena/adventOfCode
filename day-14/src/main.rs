use core::panic;
use std::collections::HashMap;

use indicatif::ProgressBar;
use itertools::Itertools;
use rayon::prelude::*;
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn part_1(input: &str) -> i32 {
    let puzzle = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut tiled_transposed: Vec<Vec<&char>> = vec![];
    for row in 0..puzzle.len() {
        let mut col = puzzle
            .iter()
            .map(|s| s.get(row).unwrap())
            .collect::<Vec<_>>();
        for i in 1..col.len() {
            match col[i] {
                'O' => {
                    if col[i - 1] == &'.' {
                        col[i] = &'.';
                        let new_pos = col[..i].iter().rev().position(|&&c| c == 'O' || c == '#');
                        match new_pos {
                            Some(pos) => col[i - pos] = &'O',
                            None => col[0] = &'O',
                        };
                    }
                }
                _ => (),
            }
        }
        tiled_transposed.push(col);
    }
    tiled_transposed.iter().fold(0i32, |acc, col| {
        col.iter().enumerate().fold(acc, |acc, (i, x)| match x {
            'O' => acc + 1 * (col.len() - i) as i32,
            _ => acc,
        })
    })
}

fn part_2(mut puzzle: Vec<Vec<char>>) -> i32 {
    let pb = ProgressBar::new(500);
    let mut seen = HashMap::new();
    let mut solution: i32 = -1;
    for i in 1..500 {
        pb.inc(1);
        for _ in 0..=3 {
            let mut tiled_transposed: Vec<Vec<char>> = vec![];
            for row in 0..puzzle.len() {
                let mut col = puzzle
                    .par_iter()
                    .map(|s| *s.get(row).unwrap())
                    .collect::<Vec<_>>();
                for i in 1..col.len() {
                    match col[i] {
                        'O' => {
                            if col[i - 1] == '.' {
                                col[i] = '.';
                                let new_pos =
                                    col[..i].iter().rev().position(|&c| c == 'O' || c == '#');
                                match new_pos {
                                    Some(pos) => col[i - pos] = 'O',
                                    None => col[0] = 'O',
                                };
                            }
                        }
                        _ => (),
                    }
                }
                tiled_transposed.push(col);
            }
            puzzle = tiled_transposed
                .iter()
                .map(|x| x.iter().rev().map(|c| *c).collect())
                .collect();
        }

        if let Some(seen_at) = seen.insert(puzzle.clone(), i) {
            if (1000000000 - i) % (i - seen_at) == 0 {
                solution = transpose(puzzle.clone()).iter().fold(0i32, |acc, col| {
                    col.iter().enumerate().fold(acc, |acc, (i, x)| match x {
                        'O' => acc + 1 * (col.len() - i) as i32,
                        _ => acc,
                    })
                });
                break;
            }
        }
    }
    pb.finish();
    solution
}
fn main() {
    let puzzle = include_str!("./input.txt")
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    println!("part_2 {:?}", part_2(puzzle));
}

#[test]
fn test_1() {
    assert_eq!(part_1(include_str!("./small.txt")), 136);
    assert_eq!(part_1(include_str!("./input.txt")), 109939);
}

#[test]
fn test_2() {
    let puzzle = include_str!("./input.txt")
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    assert_eq!(part_2(puzzle), 64);
}
/*

OOOO.#.O.. 10
OO..#....#  9
OO..O##..O  8
O..#.OO...  7
........#.  6
..#....#.#  5
..O..#.O.O  4
..O.......  3
#....###..  2
#....#....  1

*/
