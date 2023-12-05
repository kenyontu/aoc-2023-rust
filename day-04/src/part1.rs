use std::collections::HashSet;

fn solve(input: String) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut split = line.split(&[':', '|']).skip(1);
        let mut set_nums: HashSet<u32> = HashSet::new();

        let mut points = 0;

        split.next().unwrap().split(' ').for_each(|item| {
            if !item.is_empty() {
                set_nums.insert(item.parse::<u32>().unwrap());
            }
        });

        split.next().unwrap().split(' ').for_each(|item| {
            if item.is_empty() {
                return;
            }

            if set_nums.contains(&item.parse::<u32>().unwrap()) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        });

        sum += points;
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
        let input = read_input("part1_input1.txt");
        let solution = solve(input);

        println!("{solution}");
        assert_eq!(solution, 13);
    }

    #[test]
    fn solve_input() {
        let input = read_input("input.txt");
        let solution = solve(input);

        println!("Solution: {solution}");
        assert_eq!(solution, 15205)
    }
}
