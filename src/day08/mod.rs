use std::collections::HashMap;

pub fn day8() {
    println!("Result  8-1: {}", part1());
    println!("Result  8-2: {}", part2());
}

pub fn part1() -> usize {
    let (pos, end, steps, dirs) = process_input(false);
    let mut pos = pos[0];
    let end = end[0];
    let mut step_count = 0;
    loop {
        for step in steps.iter() {
            pos = dirs[pos][*step];
            step_count += 1;
            if pos == end {
                return step_count;
            }
        }
    }
}

pub fn part2() -> usize {
    let (mut pos, ends, steps, dirs) = process_input(true);
    let (paths, goals) = process_v2(&ends, &steps, &dirs);
    let ends = process_v3(ends.len(), &goals);
    let mut step_count = 0;
    loop {
        pos.iter_mut().for_each(|p| *p = paths[*p]);
        step_count += steps.len();
        for (extra_steps, es) in ends.iter() {
            if pos.iter().all(|p| es.contains(p)) {
                return step_count + extra_steps;
            }
        }
    }
}

fn process_v3(end_count: usize, paths: &[Vec<usize>]) -> Vec<(usize, Vec<usize>)> {
    let mut groups = HashMap::new();
    paths.iter().enumerate().for_each(|(i, goals)| {
        goals
            .iter()
            .for_each(|g| groups.entry(*g).or_insert(vec![]).push(i));
    });
    groups
        .into_iter()
        .filter(|(_, v)| v.len() >= end_count)
        .collect::<Vec<_>>()
}

fn process_v2(
    ends: &[usize],
    steps: &[usize],
    dirs: &[[usize; 2]],
) -> (Vec<usize>, Vec<Vec<usize>>) {
    (0..dirs.len())
        .map(|i| {
            let mut goals = vec![];
            let mut pos = i;
            let mut step_count = 0;
            steps.iter().enumerate().for_each(|(j, s)| {
                pos = dirs[pos][*s];
                step_count += 1;
                if ends.contains(&pos) {
                    goals.push(j + 1)
                }
            });
            (pos, goals)
        })
        .unzip()
}

fn process_input(two: bool) -> (Vec<usize>, Vec<usize>, Vec<usize>, Vec<[usize; 2]>) {
    let mut starts = vec![];
    let mut ends = vec![];
    let mut next_id = 0_usize;
    let mut ids = HashMap::new();
    let mut lines = if two {
        raw_input2().lines()
    } else {
        raw_input1().lines()
    };
    let steps = lines
        .next()
        .unwrap()
        .chars()
        .map(|c: char| match c {
            'L' => 0_usize,
            'R' => 1_usize,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    let lines = lines
        .skip(1)
        .map(|line| {
            let justc = line.replace(|c: char| !c.is_alphanumeric(), "");
            [
                justc[0..3].to_owned(),
                justc[3..6].to_owned(),
                justc[6..9].to_owned(),
            ]
        })
        .collect::<Vec<_>>();
    lines.iter().enumerate().for_each(|(i, chunks)| {
        if two {
            if chunks[0].ends_with('A') {
                starts.push(i);
            } else if chunks[0].ends_with('Z') {
                ends.push(i);
            }
        } else {
            if chunks[0] == "AAA" {
                starts.push(i);
            } else if chunks[0] == "ZZZ" {
                ends.push(i);
            }
        }
        ids.insert(&chunks[0], next_id);
        next_id += 1;
    });
    let dirs = lines
        .iter()
        .map(|chunks| [*ids.get(&chunks[1]).unwrap(), *ids.get(&chunks[2]).unwrap()])
        .collect::<Vec<_>>();
    (starts, ends, steps, dirs)
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
        assert_eq!(2, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(6, part2());
    }
}
