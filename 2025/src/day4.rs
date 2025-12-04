use log::{Level, debug, log_enabled};

pub fn part1(data: &Vec<String>) -> i32 {
    let (_, count) = rm_rolls(&parse_grid(data));
    return count;
}

pub fn part2(data: &Vec<String>) -> i32 {
    let mut map = parse_grid(data);
    let mut sum = 0;
    loop {
        let (nmap, count) = rm_rolls(&map);
        if log_enabled!(Level::Debug) {
            print_grid(&nmap);
            debug!("count: {}, sum: {}", count, sum);
            debug!("--------------------------------\n");
        }
        map = nmap;
        sum += count;
        if count == 0 {
            break;
        }
    }
    if log_enabled!(Level::Debug) {
        print_grid(&map);
    }
    return sum;
}

fn parse_grid(data: &Vec<String>) -> Vec<Vec<char>> {
    data.iter().map(|line| line.chars().collect()).collect()
}

fn rm_rolls(map: &Vec<Vec<char>>) -> (Vec<Vec<char>>, i32) {
    let max_y = map.len();
    let max_x = map[0].len();
    let mut count = 0;
    let mut nmap = map.clone();
    for y in 0..max_y {
        for x in 0..max_x {
            if map[y][x] == '@'
                && count_neighbors(
                    &map,
                    Coordinate {
                        x: x as i32,
                        y: y as i32,
                    },
                    max_x,
                    max_y,
                ) < 4
            {
                nmap[y][x] = 'x';
                count += 1;
            } else {
                nmap[y][x] = map[y][x];
            }
        }
    }
    (nmap, count)
}

fn print_grid(map: &Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
}

fn count_neighbors(map: &Vec<Vec<char>>, c: Coordinate, max_x: usize, max_y: usize) -> i32 {
    let mut count = 0;
    for y in (c.y - 1)..=(c.y + 1) {
        for x in (c.x - 1)..=(c.x + 1) {
            let n = Coordinate {
                x: x as i32,
                y: y as i32,
            };
            if (x == c.x && y == c.y) || !n.within_bounds(max_x, max_y) {
                continue;
            }

            if map[y as usize][x as usize] == '@' {
                count += 1;
            }
        }
    }
    count
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    fn within_bounds(&self, max_x: usize, max_y: usize) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < max_x as i32 && self.y < max_y as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logging::init_logging;

    #[test]
    fn day4_count_neighbors() {
        init_logging();
        let map = vec![
            vec!['.', '.', '@', '@', '.'],
            vec!['@', '@', '@', '.', '@'],
            vec!['@', '@', '@', '@', '@'],
            vec!['.', '@', '@', '@', '.'],
        ];
        assert_eq!(count_neighbors(&map, Coordinate { x: 0, y: 0 }, 5, 4), 2);
        assert_eq!(count_neighbors(&map, Coordinate { x: 2, y: 2 }, 5, 4), 7);
    }

    #[test]
    fn day4_part1() {
        init_logging();
        let data = vec![
            String::from("..@@.@@@@."),
            String::from("@@@.@.@.@@"),
            String::from("@@@@@.@.@@"),
            String::from("@.@@@@..@."),
            String::from("@@.@@@@.@@"),
            String::from(".@@@@@@@.@"),
            String::from(".@.@.@.@@@"),
            String::from("@.@@@.@@@@"),
            String::from(".@@@@@@@@."),
            String::from("@.@.@@@.@."),
        ];
        assert_eq!(part1(&data), 13);
    }

    #[test]
    fn day4_part2() {
        init_logging();
        let data = vec![
            String::from("..@@.@@@@."),
            String::from("@@@.@.@.@@"),
            String::from("@@@@@.@.@@"),
            String::from("@.@@@@..@."),
            String::from("@@.@@@@.@@"),
            String::from(".@@@@@@@.@"),
            String::from(".@.@.@.@@@"),
            String::from("@.@@@.@@@@"),
            String::from(".@@@@@@@@."),
            String::from("@.@.@@@.@."),
        ];
        assert_eq!(part2(&data), 43);
    }
}
