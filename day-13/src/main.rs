use itertools::Itertools;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
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

fn search_hor(mut rows: Vec<&str>) -> i32 {
    rows.iter().for_each(|x| println!("{:?}", x));
    for j in 1..rows.len() {
        let left = &rows.clone()[..j];
        let right = &mut rows[j..];

        let count: i32 = left
            .iter()
            .rev()
            .zip_longest(right.iter())
            .map(|x| {
                x.both()
                    .and_then(|(l, r)| {
                        let left = l.chars().rev().collect_vec();
                        let right = r.chars().collect_vec();
                        let count: i32 = left
                            .iter()
                            .rev()
                            .zip_longest(right.iter())
                            .map(|x| match x {
                                x if x.clone().both().is_some() => {
                                    match x.both().and_then(|(l, r)| Some(l != r)).unwrap_or(false)
                                    {
                                        true => 1,
                                        false => 0,
                                    }
                                }
                                _ => 0,
                            })
                            .sum();
                        Some(count)
                    })
                    .unwrap_or(0)
            })
            .sum();

        if count == 1 {
            println!("h {:?}", j);
            return j as i32;
        }
        println!("h {:?}", count);
    }
    0
}
fn main() {
    let input = include_str!("./input.txt");
    let puzzles = input.split("\n\n").collect::<Vec<_>>();

    let mut sum = 0;
    for puzzle in puzzles {
        let rows: Vec<&str> = puzzle.lines().collect::<Vec<_>>();
        let t = transpose(
            puzzle
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec(),
        );
        let t = t
            .iter()
            .map(|line| line.iter().collect::<String>())
            .collect_vec();
        let c: Vec<&str> = t.iter().map(|line| line.as_str()).collect_vec();

        let v = search_hor(c);
        if v > 0 {
            sum += v;
        } else {
            sum += 100 * search_hor(rows);
        }
    }

    println!("Part 1: {}", sum);
}
