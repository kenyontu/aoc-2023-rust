use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn get_points(&self) -> u32 {
        match self {
            Self::FiveOfAKind => 7,
            Self::FourOfAKind => 6,
            Self::FullHouse => 5,
            Self::ThreeOfAKind => 4,
            Self::TwoPair => 3,
            Self::OnePair => 2,
            Self::HighCard => 1,
        }
    }

    fn from_labels(labels: &Vec<Label>) -> Self {
        let mut jokers = 0;

        let map: HashMap<char, u32> = labels.iter().fold(HashMap::new(), |mut acc, c| {
            if c.val == 'J' {
                jokers += 1;
                return acc;
            }
            acc.entry(c.val)
                .and_modify(|count| *count = *count + 1)
                .or_insert(1);
            acc
        });

        let counts: Vec<u32> = map.into_values().collect();

        if counts.len() == 5 {
            return Self::HighCard;
        }

        if counts.len() == 0 || counts.len() == 1 {
            return Self::FiveOfAKind;
        }

        // four of a kind
        if counts.iter().any(|c| c + jokers == 4) {
            return Self::FourOfAKind;
        }

        // full house
        if counts.len() == 2 {
            if jokers == 1 && counts.iter().all(|c| *c == 2) {
                return Self::FullHouse;
            }
            if (counts[0] == 3 && counts[1] == 2) || (counts[0] == 2 && counts[1] == 3) {
                return Self::FullHouse;
            }
        }

        // three of a kind
        if counts.len() == 3 && counts.iter().any(|c| c + jokers == 3) {
            return Self::ThreeOfAKind;
        }

        // Two pairs
        if counts.len() == 3 {
            if counts.iter().filter(|c| **c == 2_u32).count() == 2_usize {
                return Self::TwoPair;
            }
        }

        Self::OnePair
    }
}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    labels: Vec<Label>,
    bid: u32,
}

impl Hand {
    fn new(labels: Vec<Label>, bid: u32) -> Self {
        let hand_type = HandType::from_labels(&labels);

        Hand {
            labels,
            bid,
            hand_type,
        }
    }

    fn cmp(&self, other: &Hand) -> Ordering {
        if self.hand_type.get_points() > other.hand_type.get_points() {
            return Ordering::Greater;
        } else if self.hand_type.get_points() < other.hand_type.get_points() {
            return Ordering::Less;
        }

        for i in 0..5 {
            if self.labels[i].get_point() > other.labels[i].get_point() {
                return Ordering::Greater;
            } else if self.labels[i].get_point() < other.labels[i].get_point() {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    }
}

#[derive(Debug)]
struct Label {
    val: char,
}

impl Label {
    fn get_point(&self) -> u32 {
        if self.val.is_ascii_digit() {
            return self.val.to_digit(10).unwrap();
        }

        match self.val {
            'T' => 10,
            'J' => 1,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => unreachable!(),
        }
    }
}

pub fn solve(input: &str) -> u32 {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(a, b)| {
                    Hand::new(
                        a.chars().map(|c| Label { val: c }).collect(),
                        b.parse().unwrap(),
                    )
                })
                .unwrap()
        })
        .collect();

    hands.sort_by(|a, b| a.cmp(b));

    let mut sum = 0;
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| {
            sum += (i + 1) as u32 * h.bid;
            (i + 1) as u32 * h.bid
        })
        .sum()
}
