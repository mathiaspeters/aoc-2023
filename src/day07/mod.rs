pub fn day7() {
    println!("Result  7-1: {}", part1());
    println!("Result  7-2: {}", part2());
}

pub fn part1() -> usize {
    let mut hands = get_hands(false);
    hands.sort_by(
        |l, r| match l.get_type().partial_cmp(&r.get_type()).unwrap() {
            std::cmp::Ordering::Equal => l.cards.partial_cmp(&r.cards).unwrap(),
            ord @ _ => ord,
        },
    );
    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, Hand { bid, .. })| acc + (bid * (i + 1)))
}

pub fn part2() -> usize {
    let mut hands = get_hands(true);
    hands.sort_by(|l, r| {
        match l
            .get_type_with_jokers()
            .partial_cmp(&r.get_type_with_jokers())
            .unwrap()
        {
            std::cmp::Ordering::Equal => l.cards.partial_cmp(&r.cards).unwrap(),
            ord @ _ => ord,
        }
    });
    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, Hand { bid, .. })| acc + (bid * (i + 1)))
}

fn get_hands(joker: bool) -> Vec<Hand> {
    raw_input()
        .lines()
        .map(|s| {
            let mut parts = s.split_ascii_whitespace();
            let cards = parts
                .next()
                .unwrap()
                .chars()
                .map(|c: char| match c {
                    'T' => 10_u8,
                    'J' => match joker {
                        true => 0_u8,
                        false => 11_u8,
                    },
                    'Q' => 12_u8,
                    'K' => 13_u8,
                    'A' => 14_u8,
                    _ => c.to_digit(10).unwrap() as u8,
                })
                .collect();
            let bid = parts.next().unwrap().parse::<usize>().unwrap();
            Hand { cards, bid }
        })
        .collect::<Vec<_>>()
}

#[derive(Debug)]
struct Hand {
    cards: Vec<u8>,
    bid: usize,
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut counts = [0_u8; 15];
        for val in self.cards.iter() {
            counts[*val as usize] += 1;
        }
        counts.sort_unstable();
        counts.reverse();
        if counts[0] == 5 {
            HandType::FiveOfAKind
        } else if counts[0] == 4 {
            HandType::FourOfAKind
        } else if counts[0] == 3 {
            if counts[1] == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else if counts[0] == 2 {
            if counts[1] == 2 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        } else {
            HandType::HighCard
        }
    }

    fn get_type_with_jokers(&self) -> HandType {
        let mut jokers = 0;
        let mut counts = [0_u8; 15];
        for val in self.cards.iter() {
            if *val == 0 {
                jokers += 1;
            } else {
                counts[*val as usize] += 1;
            }
        }
        counts.sort_unstable();
        counts.reverse();
        if counts[0] + jokers == 5 {
            HandType::FiveOfAKind
        } else if counts[0] + jokers == 4 {
            HandType::FourOfAKind
        } else if counts[0] + jokers == 3 {
            if counts[1] == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else if counts[0] + jokers == 2 {
            if counts[1] == 2 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        } else {
            HandType::HighCard
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
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
        assert_eq!(6440, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(5905, part2());
    }
}
