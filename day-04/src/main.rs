use std::collections::BTreeMap;
fn main() {
    let games = include_str!("./input.txt");
    let scores: BTreeMap<i32, i32> = games
        .lines()
        .map(|line| {
            let it = line.chars();
            let id: i32 = it
                .clone()
                .skip_while(|c| !c.is_ascii_digit())
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<i32>()
                .expect("ids are digits");
            let winners = it
                .clone()
                .skip_while(|c| *c != ':')
                .skip(1)
                .take_while(|c| *c != '|')
                .collect::<String>()
                .split(" ")
                .filter_map(|num| match num {
                    "" => None,
                    _ => Some(num.parse::<i32>().expect("winners are digits")),
                })
                .collect::<Vec<i32>>();
            let scratched_nums = it
                .clone()
                .skip_while(|c| *c != '|')
                .skip(1)
                .take_while(|c| c.is_ascii())
                .collect::<String>()
                .split(" ")
                .filter_map(|num| match num {
                    "" => None,
                    _ => Some(num.parse::<i32>().expect("winners are digits")),
                })
                .collect::<Vec<i32>>();
            let sum_winners =
                scratched_nums.iter().fold(
                    0,
                    |acc, x| {
                        if winners.contains(x) {
                            acc + 1
                        } else {
                            acc
                        }
                    },
                );
            (id, sum_winners)
        })
        .collect::<BTreeMap<i32, i32>>();
    let mut res: BTreeMap<i32, i32> = BTreeMap::new();
    scores.iter().for_each(|(k, v)| {
        res.insert(*k, res.get(k).unwrap_or(&0) + 1);
        for _ in 1..=*res.get(k).unwrap() {
            for i in 1..=*v {
                res.insert(*k + i, res.get(&(*k + i)).unwrap_or(&0) + 1);
            }
        }
    });
    println!("part 2 {:?}:", res.values().sum::<i32>());
}
