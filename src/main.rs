use std::fs;

mod solutions;

fn main() {
    let answer = fs::read_to_string("./src/inputs/races.txt")
        .map(|value| solutions::day6::big_race::solve(&value))
        .expect("Solving cubes failed");

    println!("{}", answer);
}
