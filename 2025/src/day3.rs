pub fn part1(data: &Vec<String>) -> i64 {
    return data
        .iter()
        .map(|d| find_larget_voltage(d, 2).parse::<i64>().unwrap())
        .sum();
}

pub fn part2(data: &Vec<String>) -> i64 {
    return data
        .iter()
        .map(|d| find_larget_voltage(d, 12).parse::<i64>().unwrap())
        .sum();
}

fn find_max(data: &String, start: usize, end: usize) -> (u32, usize) {
    let mut max = 0;
    let mut idx = 0;
    for i in start..end {
        let n = data.chars().nth(i).unwrap().to_digit(10).unwrap();
        if n > max {
            max = n;
            idx = i;
        }
    }
    return (max, idx);
}

fn find_larget_voltage(data: &String, size: usize) -> String {
    let mut idx = 0;
    let mut sb = String::new();
    for i in (0..size).rev() {
        let (max, max_idx) = find_max(data, idx, data.len() - i);
        idx = max_idx + 1;
        sb.push(max.to_string().chars().next().unwrap());
    }
    return sb;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_find_larget_voltage() {
        struct Case {
            given: String,
            size: usize,
            want: String,
        }

        let cases: Vec<Case> = vec![
            Case {
                given: String::from("987654321111111"),
                size: 2,
                want: String::from("98"),
            },
            Case {
                given: String::from("811111111111119"),
                size: 2,
                want: String::from("89"),
            },
            Case {
                given: String::from("234234234234278"),
                size: 2,
                want: String::from("78"),
            },
            Case {
                given: String::from("818181911112111"),
                size: 2,
                want: String::from("92"),
            },
        ];
        for c in cases {
            assert_eq!(find_larget_voltage(&c.given, c.size), c.want);
        }
    }

    #[test]
    fn day3_part1() {
        struct Case {
            given: Vec<String>,
            want: i64,
        }

        let cases: Vec<Case> = vec![Case {
            given: vec![
                String::from("987654321111111"),
                String::from("811111111111119"),
                String::from("234234234234278"),
                String::from("818181911112111"),
            ],
            want: 357,
        }];
        for c in cases {
            assert_eq!(part1(&c.given), c.want);
        }
    }

    #[test]
    fn day3_part2() {
        struct Case {
            given: Vec<String>,
            want: i64,
        }

        let cases: Vec<Case> = vec![Case {
            given: vec![
                String::from("987654321111111"),
                String::from("811111111111119"),
                String::from("234234234234278"),
                String::from("818181911112111"),
            ],
            want: 3121910778619i64,
        }];
        for c in cases {
            assert_eq!(part2(&c.given), c.want);
        }
    }
}
