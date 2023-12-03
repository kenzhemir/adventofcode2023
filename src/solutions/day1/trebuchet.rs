pub fn get_first_digit(input: &str) -> u32 {
    return input
        .chars()
        .find(|char| char.is_numeric())
        .and_then(|char| char.to_digit(10))
        .unwrap();
}

pub fn get_last_digit(input: &str) -> u32 {
    return input
        .chars()
        .rev()
        .find(|char| char.is_numeric())
        .and_then(|char| char.to_digit(10))
        .unwrap();
}

pub fn get_calibration_value(input: &str) -> u32 {
    return 10 * get_first_digit(input) + get_last_digit(input);
}

pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line: &str| get_calibration_value(line))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::solutions::day1::trebuchet::solve;

    #[test]
    fn it_works() {
        assert_eq!(
            solve(
                "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet",
            ),
            142
        )
    }
}
