use std::fs;

mod solutions;

fn main() {
    let answer = fs::read_to_string("./src/inputs/engine_schematic.txt")
        .map(|value| solutions::day3::gear_ratio::solve(&value))
        .expect("Solving cubes failed");

    println!("{}", answer);
}
