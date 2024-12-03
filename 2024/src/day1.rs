use std::collections::HashMap;

pub fn part1(data: &Vec<String>) -> i32 {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in data {
        let mut nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        left.push(nums[0]);
        right.push(nums[1]);
    }
    left.sort();
    right.sort();
    let mut sum: i32 = 0;
    for i in 0..left.len() {
        sum += left[i].abs_diff(right[i]) as i32;
    }
    sum
}

pub fn part2(data: &Vec<String>) -> i32 {
    let mut left: Vec<i32> = Vec::new();
    let mut right: HashMap<i32, i32> = HashMap::new();

    for line in data {
        let mut nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        left.push(nums[0]);
        if right.contains_key(&nums[1]) {
            right.insert(nums[1], right[&nums[1]] + 1);
        } else {
            right.insert(nums[1], 1);
        }
    }

    let mut sum: i32 = 0;
    for i in 0..left.len() {
        if right.contains_key(&left[i]) {
            sum += left[i] * right[&left[i]];
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_part1() {
        struct Case {
            given: Vec<String>,
            want: i32,
        }

        let cases: Vec<Case> = vec![Case {
            given: vec![
                String::from("3   4"),
                String::from("4   3"),
                String::from("2   5"),
                String::from("1   3"),
                String::from("3   9"),
                String::from("3   3"),
            ],
            want: 11,
        }];
        for c in cases {
            assert_eq!(part1(&c.given), c.want);
        }
    }

    #[test]
    fn day1_part2() {
        struct Case {
            given: Vec<String>,
            want: i32,
        }

        let cases: Vec<Case> = vec![Case {
            given: vec![
                String::from("3   4"),
                String::from("4   3"),
                String::from("2   5"),
                String::from("1   3"),
                String::from("3   9"),
                String::from("3   3"),
            ],
            want: 31,
        }];
        for c in cases {
            assert_eq!(part2(&c.given), c.want);
        }
    }
}
