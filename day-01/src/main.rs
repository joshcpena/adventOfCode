fn getFirstNumber(line: &str) -> char {
    let first_num = line
        .find(|c: char| c.is_numeric())
        .map(|c| line.chars().nth(c).unwrap())
        .unwrap();
    first_num
}

fn getLastNumber(line: &str) -> char {
    let last_num = line
        .rfind(|c: char| c.is_numeric())
        .map(|c| line.chars().nth(c).unwrap())
        .unwrap();
    last_num
}
const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
fn bTrimForNumber(line: &str) -> &str {
    // trim left until num
    let mut line = line;
    while !NUMS.iter().any(|&num| line.starts_with(num))
        && !line.chars().next().unwrap().is_numeric()
    {
        line = &line[1..];
    }
    // trim right until num
    while !NUMS.iter().any(|&num| line.ends_with(num)) && !line.chars().last().unwrap().is_numeric()
    {
        line = &line[..line.len() - 1];
    }
    line
}

fn main() {
    let input = include_str!("../input.txt").split("\n");
    let mut res = 0;
    let mut res2 = 0;
    let sum: u32 = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| vec.first().unwrap() * 10 + vec.last().unwrap())
        .sum();
    println!("{:?}", sum);

    for mut line in input {
        let mut num = String::from("");
        num.push(getFirstNumber(line));
        num.push(getLastNumber(line));
        res += num.parse::<i32>().unwrap();
        let trimmed_line = bTrimForNumber(line);
        let mut first = 0;
        let mut second = 0;
        for (i, num) in NUMS.iter().enumerate() {
            if trimmed_line.starts_with(num) {
                // 0 to 1 index it
                first = (i + 1) as i32;
            } else if trimmed_line.chars().next().unwrap().is_numeric() {
                first = trimmed_line
                    .chars()
                    .next()
                    .unwrap()
                    .to_string()
                    .parse::<i32>()
                    .unwrap()
            }
            if trimmed_line.ends_with(num) {
                // 0 to 1 index it
                second = (i + 1) as i32;
            } else if trimmed_line.chars().last().unwrap().is_numeric() {
                second = trimmed_line
                    .chars()
                    .last()
                    .unwrap()
                    .to_string()
                    .parse::<i32>()
                    .unwrap()
            }
        }
        res2 += first * 10 + second;
    }
    println!("{:?}", res);
    println!("{:?}", res2);
}
