use log::info;
use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod logging;

fn main() {
    logging::init_logging();

    let day1 = &read_to_string("day1.txt");
    info!(
        "Day 1  - \tPart 1: {}\t\tPart 2: {}",
        day1::part1(day1),
        day1::part2(day1)
    );

    let day2 = &read_to_string("day2.txt");
    info!(
        "Day 2  - \tPart 1: {}\tPart 2: {}",
        day2::part1(&day2.first().unwrap()),
        day2::part2(&day2.first().unwrap())
    );

    let day3 = &read_to_string("day3.txt");
    info!(
        "Day 3  - \tPart 1: {}\tPart 2: {}",
        day3::part1(day3),
        day3::part2(day3)
    );

    let day4 = &read_to_string("day4.txt");
    info!(
        "Day 4  - \tPart 1: {}\tPart 2: {}",
        day4::part1(day4),
        day4::part2(day4)
    );
}

// read file and return a vector of strings
fn read_to_string(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents.lines().map(|s| s.to_string()).collect()
}
