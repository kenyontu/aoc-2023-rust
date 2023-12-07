use std::collections::HashSet;

type CardMatches = usize;

fn calc_matches(matches_by_card: &[CardMatches]) -> u32 {
    if matches_by_card.len() == 0 {
        return 0;
    }

    let first = &matches_by_card[0];
    let mut sum = *first as u32;
    for i in 1..first + 1 {
        sum += calc_matches(&matches_by_card[i..]);
    }

    sum
}

pub fn solve(input: &str) -> u32 {
    let mut cards: Vec<CardMatches> = Vec::new();

    for line in input.lines() {
        let (str_winner, str_mine) = line.split_once(':').unwrap().1.split_once('|').unwrap();
        let winner =
            str_winner
                .split(' ')
                .filter(|s| !s.is_empty())
                .fold(HashSet::new(), |mut set, s| {
                    set.insert(s.parse::<u32>().unwrap());
                    set
                });

        let win_count = str_mine
            .split(' ')
            .filter(|s| !s.is_empty() && winner.contains(&s.parse::<u32>().unwrap()))
            .count();

        cards.push(win_count)
    }

    let mut sum = cards.len() as u32;
    for i in 0..cards.len() {
        sum += calc_matches(&cards[i..]);
    }

    sum
}
