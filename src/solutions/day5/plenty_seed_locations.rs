fn range_intersection(start_a: u64, len_a: u64, start_b: u64, len_b: u64) -> Option<(u64, u64)> {
    let end_a = start_a + len_a; // 79 - 93
    let end_b = start_b + len_b; // 50 - 98
    if start_a <= end_b && start_b <= end_a {
        if start_a < start_b {
            Some((start_b, std::cmp::min(end_a, end_b) - start_b))
        } else {
            Some((start_a, std::cmp::min(end_a, end_b) - start_a))
        }
    } else {
        None
    }
}

pub fn solve(input: &str) -> u64 {
    let mut blocks = input.split("\n\n");
    let seed_info = blocks.next().unwrap();
    let (_, seed_ids) = seed_info.split_once(":").unwrap();
    let seeds: Vec<u64> = seed_ids
        .trim()
        .split(" ")
        .flat_map(|val| val.parse())
        .collect();
    let mut seed_ranges: Vec<Vec<u64>> = seeds
        .chunks_exact(2)
        .map(|chunk: &[u64]| vec![chunk[0], chunk[1]])
        .collect();
    while let Some(block) = blocks.next() {
        println!("IN the block");
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

        // println!("--- seeds {:?}", seed_ranges);
        // println!("--- dicts {:?}", dict);
        seed_ranges = seed_ranges
            .iter()
            .flat_map(|seed_range| {
                let mut dicted = dict
                    .iter()
                    .filter_map(|dict_range| {
                        range_intersection(
                            seed_range[0],
                            seed_range[1],
                            dict_range[1],
                            dict_range[2],
                        )
                        .map(|(start, len)| {
                            (start, len, (dict_range[0] as i128 - dict_range[1] as i128))
                        })
                    })
                    .collect::<Vec<(u64, u64, i128)>>();
                dicted.sort_by(|&a, &b| a.0.cmp(&b.0));

                // println!("--- ranges {:?}", dicted);

                let mut holes = get_holes(seed_range[0], seed_range[1], &dicted);
                dicted.append(&mut holes);
                let res = dicted
                    .iter()
                    .map(|&(from, len, offset)| vec![(from as i128 + offset) as u64, len])
                    .collect::<Vec<_>>();
                // println!("--- ranges w/o holes {:?}", dicted);
                res
            })
            .collect();
    }
    seed_ranges.iter().map(|range| range[0]).min().unwrap()
}

fn get_holes(from: u64, len: u64, dicted: &Vec<(u64, u64, i128)>) -> Vec<(u64, u64, i128)> {
    let mut result = vec![];
    let mut current_step = from;

    for &(from, len, _) in dicted {
        if current_step < from {
            result.push((current_step, from - current_step, 0))
        }
        current_step = from + len;
    }

    if current_step < from + len {
        result.push((current_step, from + len - current_step, 0))
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::solutions::day5::plenty_seed_locations::solve;

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
            46
        )
    }
}
