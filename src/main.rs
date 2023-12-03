use std::fs;

mod solutions;

fn main() {
    let answer = fs::read_to_string("./src/inputs/trebuchet.txt")
        .map(|value| solutions::day1::better_trebuchet::solve(&value))
        .expect("Solving trebuchet failed");

    println!("{}", answer);
}
