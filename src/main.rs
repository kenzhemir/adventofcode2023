use std::fs;

mod solutions;

fn main() {
    let answer = fs::read_to_string("./src/inputs/almanac.txt")
        .map(|value| solutions::day5::plenty_seed_locations::solve(&value))
        .expect("Solving cubes failed");

    println!("{}", answer);
}
