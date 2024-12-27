mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
use std::fs;

fn main() {
    println!("day01-1: {}", day01::part1(&input("day01")));
    println!("day01-2: {}", day01::part2(&input("day01")));
    println!("day02-1: {}", day02::part1(&input("day02")));
    println!("day02-2: {}", day02::part2(&input("day02")));
    println!("day03-1: {}", day03::part1(&input("day03")));
    println!("day03-2: {}", day03::part2(&input("day03")));
    println!("day04-1: {}", day04::part1(&input("day04")));
    println!("day04-2: {}", day04::part2(&input("day04")));
    println!("day05-1: {}", day05::part1(&input("day05")));
    println!("day05-2: {}", day05::part2(&input("day05")));
    println!("day06-1: {}", day06::part1(&input("day06")));
    println!("day06-2: {}", day06::part2(&input("day06")));
    println!("day07-1: {}", day07::part1(&input("day07")));
    println!("day07-2: {}", day07::part2(&input("day07")));
}

fn input(file_name: &str) -> String {
    fs::read_to_string(format!("src/{}_input.txt", file_name))
        .expect("Should have been able to read the file")
}
