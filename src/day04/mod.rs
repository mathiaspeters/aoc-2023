use std::ops::Shl;

pub fn day4() {
    println!("Result  4-1: {}", part1());
    println!("Result  4-2: {}", part2());
}

pub fn part1() -> usize {
    raw_input()
        .lines()
        .map(|line| process(line))
        .map(|(wn, pn)| {
            let num_wins = count_wins(&wn, &pn);
            if num_wins == 0 {
                0_usize
            } else {
                1_usize.shl(num_wins.saturating_sub(1)) as usize
            }
        })
        .sum()
}

pub fn part2() -> usize {
    let numbers = raw_input()
        .lines()
        .map(|line| process(line))
        .collect::<Vec<_>>();
    let mut tickets = vec![1; numbers.len()];
    let max_t = tickets.len() - 1;

    numbers.into_iter().enumerate().for_each(|(i, (wn, pn))| {
        let num_wins = count_wins(&wn, &pn);
        for j in i + 1..=i + num_wins {
            if j > max_t {
                break;
            }
            tickets[j] += tickets[i];
        }
    });

    tickets.iter().sum()
}

fn process(line: &str) -> (Vec<u8>, Vec<u8>) {
    let mut wn = vec![];
    let mut pn = vec![];
    let mut is_w = true;
    line.split_ascii_whitespace()
        .skip_while(|s| !s.contains(':'))
        .for_each(|s| {
            if s.contains('|') {
                is_w = false;
            } else {
                match s.parse::<u8>() {
                    Ok(n) if is_w => wn.push(n),
                    Ok(n) => pn.push(n),
                    _ => {}
                }
            }
        });
    pn.sort_unstable();
    (wn, pn)
}

fn count_wins(winning_numbers: &[u8], played_numbers: &[u8]) -> usize {
    winning_numbers
        .iter()
        .fold(0, |acc, n| match played_numbers.binary_search(n) {
            Ok(_) => acc + 1,
            _ => acc,
        })
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
        assert_eq!(13, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(30, part2());
    }
}
