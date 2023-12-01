pub fn day1() {
    println!("Result  1-1: {}", part1());
    println!("Result  1-2: {}", part2());
}

pub fn part1() -> usize {
    raw_input1()
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|r| r as usize))
                .collect::<Vec<_>>()
        })
        .map(|is| (*is.first().unwrap() * 10) + *is.last().unwrap())
        .sum()
}

pub fn part2() -> usize {
    raw_input2()
        .lines()
        .map(|line| find_nums(line))
        .map(|is| (*is.first().unwrap() * 10) + *is.last().unwrap())
        .sum()
}

fn find_nums(cs: &str) -> Vec<usize> {
    let cs = cs.chars().collect::<Vec<_>>();
    let check_all = |start: usize, letters: &str| -> bool {
        letters
            .chars()
            .enumerate()
            .all(|(i, c)| matches!(cs.get(start + i), Some(n) if *n == c))
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
            _ => {
                if check_all(i, "zero") {
                    output.push(0);
                } else if check_all(i, "one") {
                    output.push(1);
                } else if check_all(i, "two") {
                    output.push(2);
                } else if check_all(i, "three") {
                    output.push(3);
                } else if check_all(i, "four") {
                    output.push(4);
                } else if check_all(i, "five") {
                    output.push(5);
                } else if check_all(i, "six") {
                    output.push(6);
                } else if check_all(i, "seven") {
                    output.push(7);
                } else if check_all(i, "eight") {
                    output.push(8);
                } else if check_all(i, "nine") {
                    output.push(9);
                }
            }
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
