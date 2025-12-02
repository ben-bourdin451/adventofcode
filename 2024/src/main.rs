mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

use std::fs;

fn main() {
    let day1 = &read_to_string("day1_input.txt");
    println!("Day 1 - Part 1 {}", day1::part1(day1));
    println!("Day 1 - Part 2 {}", day1::part2(day1));

    let day2 = &read_to_string("day2_input.txt");
    println!("Day 2 - Part 1 {}", day2::part1(day2));
    println!("Day 2 - Part 2 {}", day2::part2(day2));

    let day3 = &read_to_string("day3_input.txt")[0];
    println!("Day 3 - Part 1 {}", day3::part1(day3));
    println!("Day 3 - Part 2 {}", day3::part2(day3));

    let day4 = &read_to_string("day4_input.txt");
    println!("Day 4 - Part 1 {}", day4::part1(day4));
    println!("Day 4 - Part 2 {}", day4::part2(day4));

    let day5 = &read_to_string("day5_input.txt");
    println!("Day 5 - Part 1 {}", day5::part1(day5));
    println!("Day 5 - Part 2 {}", day5::part2(day5));

    let day6 = &read_to_string("day6_input.txt");
    println!("Day 6 - Part 1 {}", day6::part1(day6));
    println!("Day 6 - Part 2 {}", day6::part2(day6));
}

// read file and return a vector of strings
fn read_to_string(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents.lines().map(|s| s.to_string()).collect()
}
