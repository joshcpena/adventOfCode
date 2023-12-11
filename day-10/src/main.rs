use std::collections::{HashMap, VecDeque};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
const DIRECTIONS: [Point; 4] = [
    Point { x: 0, y: -1 }, //North
    Point { x: 1, y: 0 },  //East
    Point { x: 0, y: 1 },  //South
    Point { x: -1, y: 0 }, //West
];
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn get_starting_pos(input: &Vec<Vec<String>>) -> Point {
    let mut starting_pos: Point = Point { x: 0, y: 0 };
    for (y, line) in input.iter().enumerate() {
        let start = line.iter().position(|c| c == "S");
        match start {
            Some(x) => {
                starting_pos = Point {
                    x: x as i32,
                    y: y as i32,
                };
                break;
            }
            None => continue,
        }
    }
    starting_pos
}

fn main() {
    use Direction::*;
    let mapped_dirs: HashMap<Point, Direction> = HashMap::from([
        (DIRECTIONS[0], North),
        (DIRECTIONS[1], East),
        (DIRECTIONS[2], South),
        (DIRECTIONS[3], West),
    ]);
    let valid_pipes: HashMap<Direction, HashMap<&str, Vec<&str>>> = HashMap::from([
        (
            North,
            HashMap::from([
                ("S", vec!["|", "-", "L", "J", "7", "F"]),
                ("|", vec!["|", "7", "F"]),
                ("J", vec!["|", "7", "F"]),
                ("L", vec!["|", "7", "F"]),
            ]),
        ),
        (
            East,
            HashMap::from([
                ("S", vec!["|", "-", "L", "J", "7", "F"]),
                ("-", vec!["-", "J", "7"]),
                ("L", vec!["-", "J", "7"]),
                ("F", vec!["-", "J", "7"]),
            ]),
        ),
        (
            South,
            HashMap::from([
                ("S", vec!["|", "-", "L", "J", "7", "F"]),
                ("|", vec!["|", "L", "J"]),
                ("7", vec!["|", "L", "J"]),
                ("F", vec!["|", "L", "J"]),
            ]),
        ),
        (
            West,
            HashMap::from([
                ("S", vec!["|", "-", "L", "J", "7", "F"]),
                ("-", vec!["-", "L", "F"]),
                ("J", vec!["-", "L", "F"]),
                ("7", vec!["-", "L", "F"]),
            ]),
        ),
    ]);

    let mut input = include_str!("./input.txt")
        .split("\n")
        .map(|x| x.chars().collect::<Vec<_>>())
        .map(|c| c.iter().map(|x| x.to_string()).collect::<Vec<_>>())
        .collect::<Vec<Vec<String>>>();

    let starting_pos = get_starting_pos(&input);
    let mut to_visit = VecDeque::from([starting_pos]);
    let mut visited: Vec<Point> = vec![];
    while let Some(pos) = to_visit.pop_front() {
        visited.push(pos);
        DIRECTIONS.iter().for_each(|dir| {
            let next_pos = Point {
                x: pos.x + dir.x,
                y: pos.y + dir.y,
            };
            if next_pos.x as usize >= input[0].len() || next_pos.y as usize >= input.len() {
                return;
            }

            let next_char = &input[next_pos.y as usize][next_pos.x as usize].as_str();
            let dir_pipe_map = valid_pipes.get(mapped_dirs.get(dir).unwrap()).unwrap();
            let empty = vec![""];
            let valid_chars = dir_pipe_map
                .get(input[pos.y as usize][pos.x as usize].as_str())
                .unwrap_or(&empty);
            match next_char {
                c if valid_chars.contains(&c)
                    && !visited.contains(&next_pos)
                    && !to_visit.contains(&next_pos) =>
                {
                    to_visit.push_back(next_pos);
                }
                _ => {}
            }
        });
    }
    let mut total_inside = 0;
    for (row, line) in input.clone().iter().enumerate() {
        let mut ray_inside = false;
        let mut column = 0;
        while column < line.len() {
            if visited.contains(&Point {
                x: column as i32,
                y: row as i32,
            }) {
                if line[column] == "|" || line[column] == "S" {
                    ray_inside = !ray_inside;
                } else if line[column] == "L" || line[column] == "F" {
                    let left = &line[column];
                    while column < line.len() && line[column + 1] == "-" {
                        column += 1;
                    }
                    if (left == "L" && line[column + 1] == "7")
                        || (left == "F" && line[column + 1] == "J")
                    {
                        ray_inside = !ray_inside;
                    }
                }
            } else if ray_inside {
                input[row as usize][column as usize] = "#".to_string();
                total_inside += 1
            }
            column += 1;
        }
    }
    println!("max distance in path: {:?}", visited.len() / 2);
    println!("total_inside: {:?}", total_inside);
}
