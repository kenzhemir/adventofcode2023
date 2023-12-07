pub fn solve(input: &str) -> f64 {
    let times: Vec<f64> = input
        .trim()
        .lines()
        .nth(0)
        .and_then(|line| {
            line.trim().split_once(':').map(|(_, records)| {
                records
                    .trim()
                    .split(' ')
                    .filter_map(|strnum| strnum.parse::<f64>().ok())
                    .collect()
            })
        })
        .unwrap();
    let destinations: Vec<f64> = input
        .trim()
        .lines()
        .nth(1)
        .and_then(|line| {
            line.trim().split_once(':').map(|(_, records)| {
                records
                    .trim()
                    .split(' ')
                    .filter_map(|strnum| strnum.parse::<f64>().ok())
                    .collect()
            })
        })
        .unwrap();

    times
        .iter()
        .zip(destinations.iter())
        .map(|(&time, &destination)| {
            let mut min_thres = (time - (time * time - 4.0 * destination).sqrt()) / 2.0;
            let max_thres = (time + (time * time - 4.0 * destination).sqrt()) / 2.0;
            if min_thres.ceil() == min_thres {
                min_thres = min_thres + 1.0;
            }
            max_thres.ceil() - min_thres.ceil()
        })
        .reduce(|a, b| a * b)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::solutions::day6::race::solve;

    #[test]
    fn it_works() {
        assert_eq!(
            solve(
                "
                Time:      7  15   30
                Distance:  9  40  200
                "
            ),
            288.0
        )
    }
}
