use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    num: usize,
    matches: usize,
}

fn s(cards: &[Card]) -> u32 {
    if cards.len() == 0 {
        return 0;
    }

    let first = &cards[0];
    if first.matches == 0 {
        return 0;
    }

    let mut sum = first.matches as u32;
    for i in 1..first.matches + 1 {
        sum += s(&cards[i..]);
    }

    sum
}

fn solve(input: String) -> u32 {
    let mut cards: Vec<Card> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let mut a = line.split(&[':', '|']).skip(1);
        let mut set_nums: HashSet<u32> = HashSet::new();

        let mut matches = 0;

        a.next().unwrap().split(' ').for_each(|item| {
            if !item.is_empty() {
                set_nums.insert(item.parse::<u32>().unwrap());
            }
        });

        a.next().unwrap().split(' ').for_each(|item| {
            if !item.is_empty() {
                if set_nums.contains(&item.parse::<u32>().unwrap()) {
                    matches += 1;
                }
            }
        });

        cards.push(Card {
            num: i + 1,
            matches,
        })
    }

    let mut sum = 0;
    let mut i = 0;
    while i < cards.len() {
        sum += s(&cards[i..]);
        i += 1;
    }

    sum + cards.len() as u32
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
