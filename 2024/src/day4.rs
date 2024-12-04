use std::collections::VecDeque;

pub fn part1(data: &Vec<String>) -> i32 {
    let max_x = data[0].len();
    let max_y = data.len();

    let grid = parse_grid(data);
    let mut count = 0;
    for y in 0..max_y {
        for x in 0..max_x {
            if grid[y][x] == 'X' {
                if search(&grid, x, y, 1, 0) {
                    count += 1; // right
                }
                if search(&grid, x, y, 0, 1) {
                    count += 1 //down
                }
                if search(&grid, x, y, 1, 1) {
                    count += 1 // right down
                }
                if search(&grid, x, y, -1, 1) {
                    count += 1 // left down
                }
                if search(&grid, x, y, 1, -1) {
                    count += 1 // right up
                }
                if search(&grid, x, y, -1, -1) {
                    count += 1 // left up
                }
                if search(&grid, x, y, -1, 0) {
                    count += 1 // left
                }
                if search(&grid, x, y, 0, -1) {
                    count += 1 // up
                }
            }
        }
    }
    count
}

fn search(grid: &Vec<Vec<char>>, x: usize, y: usize, dx: i32, dy: i32) -> bool {
    let mut x = x as i32 + dx;
    let mut y = y as i32 + dy;
    let mut to_find = VecDeque::from(['M', 'A', 'S']);
    while x >= 0
        && x < grid[0].len() as i32
        && y >= 0
        && y < grid.len() as i32
        && !to_find.is_empty()
    {
        if grid[y as usize][x as usize] != to_find.pop_front().unwrap() {
            return false;
        }
        x += dx;
        y += dy;
    }
    to_find.is_empty()
}

fn parse_grid(data: &Vec<String>) -> Vec<Vec<char>> {
    data.iter().map(|line| line.chars().collect()).collect()
}

fn xsearch(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x == 0 || y == 0 || x >= grid[0].len() - 1 || y >= grid.len() - 1 {
        return false;
    }

    // lu . ru
    //    X
    // ld . rd
    let lu = grid[y - 1][x - 1];
    let ru = grid[y - 1][x + 1];
    let ld = grid[y + 1][x - 1];
    let rd = grid[y + 1][x + 1];

    let c1 = (lu == 'M' && rd == 'S') || (lu == 'S' && rd == 'M');
    let c2 = (ld == 'M' && ru == 'S') || (ld == 'S' && ru == 'M');
    c1 && c2
}

pub fn part2(data: &Vec<String>) -> i32 {
    let max_x = data[0].len();
    let max_y = data.len();

    let grid = parse_grid(data);
    let mut count = 0;
    for y in 1..max_y - 1 {
        for x in 1..max_x - 1 {
            if grid[y][x] == 'A' && xsearch(&grid, x, y) {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_part1() {
        struct Case {
            given: Vec<String>,
            want: i32,
        }

        let cases: Vec<Case> = vec![
            Case {
                given: vec![
                    String::from("..X..."),
                    String::from(".SAMX."),
                    String::from(".A..A."),
                    String::from("XMAS.S"),
                    String::from(".X...."),
                ],
                want: 4,
            },
            Case {
                given: vec![
                    String::from("MMMSXXMASM"),
                    String::from("MSAMXMSMSA"),
                    String::from("AMXSXMAAMM"),
                    String::from("MSAMASMSMX"),
                    String::from("XMASAMXAMM"),
                    String::from("XXAMMXXAMA"),
                    String::from("SMSMSASXSS"),
                    String::from("SAXAMASAAA"),
                    String::from("MAMMMXMMMM"),
                    String::from("MXMXAXMASX"),
                ],
                want: 18,
            },
        ];
        for c in cases {
            assert_eq!(part1(&c.given), c.want);
        }
    }

    #[test]
    fn day4_part2() {
        struct Case {
            given: Vec<String>,
            want: i32,
        }

        let cases: Vec<Case> = vec![Case {
            given: vec![
                String::from("MMMSXXMASM"),
                String::from("MSAMXMSMSA"),
                String::from("AMXSXMAAMM"),
                String::from("MSAMASMSMX"),
                String::from("XMASAMXAMM"),
                String::from("XXAMMXXAMA"),
                String::from("SMSMSASXSS"),
                String::from("SAXAMASAAA"),
                String::from("MAMMMXMMMM"),
                String::from("MXMXAXMASX"),
            ],
            want: 9,
        }];
        for c in cases {
            assert_eq!(part2(&c.given), c.want);
        }
    }
}
