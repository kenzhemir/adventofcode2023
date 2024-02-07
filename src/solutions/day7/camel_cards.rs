use std::{cmp::Ordering, collections::HashMap};

pub fn solve(input: &str) -> u64 {
    let players: Vec<(&str, u64)> = input
        .lines()
        .map(|line| {
            line.trim()
                .split_once(' ')
                .map(|(hand, bid)| (hand, bid.parse::<u64>().unwrap()))
                .unwrap()
        })
        .collect();
    players.sort_by(|a, b| {
        let type_a = determine_type(a.0);
        let type_b = determine_type(b.0);
        if type_a < type_b {
            Ordering::Less
        } else if (type_a > type_b) {
            Ordering::Greater
        } else {
            a.0.chars()
        }
    });
    0
}

fn determine_type(hand: &str) -> u64 {
    let map = hand
        .chars()
        .fold(HashMap::default(), |mut acc: HashMap<char, usize>, item| {
            let entry = acc.entry(item).or_default();
            *entry += 1;
            acc
        });
    let mut values: Vec<&usize> = map.values().collect();
    values.sort();
    values.reverse();

    match values[0] {
        5 => 6,
        4 => 5,
        3 => *values[1] as u64 + 2,
        2 => *values[1] as u64,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::day7::camel_cards::solve;

    #[test]
    fn it_works() {
        assert_eq!(
            solve(
                "32T3K 765
                T55J5 684
                KK677 28
                KTJJT 220
                QQQJA 483"
            ),
            6440
        )
    }
}
