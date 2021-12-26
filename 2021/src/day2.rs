pub fn part1(data: &Vec<String>) -> i32 {
    let mut horiz = 0;
    let mut depth = 0;

    for inst in data {
        let s: Vec<&str> = inst.split(' ').collect();
        let x: i32 = s[1].to_string().parse().unwrap();
        if s[0] == "forward" {
            horiz += x;
        } else if s[0] == "up" {
            depth -= x;
        } else if s[0] == "down" {
            depth += x;
        }
    }

    horiz * depth
}

pub fn part2(data: &Vec<String>) -> i32 {
    let mut aim = 0;
    let mut horiz = 0;
    let mut depth = 0;

    for inst in data {
        let s: Vec<&str> = inst.split(' ').collect();
        let x: i32 = s[1].to_string().parse().unwrap();
        if s[0] == "forward" {
            horiz += x;
            depth += aim * x;
        } else if s[0] == "up" {
            aim -= x;
        } else if s[0] == "down" {
            aim += x;
        }
    }

    horiz * depth
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
                String::from("forward 5"),
                String::from("down 5"),
                String::from("forward 8"),
                String::from("up 3"),
                String::from("down 8"),
                String::from("forward 2"),
            ],
            want: 150,
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
                String::from("forward 5"),
                String::from("down 5"),
                String::from("forward 8"),
                String::from("up 3"),
                String::from("down 8"),
                String::from("forward 2"),
            ],
            want: 900,
        }];
        for c in cases {
            assert_eq!(part2(&c.given), c.want);
        }
    }
}
