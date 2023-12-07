pub fn solve(input: &str) -> f64 {
    let time: f64 = input
        .trim()
        .lines()
        .nth(0)
        .and_then(|line| {
            line.trim().split_once(':').map(|(_, record)| {
                record
                    .trim()
                    .chars()
                    .filter(|char| char.is_digit(10))
                    .collect::<String>()
                    .parse()
                    .unwrap()
            })
        })
        .unwrap();
    let destination: f64 = input
        .trim()
        .lines()
        .nth(1)
        .and_then(|line| {
            line.trim().split_once(':').map(|(_, record)| {
                record
                    .trim()
                    .chars()
                    .filter(|char| char.is_digit(10))
                    .collect::<String>()
                    .parse()
                    .unwrap()
            })
        })
        .unwrap();

    let mut min_thres = (time - (time * time - 4.0 * destination).sqrt()) / 2.0;
    let max_thres = (time + (time * time - 4.0 * destination).sqrt()) / 2.0;
    if min_thres.ceil() == min_thres {
        min_thres = min_thres + 1.0;
    }
    max_thres.ceil() - min_thres.ceil()
}

#[cfg(test)]
mod tests {
    use crate::solutions::day6::big_race::solve;

    #[test]
    fn it_works() {
        assert_eq!(
            solve(
                "
                Time:      7  15   30
                Distance:  9  40  200
                "
            ),
            71503.0
        )
    }
}
