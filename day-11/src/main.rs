use std::cmp::{max, min};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn expand_universe_and_idxs(universe: &str) -> (Vec<Vec<char>>, Vec<usize>, Vec<usize>) {
    let mut col_has_galaxy: Vec<bool> = vec![false; universe.split_once("\n").unwrap().0.len()];
    let mut row_idxs: Vec<usize> = vec![];
    let mut expanded_universe: Vec<Vec<char>> = vec![];
    universe.lines().enumerate().for_each(|(row, line)| {
        line.chars().all(|c| c == '.').then(|| row_idxs.push(row));
        line.match_indices("#")
            .for_each(|(idx, _)| col_has_galaxy[idx] = true);
        expanded_universe.push(line.chars().collect());
    });
    let col_idxs = col_has_galaxy
        .iter()
        .enumerate()
        .filter_map(|(i, r)| match !r {
            true => Some(i),
            false => None,
        })
        .collect::<Vec<_>>();
    (expanded_universe, col_idxs, row_idxs)
}

fn get_galaxy_coordinates(universe: &Vec<Vec<char>>) -> Vec<Point> {
    let mut galaxy_coordinates: Vec<Point> = vec![];
    universe.iter().enumerate().for_each(|(row_idx, row)| {
        row.iter().enumerate().for_each(|(col_idx, col)| {
            if *col == '#' {
                galaxy_coordinates.push(Point {
                    y: row_idx,
                    x: col_idx,
                });
            }
        });
    });
    galaxy_coordinates
}

fn calc_min_n_distance(
    galaxy_coordinates: &Vec<Point>,
    point: &Point,
    expand_idxs: &(Vec<usize>, Vec<usize>), // (x, y)
    n_expansion: usize,
) -> usize {
    let n_expansion = n_expansion - 1;
    galaxy_coordinates
        .iter()
        .map(|galaxy| {
            let x_to_add = expand_idxs
                .0
                .iter()
                .filter(|x| min(galaxy.x, point.x) < **x && **x < max(galaxy.x, point.x))
                .count();
            let y_to_add = expand_idxs
                .1
                .iter()
                .filter(|y| min(galaxy.y, point.y) < **y && **y < max(galaxy.y, point.y))
                .count();
            let x_abs =
                (max(galaxy.x, point.x) + (x_to_add * n_expansion)) - min(galaxy.x, point.x);
            let y_abs =
                (max(galaxy.y, point.y) + (y_to_add * n_expansion)) - min(galaxy.y, point.y);
            x_abs + y_abs
        })
        .sum()
}

fn calc_sum_of_n_expansion_distances(
    galaxy_coordinates: &Vec<Point>,
    expand_idxs: &(Vec<usize>, Vec<usize>), // (x, y)
    n: usize,
) -> usize {
    galaxy_coordinates
        .iter()
        .map(|galaxy| {
            calc_min_n_distance(
                &galaxy_coordinates
                    .iter()
                    .filter_map(|point| match point != galaxy {
                        true => Some(*point),
                        false => None,
                    })
                    .collect::<Vec<_>>(),
                galaxy,
                expand_idxs, // (x, y)
                n,
            )
        })
        .sum()
}
fn main() {
    let (universe, x_idxs, y_idxs): (Vec<Vec<char>>, Vec<usize>, Vec<usize>) =
        expand_universe_and_idxs(&include_str!("./input.txt"));
    let galaxy_coordinates = get_galaxy_coordinates(&universe);
    println!(
        "result {:?}",
        calc_sum_of_n_expansion_distances(&galaxy_coordinates, &(x_idxs, y_idxs), 1_000_000) / 2
    );
}
