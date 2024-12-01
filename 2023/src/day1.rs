pub fn part1(data: &Vec<String>) -> i32 {
    let mut r = 0;
    for d in data {
        let mut first = String::from("");
        let mut last = String::from("");
        for c in d.chars() {
            if c.is_digit(10) {
                if first.eq("") {
                    first = c.to_string();
                } else {
                    last = c.to_string();
                }
            }
        }
        if last.eq("") {
            last = first.clone();
        }
        let digits = first + &last;
        r += digits.parse::<i32>().unwrap();
    }
    r
}

pub fn part2(data: &Vec<String>) -> i32 {
    0
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
                String::from("1abc2"),
                String::from("pqr3stu8vwx"),
                String::from("a1b2c3d4e5f"),
                String::from("treb7uchet"),
            ],
            want: 142,
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
                String::from("two1nine"),
                String::from("eightwothree"),
                String::from("abcone2threexyz"),
                String::from("xtwone3four"),
                String::from("4nineeightseven2"),
                String::from("zoneight234"),
                String::from("7pqrstsixteen"),
            ],
            want: 281,
        }];
        for c in cases {
            assert_eq!(part2(&c.given), c.want);
        }
    }
}
