mod day01;
use std::fs;

fn main() {
    println!("day01-1: {}", day01::part1(&input("day01")));
    println!("day01-2: {}", day01::part2(&input("day01")));
}

fn input(file_name: &str) -> String {
    fs::read_to_string(format!("src/{}_input.txt", file_name))
        .expect("Should have been able to read the file")
}
