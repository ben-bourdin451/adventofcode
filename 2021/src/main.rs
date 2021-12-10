mod day1;

use std::fs;

fn main() {
    let day1_input: Vec<i32> = fs::read_to_string("day1_input.txt")
        .expect("Something went wrong reading the file")
        .split('\n')
        .filter_map(|s| s.parse().ok())
        .collect();

    let ans1_1 = day1::part1(&day1_input);
    assert_eq!(ans1_1, 1713);
    println!("day 1 part 1 {}", ans1_1);

    let ans1_2 = day1::part2(&day1_input);
    assert_eq!(ans1_2, 0);
    println!("day 1 part 2 {}", ans1_2);
}
