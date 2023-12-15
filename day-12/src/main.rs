use std::collections::VecDeque;

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use itertools::Itertools;
use rayon::prelude::*;
fn shit_is_valid(springs: Vec<char>, groups: &Vec<u8>) -> usize {
    let mut iter = springs.iter().peekable();
    let mut groupings: Vec<u8> = vec![];
    while let Some(c) = iter.next() {
        if *c == '#' {
            let mut count = 1;
            while iter.peek() == Some(&&'#') {
                count += 1;
                iter.next();
            }
            groupings.push(count);
        }
    }
    match &groupings == groups {
        true => 1,
        false => 0,
    }
}

fn can_match(string: &[char], num: usize) -> bool {
    string.len() > num
        && (string.get(num) == Some(&'.') || string.get(num) == Some(&'?'))
        && string.iter().take(num).all(|&ch| ch == '?' || ch == '#')
}
fn shit_is_valid_bool(springs: &[char], groups: &Vec<u8>) -> bool {
    let mut iter = springs.iter().peekable();
    let mut groupings: Vec<u8> = vec![];
    while let Some(c) = iter.next() {
        if *c == '#' {
            let mut count = 1;
            while iter.peek() == Some(&&'#') {
                count += 1;
                iter.next();
            }
            groupings.push(count);
        }
    }
    println!(
        "groupings == group {:?} {:?} {:?}",
        groupings,
        groups,
        groupings[0] == groups[0]
    );
    groupings[0] == groups[0]
}

fn replace_the_shits(springs: Vec<char>, groups: &Vec<u8>) -> usize {
    let iter = springs.iter();

    if iter.clone().any(|c| *c == '?') {
        let curr_spring_count = iter.clone().fold(0usize, |mut acc, ch| {
            if ch == &'#' {
                acc += 1;
            }
            acc
        });
        if curr_spring_count > groups.iter().sum::<u8>() as usize {
            return 0;
        };

        let pos = iter.clone().position(|c| *c == '?').unwrap();
        let mut replaced_with_tag = springs.clone();
        replaced_with_tag[pos] = '#';
        let mut replaced_with_dot = springs.clone();
        replaced_with_dot[pos] = '.';

        replace_the_shits(replaced_with_dot, groups) + replace_the_shits(replaced_with_tag, groups)
    } else {
        shit_is_valid(springs, groups)
    }
}

fn unfold(penis: Vec<char>, groups: &Vec<u8>) -> (Vec<char>, Vec<u8>) {
    let mut new_penis = penis.clone();
    new_penis.push('?');
    (new_penis.repeat(5), groups.repeat(5))
}
fn dp(mut spring: Vec<char>, mut groups: Vec<u8>) -> usize {
    groups.reverse();
    groups.insert(0, 0);
    spring.push('*');
    let spring_len = spring.len();
    // println!("{:?}", groups);

    let mut dp: Vec<Vec<usize>> = vec![vec![0; spring_len]; groups.len()];
    for group in 0..groups.len() {
        println!("{:?}, ", groups[group]);

        for c in (0..spring_len).rev() {
            // println!(" #At ({:?}, {:?})", c, group);
            match spring[c] {
                '*' => {
                    if group == 0 {
                        dp[group][c] = 1;
                    } else {
                        dp[group][c] = 0;
                    }
                }
                '.' => dp[group][c] = dp[group][c + 1],
                '#' => {
                    if can_match(&spring[c..spring_len], groups[group] as usize)
                        && c + groups[group] as usize + 1 < spring_len
                        && group as isize - 1 >= 0
                    {
                        print!(
                            " # should set = to {:?}.",
                            dp[group - 1][c + groups[group] as usize + 1],
                        );
                        dp[group][c] = dp[group][c] + dp[group - 1][c + groups[group] as usize + 1]
                    } else {
                        dp[group][c] = 0
                    }
                }
                '?' => {
                    dp[group][c] = dp[group][c + 1];
                    if can_match(&spring[c..spring_len], groups[group] as usize)
                        && c + groups[group] as usize + 1 < spring_len
                        && group as isize - 1 >= 0
                    {
                        // println!(
                        //     "?  should set = to {:?}. ",
                        //     dp[group - 1][c + groups[group] as usize + 1],
                        // );
                        dp[group][c] = dp[group][c] + dp[group - 1][c + groups[group] as usize + 1]
                    }
                }
                _ => panic!("some shit went down"),
            }
        }
    }
    dp.iter().for_each(|x| println!("\n{:?}", x));
    dp[groups.len() - 1][0]
}

