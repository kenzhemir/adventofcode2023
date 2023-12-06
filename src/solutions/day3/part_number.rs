fn is_symbol(arr: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    i < arr.len() && j < arr[i].len() && arr[i][j] != '.' && !arr[i][j].is_digit(10)
}

fn neighbours(i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut result = vec![(i, j + 1), (i + 1, j), (i + 1, j + 1)];
    if i > 0 {
        result.push((i - 1, j));
        result.push((i - 1, j + 1));
    };
    if j > 0 {
        result.push((i, j - 1));
        result.push((i + 1, j - 1));
    }
    if i > 0 && j > 0 {
        result.push((i - 1, j - 1))
    }
    result
}

pub fn solve(input: &str) -> u32 {
    let items: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let n = items.len();
    let m = items.first().unwrap().len();

    let mut sum = 0;

    for i in 0..n {
        let mut curr_number = 0;
        let mut eligible: bool = false;
        for j in 0..m {
            if items[i][j].is_digit(10) {
                if neighbours(i, j)
                    .iter()
                    .any(|(a, b)| is_symbol(&items, a.clone(), b.clone()))
                {
                    eligible = true
                }
                curr_number = curr_number * 10
                    + items[i][j]
                        .to_digit(10)
                        .expect("to be digit because of if check")
            }
            if j + 1 == m || !items[i][j + 1].is_digit(10) {
                if eligible {
                    sum = sum + curr_number
                }
                curr_number = 0;
                eligible = false;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::solutions::day3::part_number::solve;

    #[test]
    fn it_works() {
        assert_eq!(
            solve(
                "467..114..
                ...*......
                ..35..633.
                ......#...
                617*......
                .....+.58.
                ..592.....
                ......755.
                ...$.*....
                .664.598..",
            ),
            4361
        )
    }
}
