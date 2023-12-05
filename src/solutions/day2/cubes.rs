use std::cmp::max;

struct Pull {
    red: u32,
    green: u32,
    blue: u32,
}

impl Pull {
    fn new() -> Pull {
        Pull {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    fn max_merge(first: Pull, second: Pull) -> Pull {
        Pull {
            red: max(first.red, second.red),
            green: max(first.green, second.green),
            blue: max(first.blue, second.blue),
        }
    }
    fn eligible(pull: Pull) -> bool {
        pull.red <= 12 && pull.green <= 13 && pull.blue <= 14
    }
}

pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (game_info, game_data) = line.trim().split_once(':').unwrap();
            let (_, game_number_str) = game_info.split_once(' ').unwrap();
            let game_number: u32 = game_number_str.parse().unwrap();
            let eligible = game_data
                .split(';')
                .map(|single_game| {
                    single_game.split(',').fold(
                        Pull::new(),
                        |mut current_pull, single_ball_type| {
                            let (amount_str, color) =
                                single_ball_type.trim().split_once(' ').unwrap();
                            let amount: u32 = amount_str.parse().unwrap();
                            match color {
                                "red" => current_pull.red = amount,
                                "green" => current_pull.green = amount,
                                "blue" => current_pull.blue = amount,
                                _ => {}
                            };
                            current_pull
                        },
                    )
                })
                .all(Pull::eligible);
            if eligible {
                game_number
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::solutions::day2::cubes::solve;

    #[test]
    fn it_works() {
        assert_eq!(
            solve(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            ),
            8
        )
    }
}
