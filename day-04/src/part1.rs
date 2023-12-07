use std::collections::HashSet;

pub fn solve(input: &str) -> u32 {
    let mut sum = 0;

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

        if win_count > 0 {
            sum += 2_u32.pow((win_count - 1) as u32);
        }
    }

    sum
}
