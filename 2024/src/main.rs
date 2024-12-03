mod day1;
mod day2;

use std::fs;

fn main() {
    let day1 = &read_to_string("day1_input.txt");
    println!("Day 1 - Part 1 {}", day1::part1(day1));
    println!("Day 1 - Part 2 {}", day1::part2(day1));

    let day2 = &read_to_string("day2_input.txt");
    println!("Day 2 - Part 1 {}", day2::part1(day2));
    println!("Day 2 - Part 2 {}", day2::part2(day2));
}

// read file and return a vector of strings
fn read_to_string(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents.lines().map(|s| s.to_string()).collect()
}
