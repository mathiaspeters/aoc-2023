pub fn day6() {
    println!("Result  6-1: {}", part1());
    println!("Result  6-2: {}", part2());
}

pub fn part1() -> usize {
    let lines = raw_input()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|s| s.parse::<f64>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    lines[0]
        .iter()
        .zip(lines[1].iter())
        .map(|(time, distance)| calculate(*time, *distance))
        .fold(1, |acc, options| acc * options)
}

pub fn part2() -> usize {
    let lines = raw_input()
        .lines()
        .map(|line| {
            line.replace(
                |c: char| c.is_whitespace() || c.is_alphabetic() || c == ':',
                "",
            )
            .parse::<f64>()
            .unwrap()
        })
        .collect::<Vec<_>>();
    calculate(lines[0], lines[1])
}

fn calculate(time: f64, distance: f64) -> usize {
    let temp = (time * time - 4.0 * (distance + 1.0)).sqrt();
    let low = ((time - temp) / 2.0).ceil() as usize;
    let high = ((time + temp) / 2.0).floor() as usize;
    high - low + 1
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
        assert_eq!(288, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(71503, part2());
    }
}
