use std::cmp;
use std::collections::HashMap;

fn main() {
    let input: std::str::Split<'_, &str> = include_str!("./input.txt").split("\n");
    let mut res = 0i32;
    let mut res2 = 0i32;
    let color_maxes: HashMap<&str, i32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    for line in input {
        let split: Vec<&str> = line.split(": ").collect();
        let [game_num, all_games] = [split[0], split[1]];
        let game_id = String::from(game_num.split("Game ").collect::<Vec<&str>>()[1])
            .parse::<i32>()
            .unwrap();
        let games: Vec<&str> = all_games.split("; ").collect();
        let mut is_valid_game = true;
        let mut color_mins: HashMap<&str, i32> =
            HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        for (_, game) in games.iter().enumerate() {
            let rolls = game.split(", ").collect::<Vec<&str>>();
            for roll in rolls {
                let split_roll = roll.split(" ").collect::<Vec<&str>>();
                let [color, count] = [split_roll[1], split_roll[0]];
                let count = count.parse::<i32>().unwrap();
                color_mins.insert(color, cmp::max(*color_mins.get(color).unwrap(), count));
                let max = *color_maxes.get(&color).unwrap();
                if count > max {
                    is_valid_game = false;
                }
            }
        }
        res2 += color_mins.values().fold(1, |acc, b| acc * b);
        if is_valid_game {
            res += game_id;
        }
    }
    println!("Sum of valid game ids: {:?}", res);
    println!("Sum of POWERS: {:?}", res2);
}
