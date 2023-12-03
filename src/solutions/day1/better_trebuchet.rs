pub fn get_first_digit(input: &str) -> usize {
    let nums = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut l_index = input.len() + 1;
    let mut l_value = None;
    for (i, num) in nums.iter().enumerate() {
        match input.find(num) {
            None => {}
            Some(loc) => {
                if loc < l_index {
                    l_index = loc;
                    l_value = Some(i + 1);
                }
            }
        };
    }
    let l_digit_index = input.chars().position(|char| char.is_numeric());
    let l_digit = l_digit_index.map(|ind| {
        input
            .chars()
            .nth(ind)
            .map(|c| c.to_digit(10).expect("l is definitely a digit"))
            .expect("char from position")
            .try_into()
            .unwrap()
    });

    match l_value {
        None => l_digit.expect("Input is invalid"),
        Some(value) => l_digit_index
            .map(|ind| {
                if ind < l_index {
                    l_digit.expect("Input is invalid")
                } else {
                    value
                }
            })
            .unwrap_or(value),
    }
}

pub fn get_last_digit(input: &str) -> usize {
    let nums = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut r_index: usize = 0;
    let mut r_value = None;
    for (i, num) in nums.iter().enumerate() {
        match input.rfind(num) {
            None => {}
            Some(loc) => {
                if loc > r_index {
                    r_index = loc;
                    r_value = Some(i + 1);
                }
            }
        };
    }
    let r_digit_index = input
        .chars()
        .rev()
        .position(|char| char.is_numeric())
        .map(|pos| input.len() - 1 - pos);
    let r_digit = r_digit_index.map(|ind| {
        input
            .chars()
            .nth(ind)
            .map(|c| c.to_digit(10).expect("l is definitely a digit"))
            .expect("char from position")
            .try_into()
            .unwrap()
    });

    match r_value {
        None => r_digit.expect("Input is invalid"),
        Some(value) => r_digit_index
            .map(|ind| {
                if ind > r_index {
                    r_digit.expect("Input is invalid")
                } else {
                    value
                }
            })
            .unwrap_or(value),
    }
}

pub fn get_calibration_value(input: &str) -> usize {
    return 10 * get_first_digit(input) + get_last_digit(input);
}

pub fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line: &str| get_calibration_value(line))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::solutions::day1::better_trebuchet::solve;

    #[test]
    fn it_works() {
        assert_eq!(
            solve(
                "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"
            ),
            281
        )
    }
}
