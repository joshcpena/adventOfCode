use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, line_ending, space0, space1},
    multi::{fold_many1, separated_list1},
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult, Parser,
};
use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    winners: HashSet<u32>,
    scratched_numbers: HashSet<u32>,
}

fn into_set(input: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(
        terminated(complete::u32, space0),
        HashSet::new,
        |mut acc: HashSet<_>, elem| {
            acc.insert(elem);
            acc
        },
    )(input)
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, id): (&str, &str) = delimited(
        tuple((tag("Card"), space1)),
        digit1,
        tuple((tag(":"), space1)),
    )(input)?;
    separated_pair(into_set, tuple((tag("|"), space1)), into_set)
        .map(|(winners, scratched_numbers)| Card {
            id: id.parse().expect("this is a digit"),
            winners,
            scratched_numbers,
        })
        .parse(input)
}
fn parse(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, card)(input)
}
impl Card {
    fn part_1(&self) -> u32 {
        let res = self.winners.intersection(&self.scratched_numbers).count();
        match res.checked_sub(1) {
            Some(num) => 2u32.pow(num as u32),
            None => 0,
        }
    }
    fn part_2(&self) -> (u32, u32) {
        let score = self.winners.intersection(&self.scratched_numbers).count() as u32;
        (self.id, score)
    }
}
fn main() {
    let games = include_str!("./input.txt");
    let (_, parsed) = parse(games).expect("parse worked");
    println!(
        "Part 1: {:?}",
        parsed.iter().map(|card| card.part_1()).sum::<u32>()
    );
    let data = parsed
        .iter()
        .map(|card| card.part_2())
        .collect::<Vec<(u32, u32)>>();

    let scores = parsed
        .iter()
        .map(|card| (card.id, 1))
        .collect::<BTreeMap<u32, u32>>();

    let res = data
        .iter()
        .fold(scores, |mut acc, (id, score)| {
            let curr = *acc.get(id).expect("id in struct");
            for i in (*id + 1)..(*id + 1 + *score) {
                acc.entry(i).and_modify(|count| {
                    *count += curr;
                });
            }
            acc
        })
        .values()
        .sum::<u32>();
    println!("Part 2: {:?}", res);
}
