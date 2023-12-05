pub fn day5() {
    println!("Result  5-1: {}", part1());
    println!("Result  5-2: {}", part2());
}

pub fn part1() -> usize {
    let (seeds, mappings) = process_input();
    seeds
        .iter()
        .map(|num| {
            let mut value = *num;
            for m in mappings.iter() {
                for (dst, src, len) in m {
                    if value >= *src && value <= *src + *len - 1 {
                        value = *dst + (value - *src);
                        break;
                    }
                }
            }
            value
        })
        .min()
        .unwrap()
}

pub fn part2() -> usize {
    let (seeds, mappings) = process_input();
    let seeds = seeds
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
        .collect::<Vec<_>>();
    let mut min_value = usize::MAX;
    seeds.into_iter().for_each(|(start, end)| {
        (start..end).for_each(|seed| {
            let mut value = seed;
            for m in mappings.iter() {
                for (dst, src, len) in m {
                    if value >= *src && value <= *src + *len - 1 {
                        value = *dst + (value - *src);
                        break;
                    }
                }
            }
            min_value = min_value.min(value);
        });
    });
    min_value
}

fn process_input() -> (Vec<usize>, Vec<Vec<(usize, usize, usize)>>) {
    let mut seeds = vec![];
    let mut mappings = vec![];
    let mut current_mapping = vec![];

    for line in raw_input().lines() {
        if let Some(line) = line.strip_prefix("seeds:") {
            line.split_ascii_whitespace().for_each(|s| {
                if let Ok(num) = s.parse::<usize>() {
                    seeds.push(num);
                }
            })
        } else if line.starts_with(|c: char| c.is_ascii_alphabetic()) && !current_mapping.is_empty()
        {
            current_mapping.sort_unstable_by_key(|(s, _, _)| *s);
            mappings.push(current_mapping);
            current_mapping = vec![];
        } else {
            let nums = line
                .split_ascii_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>();
            if nums.len() == 3 {
                current_mapping.push((nums[0], nums[1], nums[2]));
            }
        }
    }
    current_mapping.sort_unstable_by_key(|(s, _, _)| *s);
    mappings.push(current_mapping);

    (seeds, mappings)
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
        assert_eq!(35, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(46, part2());
    }
}
