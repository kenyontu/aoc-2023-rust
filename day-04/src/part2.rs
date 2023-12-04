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

fn solve(input: String) -> u32 {
    let mut cards: Vec<CardMatches> = Vec::new();

    for line in input.lines() {
        let mut split = line.split(&[':', '|']).skip(1);
        let mut set_nums: HashSet<u32> = HashSet::new();

        let mut matches = 0;

        split.next().unwrap().split(' ').for_each(|item| {
            if !item.is_empty() {
                set_nums.insert(item.parse::<u32>().unwrap());
            }
        });

        split.next().unwrap().split(' ').for_each(|item| {
            if !item.is_empty() {
                if set_nums.contains(&item.parse::<u32>().unwrap()) {
                    matches += 1;
                }
            }
        });

        cards.push(matches)
    }

    let mut sum = cards.len() as u32;
    for i in 0..cards.len() {
        sum += calc_matches(&cards[i..]);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn read_input(path: &str) -> String {
        fs::read_to_string(path).expect(&format!("{path} file not found"))
    }

    #[test]
    fn solve_input_1() {
        let input = read_input("part2_input1.txt");
        let solution = solve(input);

        println!("{solution}");
        assert_eq!(solution, 30);
    }

    #[test]
    fn solve_input() {
        let input = read_input("input.txt");
        let solution = solve(input);

        println!("Solution: {solution}");
        assert_eq!(solution, 6189740)
    }
}
