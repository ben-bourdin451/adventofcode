use env_logger::Env;
use log::info;
use std::fs;

mod day1;
mod day2;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let day1 = &read_to_string("day1.txt");
    info!(
        "Day 1  - \tPart 1: {}\t\tPart 2: {}",
        day1::part1(day1),
        day1::part2(day1)
    ); // 992, 6133

    let day2 = &read_to_string("day2.txt");
    info!(
        "Day 2  - \tPart 1: {}\tPart 2: {}",
        day2::part1(&day2.first().unwrap()),
        day2::part2(&day2.first().unwrap())
    ); // 0, 0
}

// read file and return a vector of strings
fn read_to_string(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents.lines().map(|s| s.to_string()).collect()
}
