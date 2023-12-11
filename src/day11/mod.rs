pub fn day11() {
    println!("Result 11-1: {}", part1());
    println!("Result 11-2: {}", part2());
}

pub fn part1() -> usize {
    get_cumulative_distance(1)
}

pub fn part2() -> usize {
    get_cumulative_distance(999_999)
}

fn get_cumulative_distance(empty_extra: usize) -> usize {
    let input = raw_input()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '#' { true } else { false })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut galaxies = vec![];
    let mut empty_rows = vec![];
    let mut empty_cols = vec![];
    (0..input.len()).for_each(|row| {
        let gs = (0..input[0].len())
            .filter_map(|col| {
                if input[row][col] {
                    Some((row, col))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if gs.is_empty() {
            empty_rows.push(row);
        } else {
            galaxies.extend_from_slice(&gs);
        }
    });
    (0..input[0].len()).for_each(|col| {
        let gs = (0..input.len()).fold(0, |acc, row| if input[row][col] { acc + 1 } else { acc });
        if gs == 0 {
            empty_cols.push(col);
        }
    });
    (0..galaxies.len())
        .flat_map(|i| (i + 1..galaxies.len()).map(move |j| (i, j)))
        .map(|(i, j)| {
            let (rmax, rmin) = max_min(galaxies[i].0, galaxies[j].0);
            let (cmax, cmin) = max_min(galaxies[i].1, galaxies[j].1);
            let mut row_dist = rmax - rmin;
            let mut col_dist = cmax - cmin;
            for e in &empty_rows {
                if *e > rmin && *e < rmax {
                    row_dist += empty_extra
                }
            }
            for e in &empty_cols {
                if *e > cmin && *e < cmax {
                    col_dist += empty_extra
                }
            }
            row_dist + col_dist
        })
        .sum()
}

fn max_min(left: usize, right: usize) -> (usize, usize) {
    (left.max(right), left.min(right))
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
        assert_eq!(374, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(82000210, part2());
    }
}
