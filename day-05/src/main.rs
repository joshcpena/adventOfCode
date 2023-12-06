use indicatif::{ProgressBar, ProgressIterator};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self, line_ending, newline, space0, space1},
    multi::{fold_many1, many1, many_till},
    sequence::{preceded, terminated, tuple},
    IResult,
};
use std::collections::{HashMap, HashSet};

fn parse_line(input: &str) -> IResult<&str, Vec<usize>> {
    preceded(
        newline,
        fold_many1(
            terminated(complete::u32, space0),
            Vec::new,
            |mut acc: Vec<usize>, elem| {
                acc.push(elem as usize);
                acc
            },
        ),
    )(input)
}
fn parse_multiline_mapping(input: &str) -> IResult<&str, Vec<HashMap<(i64, i64), i64>>> {
    let (input, _) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let maps = many1(parse_line)(input)
        .map(|(_, mappings)| {
            mappings
                .iter()
                .map(|v| {
                    HashMap::from([(
                        (v[1] as i64, v[1] as i64 + v[2] as i64 - 1),
                        v[0] as i64 - v[1] as i64,
                    )])
                })
                .collect::<Vec<_>>()
        })
        .unwrap();
    Ok((input, maps))
}

fn parser(input: &str) -> IResult<&str, (Vec<u32>, Vec<Vec<HashMap<(i64, i64), i64>>>)> {
    // Handle grabbing seeds
    let (input, _) = tuple((tag("seeds:"), space1))(input)?;
    let (input, (seeds, _)) = many_till(terminated(complete::u32, space0), line_ending)(input)?;
    // Handle rest of the maps
    let (_, maps) = many1(parse_multiline_mapping)(input)?;
    Ok((input, (seeds, maps)))
}

fn main() {
    let input = include_str!("./input.txt");

    let (_, (seeds, maps)) = parser(input).unwrap();

    let mut part_2_seeds: Vec<u32> = vec![];
    let mut iter = seeds.iter();

    while let Some(seed) = iter.next() {
        let range = iter.next().unwrap();
        for i in *seed..(*seed + *range) {
            part_2_seeds.push(i);
        }
    }
    let x = part_2_seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed as i64, |mut acc, map| {
                for map in map.iter() {
                    let mut was_found = false;
                    for key in map.keys() {
                        if key.0 as i64 <= acc && key.1 as i64 >= acc {
                            acc += *map.get(key).unwrap() as i64;
                            was_found = true;
                            break;
                        }
                    }
                    if was_found {
                        break;
                    }
                }
                acc
            })
        })
        .progress()
        .collect::<Vec<i64>>();
    println!("res: {:?}", x.iter().min().unwrap());
}
