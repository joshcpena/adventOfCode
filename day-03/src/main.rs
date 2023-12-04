use std::collections::HashMap;
use std::collections::HashSet;
struct Point {
    x: i32,
    y: i32,
}
type Point2d = (i32, i32);
type NumberCoordinate = (i32, i32, i32);
const DIRECTIONS: [Point; 8] = [
    Point { x: -1, y: -1 },
    Point { x: 1, y: 1 },
    Point { x: 1, y: -1 },
    Point { x: -1, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: -1 },
    Point { x: 0, y: 1 },
];
fn map_to_2d_vec(input: &str) -> Vec<Vec<char>> {
    let engine: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    c if c.is_numeric() => c,
                    '.' => '.',
                    _ => '*',
                })
                .collect::<Vec<char>>()
        })
        .collect();
    engine
}

fn main() {
    let input = include_str!("./input.txt");
    let engine = map_to_2d_vec(input);
    let mut symbol_coordinates: Vec<Point> = Vec::new();
    // (first index, last index, y value) , digit
    let mut num_map: HashMap<NumberCoordinate, u32> = HashMap::new();

    engine.iter().enumerate().for_each(|(i, line)| {
        let mut iter = line.iter().enumerate();
        while let Some((j, c)) = iter.next() {
            let first: i32;
            let mut last: i32 = engine[0].len() as i32 - 1;
            let mut num: String = String::new();
            if *c != '*' && *c != '.' {
                first = j as i32;
                num.push(*c);
                while let Some((j, c)) = iter.next() {
                    if *c != '*' && *c != '.' {
                        num.push(*c);
                    } else {
                        last = j as i32 - 1;
                        if *c == '*' {
                            symbol_coordinates.push(Point {
                                x: j as i32,
                                y: i as i32,
                            });
                        }
                        break;
                    }
                }
                num_map.insert((first, last, i as i32), num.parse().expect("this is digit"));
            } else if *c == '*' {
                symbol_coordinates.push(Point {
                    x: j as i32,
                    y: i as i32,
                });
            }
        }
    });
    let mut already_added: HashSet<NumberCoordinate> = HashSet::new();
    let mut nums_to_sum: Vec<i32> = Vec::new();
    let mut gear_map: HashMap<Point2d, (i32, i32)> = HashMap::new();

    symbol_coordinates.iter().for_each(|coordinate| {
        DIRECTIONS.iter().for_each(|dir| {
            let key = (coordinate.x + dir.x, coordinate.y + dir.y);
            num_map.keys().for_each(|k| {
                if k.0 <= key.0 && k.1 >= key.0 && k.2 == key.1 {
                    if !already_added.contains(k) {
                        let num_map_val = num_map.get(k).unwrap().clone() as i32;
                        //? part 1 logic
                        nums_to_sum.push(num_map_val);
                        //? part 2 logic
                        let curr_values = gear_map
                            .get(&(coordinate.x, coordinate.y))
                            .unwrap_or(&(0, 1)); // default to (0 occurrences, 1)
                        println!(
                            "adding to map: {:?}, num_map_val {:?}",
                            curr_values, num_map_val
                        );
                        gear_map.insert(
                            (coordinate.x, coordinate.y),
                            (curr_values.0 + 1, curr_values.1 * num_map_val),
                        );
                    }
                    already_added.insert(*k);
                }
            });
        });
    });
    println!(
        "part 1 = {:?}",
        nums_to_sum.iter().fold(0, |acc, x| acc + x)
    );
    println!(
        "part 2 = {:?}",
        gear_map
            .values()
            .filter(|(times_inserted, _)| *times_inserted == 2)
            .fold(0, |acc, (_, value)| acc + value)
    );
}
