use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn part_1(start: Point, dir: Point, grid: &Vec<Vec<char>>) -> usize {
    let mut visited: Vec<(Point, Point)> = Vec::new();
    let mut queue: VecDeque<(Point, Point)> = VecDeque::new();
    queue.push_back((start, dir));
    let mut counter = 0;

    while let Some(point) = queue.pop_front() {
        if counter > 100_000_000 {
            break;
        }
        counter += 1;
        let (current_point, direction) = point;
        if current_point.x < 0
            || current_point.y < 0
            || current_point.x >= grid[0].len() as i32
            || current_point.y >= grid.len() as i32
        {
            continue;
        }
        if visited.contains(&(current_point, direction)) {
            continue;
        }
        visited.push((current_point, direction));
        match grid[current_point.y as usize][current_point.x as usize] {
            '.' => {
                let next_point = Point {
                    x: current_point.x + direction.x,
                    y: current_point.y + direction.y,
                };
                queue.push_back((next_point, direction));
            }
            '\\' => {
                // case direction is left to right
                if direction.x == 1 && direction.y == 0 {
                    let next_point = Point {
                        x: current_point.x,
                        y: current_point.y + 1,
                    };
                    queue.push_back((next_point, Point { x: 0, y: 1 }));
                }
                // case direction is right to left
                else if direction.x == -1 && direction.y == 0 {
                    let next_point = Point {
                        x: current_point.x,
                        y: current_point.y + -1,
                    };
                    queue.push_back((next_point, Point { x: 0, y: -1 }));
                }
                // case direction is up to down
                else if direction.x == 0 && direction.y == 1 {
                    let next_point = Point {
                        x: current_point.x + 1,
                        y: current_point.y,
                    };
                    queue.push_back((next_point, Point { x: 1, y: 0 }));
                }
                // case direction is down to up
                else if direction.x == 0 && direction.y == -1 {
                    let next_point = Point {
                        x: current_point.x + -1,
                        y: current_point.y,
                    };
                    queue.push_back((next_point, Point { x: -1, y: 0 }));
                }
            }
            '/' => {
                // case direction is left to right
                if direction.x == 1 && direction.y == 0 {
                    let next_point = Point {
                        x: current_point.x,
                        y: current_point.y + -1,
                    };
                    queue.push_back((next_point, Point { x: 0, y: -1 }));
                }
                // case direction is right to left
                else if direction.x == -1 && direction.y == 0 {
                    let next_point = Point {
                        x: current_point.x,
                        y: current_point.y + 1,
                    };
                    queue.push_back((next_point, Point { x: 0, y: 1 }));
                }
                // case direction is up to down
                else if direction.x == 0 && direction.y == 1 {
                    let next_point = Point {
                        x: current_point.x + -1,
                        y: current_point.y,
                    };
                    queue.push_back((next_point, Point { x: -1, y: 0 }));
                }
                // case direction is down to up
                else if direction.x == 0 && direction.y == -1 {
                    let next_point = Point {
                        x: current_point.x + 1,
                        y: current_point.y,
                    };
                    queue.push_back((next_point, Point { x: 1, y: 0 }));
                }
            }
            '-' => {
                // horizontal beam, treat as .
                if direction.y == 0 {
                    let next_point = Point {
                        x: current_point.x + direction.x,
                        y: current_point.y,
                    };
                    queue.push_back((next_point, direction));
                }
                // vertical beam, split into two
                else {
                    let next_point = Point {
                        x: current_point.x + 1,
                        y: current_point.y,
                    };
                    queue.push_back((next_point, Point { x: 1, y: 0 }));
                    let next_point = Point {
                        x: current_point.x - 1,
                        y: current_point.y,
                    };
                    queue.push_back((next_point, Point { x: -1, y: 0 }));
                }
            }
            '|' => {
                // vertical beam, treat as .
                if direction.x == 0 {
                    let next_point = Point {
                        x: current_point.x,
                        y: current_point.y + direction.y,
                    };
                    queue.push_back((next_point, direction));
                }
                // horizontal beam, split into two
                else {
                    let next_point = Point {
                        x: current_point.x,
                        y: current_point.y + 1,
                    };
                    queue.push_back((next_point, Point { x: 0, y: 1 }));
                    let next_point = Point {
                        x: current_point.x,
                        y: current_point.y - 1,
                    };
                    queue.push_back((next_point, Point { x: 0, y: -1 }));
                }
            }
            _ => panic!("some shit went down"),
        }
    }
    let mut unique_points = visited
        .iter()
        .map(|vec: &(Point, Point)| vec.0)
        .unique()
        .collect_vec();
    unique_points.len()
}

fn main() {
    let input: &str = include_str!("./input.txt");
    let grid = input
        .split("\n")
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    // part_1(Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, &grid);

    let mut starting_points: Vec<(Point, Point)> = Vec::new();

    grid[0].iter().enumerate().for_each(|(i, _)| {
        // top row going south
        starting_points.push((Point { x: i as i32, y: 0 }, Point { x: 0, y: 1 }));
        // bot rows going north
        starting_points.push((
            Point {
                x: i as i32,
                y: grid.len() as i32 - 1,
            },
            Point { x: 0, y: -1 },
        ));
    });
    grid.iter().enumerate().for_each(|(i, _)| {
        // left col going east
        starting_points.push((Point { x: 0, y: i as i32 }, Point { x: 1, y: 0 }));
        // right col going west
        starting_points.push((
            Point {
                x: grid[0].len() as i32 - 1,
                y: i as i32,
            },
            Point { x: -1, y: 0 },
        ));
    });
    let energized = starting_points
        .par_iter()
        .map(|(start, dir)| part_1(*start, *dir, &grid))
        .max()
        .unwrap();
    println!("energized {:?}", energized);
}
