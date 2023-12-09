use num::integer::lcm;
use std::collections::HashMap;
fn get_steps<'a>(
    mut node: &'a &'a str,
    map: &'a HashMap<&'a str, (&'a str, &'a str)>,
    instructions: &'a str,
) -> usize {
    let mut steps = 0usize;
    loop {
        for c in instructions.chars() {
            steps += 1;
            match c {
                'R' => node = &map.get(node).unwrap().1,
                'L' => node = &map.get(node).unwrap().0,
                _ => panic!("Unknown instruction: {:?}", c),
            }
            if node.ends_with("Z") {
                return steps;
            }
        }
    }
}
fn main() {
    let (instructions, mappings) = include_str!("./input.txt").split_once("\n\n").unwrap();
    let x = mappings.lines().fold(HashMap::new(), |mut acc, line| {
        let (key, value) = line.split_once(" = ").unwrap();
        let value = value
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .expect("Value tuple");
        acc.insert(key, value);
        acc
    });
    let current_nodes = x.keys().filter(|k| k.ends_with("A")).collect::<Vec<_>>();
    let steps = current_nodes
        .iter()
        .map(|node| get_steps(node, &x, instructions))
        .fold(1, |acc, x| lcm(acc, x));
    println!("steps: {:?}", steps);
}
