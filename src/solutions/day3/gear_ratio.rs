fn expand_digit(arr: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let mut left = y;
    while left > 0 && arr[x][left - 1].is_digit(10) {
        left = left - 1
    }
    let mut right = y;
    while right < arr.len() - 1 && arr[x][right + 1].is_digit(10) {
        right = right + 1
    }

    let str: String = arr[x][left..right + 1].iter().collect();
    str.parse().unwrap()
}

fn count_ratio(arr: &Vec<Vec<char>>, i: usize, j: usize) -> Option<u32> {
    let mut nums = vec![];
    // check left
    if j > 0 && arr[i][j - 1].is_digit(10) {
        nums.push(expand_digit(arr, i, j - 1))
    }
    // check right
    if j + 1 < arr.len() && arr[i][j + 1].is_digit(10) {
        nums.push(expand_digit(arr, i, j + 1))
    }
    // check top
    if i > 0 {
        if arr[i - 1][j].is_digit(10) {
            nums.push(expand_digit(arr, i - 1, j))
        } else {
            // check top left
            if j > 0 && arr[i - 1][j - 1].is_digit(10) {
                nums.push(expand_digit(arr, i - 1, j - 1))
            }
            // check top right
            if j + 1 < arr.len() && arr[i - 1][j + 1].is_digit(10) {
                nums.push(expand_digit(arr, i - 1, j + 1))
            }
        }
    }
    // check bottom
    if i + 1 < arr.first().unwrap().len() {
        if arr[i + 1][j].is_digit(10) {
            nums.push(expand_digit(arr, i + 1, j))
        } else {
            // check bottom left
            if j > 0 && arr[i + 1][j - 1].is_digit(10) {
                nums.push(expand_digit(arr, i + 1, j - 1))
            }
            // check bottom right
            if j + 1 < arr.len() && arr[i + 1][j + 1].is_digit(10) {
                nums.push(expand_digit(arr, i + 1, j + 1))
            }
        }
    }

    if nums.len() == 2 {
        Some(nums[0] * nums[1])
    } else {
        None
    }
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
        for j in 0..m {
            if items[i][j] == '*' {
                match count_ratio(&items, i, j) {
                    Some(val) => sum = sum + val,
                    None => {}
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::solutions::day3::gear_ratio::solve;

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
            467835
        )
    }
}
