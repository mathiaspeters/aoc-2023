pub fn day9() {
    println!("Result  9-1: {}", part1());
    println!("Result  9-2: {}", part2());
}

pub fn part1() -> isize {
    raw_input()
        .lines()
        .map(|line| {
            let values = line
                .split_ascii_whitespace()
                .map(|v| v.parse::<isize>().unwrap())
                .collect();
            find_next(values)
        })
        .sum()
}

pub fn part2() -> isize {
    raw_input()
        .lines()
        .map(|line| {
            let values = line
                .split_ascii_whitespace()
                .map(|v| v.parse::<isize>().unwrap())
                .collect();
            find_previous(values)
        })
        .sum()
}

fn find_next(mut values: Vec<isize>) -> isize {
    let mut lasts = vec![];
    let mut tmp = vec![];
    let mut curr = 0;
    loop {
        lasts.push(values[values.len() - 1]);
        values.windows(2).for_each(|w| tmp.push(w[1] - w[0]));
        if tmp.iter().all(|t| *t == 0) {
            break;
        } else {
            std::mem::swap(&mut values, &mut tmp);
            tmp.clear();
        }
    }
    loop {
        let last = lasts.pop();
        match last {
            Some(l) => curr = l + curr,
            None => return curr,
        }
    }
}

fn find_previous(mut values: Vec<isize>) -> isize {
    let mut firsts = vec![];
    let mut tmp = vec![];
    let mut curr = 0;
    loop {
        firsts.push(values[0]);
        values.windows(2).for_each(|w| tmp.push(w[1] - w[0]));
        if tmp.iter().all(|t| *t == 0) {
            break;
        } else {
            std::mem::swap(&mut values, &mut tmp);
            tmp.clear();
        }
    }
    loop {
        loop {
            let first = firsts.pop();
            match first {
                Some(f) => curr = f - curr,
                None => return curr,
            }
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
        assert_eq!(114, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(2, part2());
    }
}
