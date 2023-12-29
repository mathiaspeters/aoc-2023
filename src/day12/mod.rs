pub fn day12() {
    println!("Result 12-1: {}", part1());
    println!("Result 12-2: {}", part2());
}

const BROKEN: u8 = 0;
const WORKING: u8 = 1;
const UNKNOWN: u8 = 2;

pub fn part1() -> usize {
    get_answer(1)
}

pub fn part2() -> usize {
    get_answer(5)
}

fn get_answer(multiplier: usize) -> usize {
    raw_input()
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let springs = parts
                .next()
                .unwrap()
                .split('.')
                .filter(|part| !part.is_empty())
                .flat_map(|group| {
                    let mut group = group
                        .chars()
                        .map(|c| match c {
                            '?' => UNKNOWN,
                            _ => BROKEN,
                        })
                        .collect::<Vec<_>>();
                    group.push(WORKING);
                    group
                })
                .collect::<Vec<_>>();
            let sizes = parts
                .next()
                .unwrap()
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (springs.repeat(multiplier), sizes.repeat(multiplier))
        })
        .map(|(springs, sizes)| {
            println!("New row");
            let res = count_options(&springs, &sizes);
            println!("Options: {}", res);
            res
        })
        .sum()
}

fn count_options(springs: &[u8], sizes: &[usize]) -> usize {
    let spots = sizes
        .iter()
        .enumerate()
        .map(|(i, size)| {
            let sskip: usize = sizes
                .get(0..i)
                .map(|s| s.iter().map(|s| *s + 1).sum::<usize>())
                .unwrap_or(0);
            let eskip = sizes
                .get(i + 1..)
                .map(|s| s.iter().map(|s| *s + 1).sum::<usize>())
                .unwrap_or(0);
            let mut fits = vec![];
            for j in sskip..springs.len() - eskip {
                if let Some(window) = springs.get(j..j + size) {
                    let prev = if j > 0 {
                        springs[j - 1] == UNKNOWN || springs[j - 1] == WORKING
                    } else {
                        true
                    };
                    let next = match springs.get(j + size) {
                        Some(x) => *x == UNKNOWN || *x == WORKING,
                        None => true,
                    };
                    if prev && window.iter().all(|w| *w == BROKEN) && next {
                        fits = vec![j];
                        break;
                    }
                    if prev && window.iter().all(|w| *w == BROKEN || *w == UNKNOWN) && next {
                        fits.push(j);
                    }
                }
            }
            fits
        })
        .collect::<Vec<_>>();

    dbg!(spots.iter().fold(1_usize, |acc, spot| acc * spot.len()));
    0
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
        assert_eq!(21, part1()); // 7110
    }

    #[test]
    fn test_part2() {
        assert_eq!(525152, part2());
    }
}
