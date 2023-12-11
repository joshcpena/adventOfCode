fn main() {
    let input = include_str!("./input.txt");
    let result: i32 = input
        .lines()
        .map(|line| {
            let mut diffs: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            let mut prev_diffs: Vec<Vec<i32>> = vec![diffs.clone()];
            while diffs.iter().any(|x| x != &0) {
                diffs = diffs
                    .iter()
                    .enumerate()
                    .filter_map(|(i, _)| match i {
                        0 => None,
                        _ => Some(diffs[i] - diffs[i - 1]),
                    })
                    .collect();
                prev_diffs.push(diffs.clone());
            }
            prev_diffs
                .iter()
                .rev()
                .fold(0, |acc: i32, diff_vec| diff_vec[0] - acc)
        })
        .sum();

    println!("result: {:?}", result);
}
