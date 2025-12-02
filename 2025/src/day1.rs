pub fn part1(data: &Vec<String>) -> i32 {
    let mut dial = 50;
    let mut count = 0;
    for instruction in data {
        let direction = instruction.chars().next().unwrap();

        // strip the first character and parse the rest as an integer
        let steps = instruction
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        if direction == 'L' {
            dial -= steps;
        } else {
            dial += steps;
        }

        dial = dial % 100;
        if dial == 0 {
            count += 1;
        }
    }

    return count;
}

pub fn part2(data: &Vec<String>) -> i32 {
    let mut dial = 50;
    let mut count = 0;
    for instruction in data {
        let direction = instruction.chars().next().unwrap();

        // strip the first character and parse the rest as an integer
        let mut steps = instruction
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        // remove multiples of 100 from steps
        let q: i32 = (steps / 100) as i32;
        if q > 0 {
            steps -= q * 100;
            count += q;
        }

        // count the number of times the dial passes through 0
        for _ in 0..steps {
            if direction == 'L' {
                dial -= 1;
            } else {
                dial += 1;
            }
            if dial == 0 {
                count += 1;
            }
        }

        // reset dial overflow
        if dial <= -100 || dial >= 100 {
            dial = dial % 100;
            count += 1;
        }
    }

    return count;
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
                String::from("L68"),
                String::from("L30"),
                String::from("R48"),
                String::from("L5"),
                String::from("R60"),
                String::from("L55"),
                String::from("L1"),
                String::from("L99"),
                String::from("R14"),
                String::from("L82"),
            ],
            want: 3,
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

        let cases: Vec<Case> = vec![
            Case {
                given: vec![
                    String::from("L68"),
                    String::from("L30"),
                    String::from("R48"),
                    String::from("L5"),
                    String::from("R60"),
                    String::from("L55"),
                    String::from("L1"),
                    String::from("L99"),
                    String::from("R14"),
                    String::from("L82"),
                ],
                want: 6,
            },
            Case {
                given: vec![String::from("L400")],
                want: 4,
            },
            Case {
                given: vec![String::from("R400")],
                want: 4,
            },
            Case {
                given: vec![
                    String::from("L18"),  // (0) 50-18 = 32
                    String::from("L652"), // (7) 32-652 = -620 = -20
                    String::from("R7"),   // (0) -20+7 = -13
                    String::from("L888"), // (9) -13-888 = -901 = -1
                    String::from("R763"), // (8) -1+763 = 762 = 62
                    String::from("L24"),  // (0) 62-24 = 38
                ],
                want: 24, // 7 + 9 + 8
            },
        ];
        for c in cases {
            assert_eq!(part2(&c.given), c.want);
        }
    }
}
