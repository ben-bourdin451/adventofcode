use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    io::{empty, Write},
};

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

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn step(&self, c: &Coordinate) -> (Coordinate) {
        match self {
            Direction::North => Coordinate { x: c.x, y: c.y - 1 },
            Direction::East => Coordinate { x: c.x + 1, y: c.y },
            Direction::South => Coordinate { x: c.x, y: c.y + 1 },
            Direction::West => Coordinate { x: c.x - 1, y: c.y },
        }
    }

    fn print(&self) -> char {
        match self {
            Direction::North => '^',
            Direction::East => '>',
            Direction::South => 'v',
            Direction::West => '<',
        }
    }
}

pub fn part1(data: &Vec<String>) -> i32 {
    let (grid, start) = parse_grid(data);

    let (_, mut visited) = walk(&grid, &start, Direction::North);
    visited.len() as i32
}

fn parse_grid(map: &Vec<String>) -> (Vec<Vec<char>>, Coordinate) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut start = Coordinate { x: 0, y: 0 };
    let mut y = 0;
    for line in map {
        let mut x = 0;
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            if c == '^' {
                start = Coordinate { x, y };
                row.push('.');
            } else {
                row.push(c);
            }
            x += 1;
        }
        grid.push(row);
        y += 1;
    }
    (grid, start)
}

fn walk(
    grid: &Vec<Vec<char>>,
    start: &Coordinate,
    d: Direction,
) -> (bool, HashMap<Coordinate, HashSet<Direction>>) {
    let mut d = d.clone();
    let mut c = start.clone();
    let mut next = d.step(&c);
    let mut visited: HashMap<Coordinate, HashSet<Direction>> = HashMap::new();
    visited.entry(c).or_insert(HashSet::new()).insert(d);
    while !(visited.contains_key(&next) && visited[&next].contains(&d)) {
        while grid[next.y as usize][next.x as usize] != '#' {
            c = next.clone();
            visited.entry(c).or_insert(HashSet::new()).insert(d);
            next = d.step(&c);

            // check if we have visited this coordinate in this direction before (infinite loop)
            if visited.contains_key(&next) && visited[&next].contains(&d) {
                return (true, visited);
            }

            // check if we are about to run out of bounds
            if !next.within_bounds(grid[0].len(), grid.len()) {
                return (false, visited);
            }
        }
        d = d.turn_right();
        next = d.step(&c);
    }
    (true, visited)
}

pub fn part2(data: &Vec<String>) -> i32 {
    let (mut grid, start) = parse_grid(data);
    let (_, mut visited) = walk(&grid, &start, Direction::North);
    let mut obstacles: HashMap<Coordinate, bool> = HashMap::new();
    for (c, hs) in visited {
        for d in hs {
            let obstacle = d.step(&c);
            if !obstacle.within_bounds(grid[0].len(), grid.len())
                || obstacles.contains_key(&obstacle)
                // || (obstacle.x == start.x && obstacle.y == start.y)
                || grid[obstacle.y as usize][obstacle.x as usize] == '#'
            {
                continue;
            }

            // place obstacle
            let prev = grid[obstacle.y as usize][obstacle.x as usize];
            grid[obstacle.y as usize][obstacle.x as usize] = '#';

            let (is_loop, _) = walk(&grid, &start, Direction::North);
            obstacles.insert(obstacle, is_loop);

            // remove obstacle
            grid[obstacle.y as usize][obstacle.x as usize] = prev;
        }
    }
    obstacles.values().filter(|o| **o).count() as i32
}

fn print_grid(
    grid: &Vec<Vec<char>>,
    visited: &HashMap<Coordinate, HashSet<Direction>>,
    pos: &Coordinate,
    d: Direction,
) {
    let mut y = 0;
    for row in grid {
        let mut x = 0;
        for c in row {
            if pos.x == x && pos.y == y {
                print!("{}", d.print());
            } else if visited.contains_key(&Coordinate { x, y }) {
                print!("X");
            } else {
                print!("{}", c);
            }
            x += 1;
        }
        println!();
        y += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_part1() {
        struct Case {
            given: Vec<String>,
            want: i32,
        }

        let cases: Vec<Case> = vec![Case {
            given: vec![
                String::from("....#....."),
                String::from(".........#"),
                String::from(".........."),
                String::from("..#......."),
                String::from(".......#.."),
                String::from(".........."),
                String::from(".#..^....."),
                String::from("........#."),
                String::from("#........."),
                String::from("......#..."),
            ],
            want: 41,
        }];
        for c in cases {
            assert_eq!(part1(&c.given), c.want);
        }
    }

    #[test]
    fn day6_walk() {
        struct Case {
            given: Vec<String>,
            want_loop: bool,
            want: i32,
        }

        let cases: Vec<Case> = vec![Case {
            given: vec![
                String::from("....#....."),
                String::from(".........#"),
                String::from(".........."),
                String::from("..#......."),
                String::from(".......#.."),
                String::from(".........."),
                String::from(".#..^....."),
                String::from("........#."),
                String::from("#........."),
                String::from("......##.."),
            ],
            want_loop: true,
            want: 40,
        }];
        for c in cases {
            let (grid, start) = parse_grid(&c.given);
            let (is_loop, visited) = walk(&grid, &start, Direction::North);
            assert_eq!(is_loop, c.want_loop);
            assert_eq!(visited.len(), c.want as usize);
        }
    }

    #[test]
    fn day6_part2() {
        struct Case {
            given: Vec<String>,
            want: i32,
        }

        let cases: Vec<Case> = vec![Case {
            given: vec![
                String::from("....#....."),
                String::from(".........#"),
                String::from(".........."),
                String::from("..#......."),
                String::from(".......#.."),
                String::from(".........."),
                String::from(".#..^....."),
                String::from("........#."),
                String::from("#........."),
                String::from("......#..."),
            ],
            want: 6,
        }];
        for c in cases {
            assert_eq!(part2(&c.given), c.want);
        }
    }
}
