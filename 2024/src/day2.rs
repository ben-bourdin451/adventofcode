use core::num;

pub fn part1(data: &Vec<String>) -> i32 {
    let mut sum: i32 = 0;
    for line in data {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if is_safe(&nums) {
            sum += 1;
        }
    }
    sum
}

fn is_safe(nums: &Vec<i32>) -> bool {
    let mut ascending: bool = true;
    if nums[0] > nums[1] {
        ascending = false;
    }
    for i in 1..nums.len() {
        if nums[i - 1] > nums[i] && ascending {
            return false;
        }
        if nums[i - 1] < nums[i] && !ascending {
            return false;
        }
        let diff = nums[i - 1].abs_diff(nums[i]);
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}

fn is_safe_with_problem_dampener(nums: &Vec<i32>) -> bool {
    if is_safe(nums) {
        return true;
    }

    for i in 0..nums.len() {
        let mut c = nums.clone();
        c.remove(i);
        if is_safe(&c) {
            return true;
        }
    }
    false
}

pub fn part2(data: &Vec<String>) -> i32 {
    let mut sum: i32 = 0;
    for line in data {
        let mut nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if is_safe_with_problem_dampener(&nums) {
            sum += 1;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_part1() {
        struct Case {
            given: Vec<String>,
            want: i32,
        }

        let cases: Vec<Case> = vec![Case {
            given: vec![
                String::from("7 6 4 2 1"),
                String::from("1 2 7 8 9"),
                String::from("9 7 6 2 1"),
                String::from("1 3 2 4 5"),
                String::from("8 6 4 4 1"),
                String::from("1 3 6 7 9"),
            ],
            want: 2,
        }];
        for c in cases {
            assert_eq!(part1(&c.given), c.want);
        }
    }

    #[test]
    fn day2_is_safe() {
        struct Case {
            given: Vec<i32>,
            want: bool,
        }

        let cases: Vec<Case> = vec![
            Case {
                given: vec![7, 6, 4, 2, 1],
                want: true,
            },
            Case {
                given: vec![1, 2, 7, 8, 9],
                want: false,
            },
            Case {
                given: vec![9, 7, 6, 2, 1],
                want: false,
            },
            Case {
                given: vec![1, 3, 2, 4, 5],
                want: false,
            },
            Case {
                given: vec![8, 6, 4, 4, 1],
                want: false,
            },
            Case {
                given: vec![1, 3, 6, 7, 9],
                want: true,
            },
            Case {
                given: vec![42, 44, 47, 49, 51, 52, 54, 52],
                want: false,
            },
            Case {
                given: vec![24, 27, 30, 31, 32, 35, 36, 36],
                want: false,
            },
            Case {
                given: vec![80, 82, 85, 86, 87, 90, 94],
                want: false,
            },
        ];

        for c in cases {
            assert_eq!(is_safe(&c.given), c.want, "{:?}", c.given);
        }
    }

    #[test]
    fn day2_is_safe_with_problem_dampener() {
        struct Case {
            given: Vec<i32>,
            want: bool,
        }

        let cases: Vec<Case> = vec![
            Case {
                given: vec![7, 6, 4, 2, 1],
                want: true,
            },
            Case {
                given: vec![1, 2, 7, 8, 9],
                want: false,
            },
            Case {
                given: vec![9, 7, 6, 2, 1],
                want: false,
            },
            Case {
                given: vec![1, 3, 2, 4, 5],
                want: true,
            },
            Case {
                given: vec![8, 6, 4, 4, 1],
                want: true,
            },
            Case {
                given: vec![1, 3, 6, 7, 9],
                want: true,
            },
            Case {
                given: vec![42, 44, 47, 49, 51, 52, 54, 52],
                want: true,
            },
            Case {
                given: vec![24, 27, 30, 31, 32, 35, 36, 36],
                want: true,
            },
            Case {
                given: vec![80, 82, 85, 86, 87, 90, 94],
                want: true,
            },
        ];

        for c in cases {
            assert_eq!(
                is_safe_with_problem_dampener(&c.given),
                c.want,
                "{:?}",
                c.given
            );
        }
    }

    #[test]
    fn day2_part2() {
        struct Case {
            given: Vec<String>,
            want: i32,
        }

        let cases: Vec<Case> = vec![Case {
            given: vec![
                String::from("7 6 4 2 1"),
                String::from("1 2 7 8 9"),
                String::from("9 7 6 2 1"),
                String::from("1 3 2 4 5"),
                String::from("8 6 4 4 1"),
                String::from("1 3 6 7 9"),
            ],
            want: 4,
        }];
        for c in cases {
            assert_eq!(part2(&c.given), c.want);
        }
    }
}
