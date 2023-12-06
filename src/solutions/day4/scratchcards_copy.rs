use std::thread::yield_now;

pub fn solve(input: &str) -> u32 {
    let tickets: Vec<u32> = input
        .lines()
        .map(|line| {
            let (_, card_data) = line.trim().split_once(':').unwrap();
            let (winning, owned) = card_data.split_once('|').unwrap();
            let owned_numbers: Vec<u32> = owned
                .trim()
                .split(' ')
                .flat_map(|val| val.parse())
                .collect();
            winning
                .trim()
                .split(' ')
                .flat_map(|val| val.parse::<u32>())
                .filter(|&winning_num| {
                    owned_numbers
                        .iter()
                        .find(|&&owned_num| winning_num == owned_num)
                        .is_some()
                })
                .count() as u32
        })
        .collect();

    let mut results = vec![];
    for i in 0..tickets.len() {
        let mut copies = 1;
        for j in 0..i {
            if tickets[j] + j as u32 >= i as u32 {
                copies += results[j];
            }
        }
        results.push(copies)
    }

    results.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::solutions::day4::scratchcards_copy::solve;

    #[test]
    fn it_works() {
        assert_eq!(
            solve(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
            ),
            30
        )
    }
}
