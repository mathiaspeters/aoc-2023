pub fn day2() {
    println!("Result  2-1: {}", part1());
    println!("Result  2-2: {}", part2());
}

pub fn part1() -> usize {
    raw_input()
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let gid = parts
                .next()
                .unwrap()
                .strip_prefix("Game ")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let mut possible = true;
            parts.next().unwrap().split("; ").for_each(|round| {
                round.split(", ").for_each(|draw| {
                    let mut parts = draw.split_ascii_whitespace();
                    let amount = parts.next().unwrap().parse::<usize>().unwrap();
                    let color = parts.next().unwrap();
                    match color {
                        "red" if amount > 12 => possible = false,
                        "green" if amount > 13 => possible = false,
                        "blue" if amount > 14 => possible = false,
                        _ => {}
                    }
                })
            });
            (gid, possible)
        })
        .filter_map(|(gid, possible)| if possible { Some(gid) } else { None })
        .sum()
}

pub fn part2() -> usize {
    raw_input()
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            parts.next().unwrap();
            let mut max_r = 0;
            let mut max_g = 0;
            let mut max_b = 0;
            parts.next().unwrap().split("; ").for_each(|round| {
                round.split(", ").for_each(|draw| {
                    let mut parts = draw.split_ascii_whitespace();
                    let amount = parts.next().unwrap().parse::<usize>().unwrap();
                    let color = parts.next().unwrap();
                    match color {
                        "red" => max_r = max_r.max(amount),
                        "green" => max_g = max_g.max(amount),
                        "blue" => max_b = max_b.max(amount),
                        _ => {}
                    }
                })
            });
            max_r * max_g * max_b
        })
        .sum()
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
        assert_eq!(8, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(2286, part2());
    }
}
