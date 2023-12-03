use std::collections::HashMap;

pub fn day3() {
    println!("Result  3-1: {}", part1());
    println!("Result  3-2: {}", part2());
}

pub fn part1() -> usize {
    let mut symbols: Vec<Vec<bool>> = vec![];
    let mut digits: Vec<Vec<bool>> = vec![];
    let mut numbers: Vec<Number> = vec![];
    raw_input().lines().enumerate().for_each(|(i, line)| {
        let mut line_symbols = vec![];
        let mut line_digits = vec![];
        let mut curr: Option<Number> = None;
        line.chars().enumerate().for_each(|(j, c)| {
            if c.is_numeric() {
                line_symbols.push(false);
                line_digits.push(true);
                match &mut curr {
                    Some(ref mut n) => {
                        n.value = n.value * 10 + c.to_digit(10).unwrap() as usize;
                        n.len += 1;
                    }
                    None => {
                        curr = Some(Number {
                            value: c.to_digit(10).unwrap() as usize,
                            start: (i, j),
                            len: 1,
                        })
                    }
                }
            } else {
                if let Some(n) = curr {
                    numbers.push(n);
                    curr = None;
                }
                if c != '.' {
                    line_symbols.push(true);
                    line_digits.push(false);
                } else {
                    line_symbols.push(false);
                    line_digits.push(false);
                }
            }
        });
        if let Some(n) = curr {
            numbers.push(n);
        }
        symbols.push(line_symbols);
        digits.push(line_digits);
    });
    numbers
        .iter()
        .map(|n| {
            (
                n.value,
                n.get_bounds(symbols[0].len() - 1, symbols.len() - 1),
            )
        })
        .filter_map(|(value, bounds)| {
            let mut is_adjacent = false;
            'outer: for i in bounds.top..bounds.bottom + 1 {
                for j in bounds.left..bounds.right + 1 {
                    if symbols[i][j] {
                        is_adjacent = true;
                        break 'outer;
                    } else if i != bounds.center && digits[i][j] {
                        is_adjacent = true;
                        break 'outer;
                    }
                }
            }
            if is_adjacent {
                Some(value)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2() -> usize {
    let mut symbols = HashMap::<(usize, usize), Vec<usize>>::new();
    let mut numbers: Vec<Number> = vec![];
    let mut max_w = 0;
    let mut max_h = 0;
    raw_input().lines().enumerate().for_each(|(i, line)| {
        let mut curr: Option<Number> = None;
        line.chars().enumerate().for_each(|(j, c)| {
            if c.is_numeric() {
                match &mut curr {
                    Some(ref mut n) => {
                        n.value = n.value * 10 + c.to_digit(10).unwrap() as usize;
                        n.len += 1;
                    }
                    None => {
                        curr = Some(Number {
                            value: c.to_digit(10).unwrap() as usize,
                            start: (i, j),
                            len: 1,
                        })
                    }
                }
            } else {
                if let Some(n) = curr {
                    numbers.push(n);
                    curr = None;
                }
                if c == '*' {
                    symbols.insert((i, j), vec![]);
                }
            }
            max_w = max_w.max(j);
        });
        if let Some(n) = curr {
            numbers.push(n);
        }
        max_h = max_h.max(i);
    });
    numbers
        .iter()
        .map(|n| (n.value, n.get_bounds(max_w, max_h)))
        .for_each(|(value, bounds)| {
            for i in bounds.top..bounds.bottom + 1 {
                for j in bounds.left..bounds.right + 1 {
                    if let Some(v) = symbols.get_mut(&(i, j)) {
                        v.push(value);
                    }
                }
            }
        });
    symbols
        .values()
        .filter_map(|values| {
            if values.len() == 2 {
                Some(values[0] * values[1])
            } else {
                None
            }
        })
        .sum()
}

#[derive(Copy, Clone, Debug)]
struct Number {
    value: usize,
    start: (usize, usize),
    len: usize,
}

#[derive(Copy, Clone, Debug)]
struct Bounds {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
    center: usize,
}

impl Number {
    fn get_bounds(&self, max_w: usize, max_h: usize) -> Bounds {
        Bounds {
            top: self.start.0.saturating_sub(1),
            bottom: (self.start.0 + 1).min(max_h),
            left: self.start.1.saturating_sub(1),
            right: (self.start.1 + self.len).min(max_w),
            center: self.start.0,
        }
    }
}

#[cfg(not(test))]
fn raw_input() -> &'static str {
    include_str!("input")
}

#[cfg(test)]
fn raw_input() -> &'static str {
    include_str!("testinput")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(4361, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(467835, part2());
    }
}
