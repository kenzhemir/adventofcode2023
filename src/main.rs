use std::fs;

mod solutions;

fn main() {
    let answer = fs::read_to_string("./src/inputs/scratchpads.txt")
        .map(|value| solutions::day4::scratchcards_copy::solve(&value))
        .expect("Solving cubes failed");

    println!("{}", answer);
}
