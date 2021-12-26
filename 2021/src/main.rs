mod day1;
mod day2;

use std::fs;

fn main() {
    day_1();
    day_2();
}

fn day_1() {
    println!("day 1");
    let data: Vec<i32> = fs::read_to_string("day1_input.txt")
        .expect("Something went wrong reading the file")
        .split('\n')
        .filter_map(|s| s.parse().ok())
        .collect();

    let ans1 = day1::part1(&data);
    assert_eq!(ans1, 1713);
    println!("\tpart 1 {}", ans1);

    let ans2 = day1::part2(&data);
    assert_eq!(ans2, 1734);
    println!("\tpart 2 {}", ans2);
}

fn day_2() {
    println!("day 2");
    let data: Vec<String> = fs::read_to_string("day2_input.txt")
        .expect("Something went wrong reading the file")
        .split('\n')
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let ans1 = day2::part1(&data);
    assert_eq!(ans1, 1882980);
    println!("\tpart 1 {}", ans1);

    let ans2 = day2::part2(&data);
    assert_eq!(ans2, 1971232560);
    println!("\tpart 2 {}", ans2);
}
