use std::fs;

mod solutions;

fn main() {
    let answer = fs::read_to_string("./src/inputs/cubes.txt")
        .map(|value| solutions::day2::power_cubes::solve(&value))
        .expect("Solving cubes failed");

    println!("{}", answer);
}
