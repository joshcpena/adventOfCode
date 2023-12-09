use std::collections::BTreeMap;
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
struct Hand<'a> {
    cards: &'a str,
    bid: u32,
    map: BTreeMap<char, u32>,
    hand_type: HandType,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

impl Hand<'_> {
    fn get_hand(&mut self) {
        let joker_map = self.map.remove_entry(&'J').unwrap_or(('J', 0));
        let mut count_vec = self.map.values().cloned().collect::<Vec<u32>>();
        let max = count_vec.iter().max().unwrap_or(&0);
        if max == &5 || joker_map.1 == 5 {
            self.hand_type = HandType::FiveOfAKind;
            return;
        }
        let y = count_vec.iter().position(|&x| x == *max).unwrap_or(0);
        count_vec[y] += joker_map.1;
        self.hand_type = match count_vec {
            x if x.contains(&5) => HandType::FiveOfAKind,
            x if x.contains(&4) => HandType::FourOfAKind,
            x if x.contains(&3) && x.contains(&2) => HandType::FullHouse,
            x if x.contains(&3) => HandType::ThreeOfAKind,
            x if x.contains(&2) => {
                if x.iter().filter(|&x| x == &2).count() == 2 {
                    HandType::TwoPair
                } else {
                    HandType::Pair
                }
            }
            _ => HandType::HighCard,
        };
    }
    fn get_replaced_chars(&self) -> String {
        self.cards
            .replace("A", "Z")
            .replace("K", "Y")
            .replace("Q", "X")
            .replace("J", "1")
            .replace("T", "V")
    }
}
fn main() {
    let mut input = include_str!("./input.txt")
        .lines()
        .map(|line| {
            let [cards, bid] = <[&str; 2]>::try_from(line.split(" ").collect::<Vec<_>>())
                .expect("cards and bid in line");
            let map = cards.chars().fold(BTreeMap::new(), |mut acc, card| {
                *acc.entry(card).or_insert(0) += 1;
                acc
            });
            let mut hand = Hand {
                cards,
                bid: bid.parse::<u32>().expect("bid is numeric"),
                map,
                hand_type: HandType::HighCard,
            };
            hand.get_hand();
            hand
        })
        .collect::<Vec<_>>();
    input.sort_by(|a, b| {
        if a.hand_type == b.hand_type {
            let a_chars = a.get_replaced_chars();
            let b_chars = b.get_replaced_chars();
            let b_chars = b_chars.chars();
            let a_chars = a_chars.chars();
            for i in 0..a.cards.len() {
                if a_chars.clone().nth(i).unwrap() > b_chars.clone().nth(i).unwrap() {
                    return std::cmp::Ordering::Less;
                } else if a_chars.clone().nth(i).unwrap() < b_chars.clone().nth(i).unwrap() {
                    return std::cmp::Ordering::Greater;
                }
            }
            std::cmp::Ordering::Equal
        } else {
            a.hand_type.cmp(&b.hand_type)
        }
    });
    let res = input.iter().enumerate().fold(0u32, |mut acc, (i, hand)| {
        acc += hand.bid * (input.len() - i) as u32;
        acc
    });
    println!(" {:?}", res);
}
