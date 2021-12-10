pub fn part1(data: &Vec<i32>) -> i32 {
    let mut prev = 0;
    let mut r = -1;
    for d in data {
        if *d > prev {
            r += 1;
        }
        prev = *d;
    }
    r
}

pub fn part2(data: &Vec<i32>) -> i32 {
    let mut prev = 0;
    let mut r = -1;
    for i in 2usize..data.len() {
        let sum: i32 = &data[i] + &data[i - 1] + &data[i - 2];
        if sum > prev {
            r += 1;
        }
        prev = sum;
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_part1() {
        struct Case {
            given: Vec<i32>,
            want: i32,
        }

        let cases: Vec<Case> = vec![Case {
            given: vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263],
            want: 7,
        }];
        for c in cases {
            assert_eq!(part1(&c.given), c.want);
        }
    }

    #[test]
    fn day1_part2() {
        struct Case {
            given: Vec<i32>,
            want: i32,
        }

        let cases: Vec<Case> = vec![Case {
            given: vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263],
            want: 5,
        }];
        for c in cases {
            assert_eq!(part2(&c.given), c.want);
        }
    }
}
