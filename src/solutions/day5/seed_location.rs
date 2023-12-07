pub fn solve(input: &str) -> u64 {
    let mut blocks = input.split("\n\n");
    let seed_info = blocks.next().unwrap();
    let (_, seed_ids) = seed_info.split_once(":").unwrap();
    let mut seeds: Vec<u64> = seed_ids
        .trim()
        .split(" ")
        .flat_map(|val| val.parse())
        .collect();
    while let Some(block) = blocks.next() {
        let dict: Vec<Vec<u64>> = block
            .lines()
            .skip(1)
            .map(|line| {
                line.trim()
                    .splitn(3, " ")
                    .flat_map(|num| num.parse::<u64>())
                    .collect()
            })
            .collect();
        seeds = seeds
            .iter()
            .map(|&seed| {
                dict.iter()
                    .find_map(|row| {
                        if row[1] <= seed && row[1] + row[2] > seed {
                            Some(row[0] + (seed - row[1]))
                        } else {
                            None
                        }
                    })
                    .unwrap_or(seed)
            })
            .collect()
    }
    *seeds.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::solutions::day5::seed_location::solve;

    #[test]
    fn it_works() {
        assert_eq!(
            solve(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
            ),
            35
        )
    }
}
