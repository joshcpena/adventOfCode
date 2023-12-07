use std::vec;

use nom::{
    bytes::complete::take_till,
    character::{
        complete::{self, newline, space0},
        is_digit,
    },
    multi::{fold_many1, separated_list1},
    sequence::terminated,
    IResult,
};

use itertools::Itertools;

fn parse_line(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, _) = take_till(|c| is_digit(c as u8))(input)?;
    fold_many1(
        terminated(complete::i64, space0),
        Vec::new,
        |mut acc: Vec<i64>, elem| {
            acc.push(elem);
            acc
        },
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(newline, parse_line)(input)
}

fn calc_margin(input: Vec<Vec<i64>>) -> i64 {
    let mut margin_of_error = 1i64;
    input[0].iter().enumerate().for_each(|(i, &time)| {
        let record_distance = input[1][i];
        let mut winning_velocities = 0i64;
        for velocity in 1..time {
            let distance = velocity * (time - velocity);
            if distance > record_distance {
                winning_velocities += 1;
            }
        }
        margin_of_error *= winning_velocities;
    });
    margin_of_error
}

fn concat_vec(input: Vec<i64>) -> i64 {
    Itertools::join(&mut input.iter(), "")
        .parse::<i64>()
        .expect("it parsed into i64")
}

fn main() {
    let (_, input) = parse(include_str!("./input.txt")).expect("it parsed");
    let joined_time = concat_vec(input[0].clone());
    let joined_distance = concat_vec(input[1].clone());
    println!(
        "part_1: {} \npart_2: {}",
        calc_margin(input.clone()),
        calc_margin(vec![vec![joined_time], vec![joined_distance]])
    );
}
