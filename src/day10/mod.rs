pub fn day10() {
    println!("Result 10-1: {}", part1());
    println!("Result 10-2: {}", part2());
}

const UP: u8 = 0;
const DOWN: u8 = 1;
const LEFT: u8 = 2;
const RIGHT: u8 = 3;

const EMPTY: u8 = 0;
const PATH_V: u8 = 2;
const PATH_H: u8 = 3;
const PATH_BEND_UP: u8 = 4;
const PATH_BEND_DOWN: u8 = 5;

pub fn part1() -> usize {
    let (start, map) = get_map(false);
    let loop_length = get_longest_loop(start, &map).len();
    ((loop_length as f64) / 2.0).ceil() as usize
}

pub fn part2() -> usize {
    let (start, map) = get_map(true);
    let longest_loop = get_longest_loop(start, &map);
    let mut simple_map = vec![vec![0; map[0].len()]; map.len()];
    longest_loop.iter().for_each(|(row, col)| {
        simple_map[*row][*col] = if map[*row][*col].is_horizontal() {
            PATH_H
        } else if map[*row][*col].is_vertical() {
            PATH_V
        } else {
            map[*row][*col].bend_dir()
        };
    });
    simple_map[start.0][start.1] = if longest_loop[1].0 == longest_loop.last().unwrap().0 {
        PATH_H
    } else if longest_loop[1].1 == longest_loop.last().unwrap().1 {
        PATH_V
    } else {
        map[start.0][start.1].bend_dir()
    };
    let mut inside = 0;
    (0..simple_map.len()).for_each(|row| {
        let mut wall = 0;
        let mut last_bend = None;
        (0..simple_map[0].len()).for_each(|col| match simple_map[row][col] {
            EMPTY => {
                if wall % 2 != 0 {
                    inside += 1;
                }
            }
            PATH_V => wall += 1,
            bend @ PATH_BEND_UP | bend @ PATH_BEND_DOWN => match last_bend {
                Some(lb) => {
                    if bend == lb {
                        wall += 2;
                    } else {
                        wall += 1;
                    }
                    last_bend = None;
                }
                None => last_bend = Some(bend),
            },
            _ => {}
        })
    });
    inside
}

fn get_longest_loop(start: (usize, usize), map: &[Vec<Tile>]) -> Vec<(usize, usize)> {
    let (srow, scol) = start;
    let max_row = map.len() - 1;
    let max_col = map[0].len() - 1;

    let mut longest_loop = vec![];
    if srow > 0 && map[srow - 1][scol].has(DOWN) {
        if let Some(path) = find_loop(
            (srow - 1, scol),
            DOWN,
            &map,
            vec![(srow, scol), (srow - 1, scol)],
        ) {
            if path.len() > longest_loop.len() {
                longest_loop = path;
            }
        }
    }
    if srow < max_row && map[srow + 1][scol].has(UP) {
        if let Some(path) = find_loop(
            (srow + 1, scol),
            UP,
            &map,
            vec![(srow, scol), (srow + 1, scol)],
        ) {
            if path.len() > longest_loop.len() {
                longest_loop = path;
            }
        }
    }
    if scol > 0 && map[srow][scol - 1].has(RIGHT) {
        if let Some(path) = find_loop(
            (srow, scol - 1),
            RIGHT,
            &map,
            vec![(srow, scol), (srow, scol - 1)],
        ) {
            if path.len() > longest_loop.len() {
                longest_loop = path;
            }
        }
    }
    if scol < max_col && map[srow][scol + 1].has(LEFT) {
        if let Some(path) = find_loop(
            (srow, scol + 1),
            LEFT,
            &map,
            vec![(srow, scol), (srow, scol + 1)],
        ) {
            if path.len() > longest_loop.len() {
                longest_loop = path;
            }
        }
    }
    longest_loop
}

fn find_loop(
    curr: (usize, usize),
    mut dir: u8,
    map: &[Vec<Tile>],
    mut path: Vec<(usize, usize)>,
) -> Option<Vec<(usize, usize)>> {
    let (mut row, mut col) = curr;
    loop {
        match map[row][col].get_exit(dir) {
            UP if row == 0 => return None,
            DOWN if row == map.len() - 1 => return None,
            LEFT if col == 0 => return None,
            RIGHT if col == map[0].len() - 1 => return None,
            UP => {
                row -= 1;
                dir = DOWN;
            }
            DOWN => {
                row += 1;
                dir = UP;
            }
            LEFT => {
                col -= 1;
                dir = RIGHT;
            }
            RIGHT => {
                col += 1;
                dir = LEFT;
            }
            _ => unreachable!(),
        }
        if map[row][col].is_end() {
            return Some(path);
        }
        if !map[row][col].has(dir) {
            return None;
        }
        path.push((row, col));
    }
}

fn get_map(two: bool) -> ((usize, usize), Vec<Vec<Tile>>) {
    let mut start = (0, 0);
    let map = if two { raw_input2() } else { raw_input1() }
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '|' => Tile(vec![UP, DOWN]),
                    '-' => Tile(vec![LEFT, RIGHT]),
                    'L' => Tile(vec![UP, RIGHT]),
                    'J' => Tile(vec![UP, LEFT]),
                    '7' => Tile(vec![DOWN, LEFT]),
                    'F' => Tile(vec![DOWN, RIGHT]),
                    '.' => Tile(vec![]),
                    'S' => {
                        start = (row, col);
                        Tile(vec![UP, DOWN, LEFT, RIGHT])
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (start, map)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Tile(Vec<u8>);

impl Tile {
    fn is_end(&self) -> bool {
        self.0.len() == 4
    }

    fn is_horizontal(&self) -> bool {
        self.0
            .iter()
            .all(|d| *d != UP && *d != DOWN && (*d == LEFT || *d == RIGHT))
    }

    fn is_vertical(&self) -> bool {
        self.0
            .iter()
            .all(|d| *d != LEFT && *d != RIGHT && (*d == UP || *d == DOWN))
    }

    fn bend_dir(&self) -> u8 {
        if self.0.contains(&UP) && !self.0.contains(&DOWN) {
            PATH_BEND_UP
        } else {
            PATH_BEND_DOWN
        }
    }

    fn has(&self, d: u8) -> bool {
        self.0.contains(&d)
    }

    fn get_exit(&self, entrance: u8) -> u8 {
        assert!(!self.is_end());
        for d in &self.0 {
            if *d != entrance {
                return *d;
            }
        }
        unreachable!()
    }
}

#[cfg(not(test))]
fn raw_input1() -> &'static str {
    include_str!("input")
}

#[cfg(not(test))]
fn raw_input2() -> &'static str {
    include_str!("input")
}

#[cfg(test)]
fn raw_input1() -> &'static str {
    include_str!("testinput1")
}

#[cfg(test)]
fn raw_input2() -> &'static str {
    include_str!("testinput2")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(8, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(10, part2());
    }
}
