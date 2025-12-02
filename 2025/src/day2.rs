use log::debug;

pub fn part1(data: &String) -> i64 {
    return count_invalid_ranges(data, is_invalid);
}

pub fn part2(data: &String) -> i64 {
    return count_invalid_ranges(data, is_invalid_part2);
}

fn count_invalid_ranges(data: &String, valid_fn: fn(&String) -> bool) -> i64 {
    let mut count: i64 = 0;
    let pairs: Vec<String> = data.split(",").map(|s| s.to_string()).collect();
    for pair in pairs {
        let parts: Vec<&str> = pair.split("-").collect();
        let left: i64 = parts[0].parse::<i64>().unwrap();
        let right: i64 = parts[1].parse::<i64>().unwrap();
        let invalid = find_invalid_in_range(left, right, valid_fn);
        count += invalid.iter().sum::<i64>();
    }
    return count;
}

fn is_invalid(data: &String) -> bool {
    if data.len() % 2 != 0 {
        return false;
    }

    // split string in half
    let half = data.len() / 2;
    let left = &data[..half];
    let right = &data[half..];

    return left == right;
}

fn find_invalid_in_range(left: i64, right: i64, valid_fn: fn(&String) -> bool) -> Vec<i64> {
    debug!("find_invalid_in_range({}, {})", left, right);
    let mut invalid = Vec::new();
    for i in left..right + 1 {
        if valid_fn(&i.to_string()) {
            debug!("\t\tfound invalid: {}", i);
            invalid.push(i);
        }
    }
    invalid
}

fn is_pattern_repeating(data: &String, pattern_len: usize) -> bool {
    let pattern = &data[..pattern_len];
    let mut i = pattern_len;
    let mut count = 1;
    while i <= data.len() - pattern_len {
        let next = &data[i..i + pattern_len];
        debug!("\tis_pattern_repeating: {} {}", pattern, next);
        if next != pattern {
            return false;
        }
        i += pattern_len;
        count += 1;
    }
    return count >= 2;
}

fn is_invalid_part2(data: &String) -> bool {
    debug!("is_invalid_part2({})", data);
    let mut pattern_len = 1;
    while pattern_len <= data.len() / 2 {
        if data.len() % pattern_len == 0 && is_pattern_repeating(data, pattern_len) {
            debug!("\tfound pattern: {}", &data[..pattern_len]);
            return true;
        }

        pattern_len += 1;
    }
    return false;
}

mod tests {
    use super::*;
    use env_logger::Env;

    fn init() {
        let _ = env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
            .is_test(true)
            .try_init();
    }

    #[test]
    fn day2_is_invalid() {
        init();
        struct Case {
            given: String,
            want: bool,
        }
        let cases: Vec<Case> = vec![
            Case {
                given: String::from("10"),
                want: false,
            },
            Case {
                given: String::from("101"),
                want: false,
            },
            Case {
                given: String::from("1188511880"),
                want: false,
            },
            Case {
                given: String::from("55"),
                want: true,
            },
            Case {
                given: String::from("123123"),
                want: true,
            },
        ];
        for c in cases {
            assert_eq!(is_invalid(&c.given), c.want);
        }
    }

    #[test]
    fn day2_is_invalid_part2() {
        init();
        struct Case {
            given: String,
            want: bool,
        }
        let cases: Vec<Case> = vec![
            Case {
                given: String::from("1"),
                want: false,
            },
            Case {
                given: String::from("12"),
                want: false,
            },
            Case {
                given: String::from("123"),
                want: false,
            },
            Case {
                given: String::from("11"),
                want: true,
            },
            Case {
                given: String::from("123123"),
                want: true,
            },
            Case {
                given: String::from("12341234"),
                want: true,
            },
            Case {
                given: String::from("123123123"),
                want: true,
            },
            Case {
                given: String::from("1212121212"),
                want: true,
            },
            Case {
                given: String::from("1111111"),
                want: true,
            },
        ];
        for c in cases {
            let got = is_invalid_part2(&c.given);
            assert_eq!(
                got, c.want,
                "given: {}, want: {}, got: {}",
                c.given, c.want, got
            );
        }
    }

    #[test]
    fn day2_find_invalid_in_range() {
        init();
        struct Case {
            given: (i64, i64),
            want: Vec<i64>,
        }

        let cases: Vec<Case> = vec![
            Case {
                given: (11, 22),
                want: vec![11, 22],
            },
            Case {
                given: (95, 115),
                want: vec![99],
            },
            Case {
                given: (998, 1012),
                want: vec![1010],
            },
            Case {
                given: (1188511880, 1188511890),
                want: vec![1188511885],
            },
            Case {
                given: (222220, 222224),
                want: vec![222222],
            },
            Case {
                given: (1698522, 1698528),
                want: vec![],
            },
            Case {
                given: (446443, 446449),
                want: vec![446446],
            },
            Case {
                given: (38593856, 38593862),
                want: vec![38593859],
            },
            Case {
                given: (2121212118, 2121212124),
                want: vec![],
            },
        ];
        for c in cases {
            assert_eq!(
                find_invalid_in_range(c.given.0, c.given.1, is_invalid),
                c.want
            );
        }

        let cases_part2: Vec<Case> = vec![
            Case {
                given: (11, 22),
                want: vec![11, 22],
            },
            Case {
                given: (95, 115),
                want: vec![99, 111],
            },
            Case {
                given: (998, 1012),
                want: vec![999, 1010],
            },
            Case {
                given: (1188511880, 1188511890),
                want: vec![1188511885],
            },
            Case {
                given: (222220, 222224),
                want: vec![222222],
            },
            Case {
                given: (1698522, 1698528),
                want: vec![],
            },
            Case {
                given: (446443, 446449),
                want: vec![446446],
            },
            Case {
                given: (38593856, 38593862),
                want: vec![38593859],
            },
            Case {
                given: (565653, 565659),
                want: vec![565656],
            },
            Case {
                given: (824824821, 824824827),
                want: vec![824824824],
            },
            Case {
                given: (2121212118, 2121212124),
                want: vec![2121212121, 0],
            },
        ];
        for c in cases_part2 {
            assert_eq!(
                find_invalid_in_range(c.given.0, c.given.1, is_invalid_part2),
                c.want
            );
        }
    }

    #[test]
    fn day2_part1() {
        init();
        struct Case {
            given: String,
            want: i64,
        }
        let cases: Vec<Case> = vec![Case {
            given: String::from(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
            ),
            want: 1227775554i64,
        }];
        for c in cases {
            assert_eq!(part1(&c.given), c.want);
        }
    }

    #[test]
    fn day2_part2() {
        init();
        struct Case {
            given: String,
            want: i64,
        }
        let cases: Vec<Case> = vec![Case {
            given: String::from(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
            ),
            want: 4174379265i64,
        }];
        for c in cases {
            assert_eq!(part2(&c.given), c.want);
        }
    }
}
