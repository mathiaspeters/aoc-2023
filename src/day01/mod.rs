pub fn day1() {
    println!("Result  1-1: {}", part1());
    println!("Result  1-2: {}", part2());
}

pub fn part1() -> usize {
    raw_input1()
    .lines()
    .map(|line| line.chars().filter_map(|c| c.to_digit(10).map(|r| r as usize)).collect::<Vec<_>>())
    .map(|is| {
        (*is.first().unwrap() * 10) + *is.last().unwrap()
    })
    .sum()
}

pub fn part2() -> usize {
    raw_input2()
        .lines()
        .map(|line| find_nums(line))
        .map(|is| {
            (*is.first().unwrap() * 10) + *is.last().unwrap()
        })
        .sum()
        
}

fn find_nums(cs: &str) -> Vec<usize> {
    let cs = cs.chars().collect::<Vec<_>>();
    let check = |i: usize, c: char| -> bool {
        matches!(cs.get(i), Some(n) if *n == c)
    };
    let mut output = vec![];
    for (i, c) in cs.iter().enumerate() {
        match c {
            '0' => output.push(0),
            '1' => output.push(1),
            '2' => output.push(2),
            '3' => output.push(3),
            '4' => output.push(4),
            '5' => output.push(5),
            '6' => output.push(6),
            '7' => output.push(7),
            '8' => output.push(8),
            '9' => output.push(9),
            'o' => {
                if check(i+1, 'n') && check(i+2, 'e') {
                    output.push(1)
                }
            }
            't' => {
                if check(i+1, 'w') && check(i+2, 'o') {
                    output.push(2)
                } else if check(i+1, 'h') && check(i+2, 'r') && check(i+3, 'e') && check(i+4, 'e') {
                    output.push(3)
                }
            }
            'f' => {
                if check(i+1, 'o') && check(i+2, 'u') && check(i+3, 'r') {
                    output.push(4)
                } else if check(i+1, 'i') && check(i+2, 'v') && check(i+3, 'e') {
                    output.push(5)
                }
            }
            's' => {
                if check(i+1, 'i') && check(i+2, 'x') {
                    output.push(6)
                } else if check(i+1, 'e') && check(i+2, 'v') && check(i+3, 'e') && check(i+4, 'n') {
                    output.push(7)
                }
            }
            'e' => {
                if check(i+1, 'i') && check(i+2, 'g') && check(i+3, 'h') && check(i+4, 't') {
                    output.push(8)
                }
            }
            'n' => {
                if check(i+1, 'i') && check(i+2, 'n') && check(i+3, 'e') {
                    output.push(9)
                }
            }
            'z' => {
                if check(i+1, 'e') && check(i+2, 'r') && check(i+3, 'o') {
                    output.push(0)
                }
            }
            _ => {}
        }
    }
    output
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
        assert_eq!(142, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(281, part2());
    }
}
