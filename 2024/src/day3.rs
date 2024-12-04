use regex::Regex;

pub fn part1(data: &String) -> i32 {
    let mut sum: i32 = 0;
    let r1 = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for cap in r1.captures_iter(data) {
        let a: i32 = cap[1].parse().unwrap();
        let b: i32 = cap[2].parse().unwrap();
        sum += a * b;
    }
    sum
}

pub fn part2(data: &String) -> i32 {
    let mut input = data.clone();
    remove_dont(&mut input);
    part1(&input)
}

fn remove_dont(data: &mut String) {
    // println!("data: {}", data);
    let i = data.find("don't()");
    if i.is_some() {
        // println!("found `don't()` at: {}", i.unwrap());
        let mut rest = data.split_off(i.unwrap());
        // println!("rest: {}", rest);
        let j = rest.find("do()");
        if j.is_some() {
            // println!("found `do` at: {}", j.unwrap());
            let dos = rest.split_off(j.unwrap());
            data.push_str(&dos);
        }
    }
    // println!("data: {}", data);
    // recurse if we find more don't instructions
    if data.find("don't()").is_some() {
        return remove_dont(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_part1() {
        struct Case {
            given: String,
            want: i32,
        }

        let cases: Vec<Case> = vec![Case {
            given: String::from(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
            ),
            want: 161,
        }];
        for c in cases {
            assert_eq!(part1(&c.given), c.want);
        }
    }

    #[test]
    fn day3_part2() {
        struct Case {
            given: String,
            want: i32,
        }

        let cases: Vec<Case> = vec![Case {
            given: String::from(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
            ),
            want: 48,
        }];
        for c in cases {
            assert_eq!(part2(&c.given), c.want);
        }
    }

    #[test]
    fn day3_remove_dont() {
        struct Case {
            given: String,
            want: String,
        }

        let cases: Vec<Case> = vec![
            Case {
                given: String::from(
                    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
                ),
                want: String::from("xmul(2,4)&mul[3,7]!^do()?mul(8,5))"),
            },
            Case {
                given: String::from(
                    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))don't()_mul(5,5)+mul(32,64](mul(11,8)",
                ),
                want: String::from("xmul(2,4)&mul[3,7]!^do()?mul(8,5))"),
            },
						Case {
                given: String::from(
                    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
                ),
                want: String::from("xmul(2,4)&mul[3,7]!^do()?mul(8,5))do()?mul(8,5))"),
            },
        ];
        for c in cases {
            let mut got = c.given.clone();
            remove_dont(&mut got);
            assert_eq!(got, c.want);
        }
    }
}