fn main() {
    let input = include_str!("./input.txt");
    // let part_1 = input
    //     .lines()
    //     // .par_bridge() // Add this line to convert the iterator into a parallel iterator
    //     .map(|line| {
    //         let (springs, groups) = line.split_once(" ").unwrap();
    //         let groups = groups
    //             .split(",")
    //             .map(|g| g.parse::<u8>().unwrap())
    //             .collect::<Vec<u8>>();
    //         // println!("groups: {:?}", groups);
    //         // println!("springs: {:?}", springs);

    //         let springs = springs.chars().collect_vec();
    //         // let (springs, groups) = unfold(springs.clone(), &groups);
    //         dp(springs, groups)
    //         // replace_the_shits(springs, &groups)
    //     })
    //     // .progress_count(input.lines().count() as u64)
    //     .sum::<usize>();
    // println!("part_1: {}", part_1);
    println!("part_2: {}", part_2(input));
}
/*
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
--
?###???????? 3,2,1
.###.##.#...
.###.##..#..
.###.##...#.
.###.##....#
.###..##.#..
.###..##..#.
.###..##...#
.###...##.#.
.###...##..#
.###....##.#
 */

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Spring {
    Unknown,
    Damaged,
    Operational,
}

fn parse(input: &str) -> impl Iterator<Item = (Vec<Spring>, Vec<usize>)> + '_ {
    input.lines().map(|line| {
        let (springs, counts) = line.split_once(' ').unwrap();
        let springs: Vec<Spring> = springs
            .chars()
            .map(|c| match c {
                '.' => Spring::Operational,
                '#' => Spring::Damaged,
                '?' => Spring::Unknown,
                _ => panic!("at the disco"),
            })
            .collect();
        let counts: Vec<usize> = counts.split(',').filter_map(|s| s.parse().ok()).collect();

        (springs, counts)
    })
}

fn count_possible_arangements(mut springs: Vec<Spring>, counts: Vec<usize>) -> u64 {
    // to make the Damaged recursion case simpler
    springs.push(Spring::Operational);
    let mut cache = vec![vec![None; springs.len()]; counts.len()];
    count_possible_arangements_inner(&springs, &counts, &mut cache)
}

fn count_possible_arangements_inner(
    springs: &[Spring],
    counts: &[usize],
    cache: &mut [Vec<Option<u64>>],
) -> u64 {
    if counts.is_empty() {
        return if springs.contains(&Spring::Damaged) {
            // Too many previous unknowns were counted as damaged
            0
        } else {
            // All remaining unknowns are operational
            1
        };
    }
    if springs.len() < counts.iter().sum::<usize>() + counts.len() {
        // Not enough space for remaining numbers
        return 0;
    }
    if let Some(cached) = cache[counts.len() - 1][springs.len() - 1] {
        return cached;
    }
    let mut arangements = 0;
    if springs[0] != Spring::Damaged {
        // Assume operational
        arangements += count_possible_arangements_inner(&springs[1..], counts, cache);
    }
    let next_group_size = counts[0];
    if !springs[..next_group_size].contains(&Spring::Operational)
        && springs[next_group_size] != Spring::Damaged
    {
        // Assume damaged
        arangements +=
            count_possible_arangements_inner(&springs[next_group_size + 1..], &counts[1..], cache);
    }
    cache[counts.len() - 1][springs.len() - 1] = Some(arangements);
    arangements
}

pub fn part_2(input: &str) -> u64 {
    parse(input)
        .map(|(mut springs, mut counts)| {
            springs = springs
                .iter()
                .copied()
                .chain([Spring::Unknown])
                .cycle()
                .take(springs.len() * 5 + 4)
                .collect();
            counts = counts
                .iter()
                .copied()
                .cycle()
                .take(counts.len() * 5)
                .collect();

            count_possible_arangements(springs, counts)
        })
        .sum()
}
