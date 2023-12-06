fn parse_line(line: &str) -> Vec<u32> {
    line.split(':')
        .skip(1)
        .next()
        .unwrap()
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}

pub fn solve(input: String) -> u32 {
    let mut lines = input.lines();

    let times = parse_line(lines.next().unwrap());
    let distances = parse_line(lines.next().unwrap());

    let mut result = 1;

    let mut i = 0;
    while i < times.len() {
        let time_limit = times[i];
        let record = distances[i];

        let ways_to_win = (1..time_limit)
            .map(|pressed_time| (time_limit - pressed_time) * pressed_time)
            .filter(|distance| distance > &record)
            .fold(0_u32, |acc, _| acc + 1);

        result *= ways_to_win;

        i += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::read_input;

    #[test]
    fn solve_input_1() {
        let input = read_input("part1_input1.txt");
        let solution = solve(input);

        println!("Solution: {solution}");
        assert_eq!(solution, 288);
    }

    #[test]
    fn solve_input() {
        let input = read_input("input.txt");
        let solution = solve(input);

        println!("Solution: {solution}");
        assert_eq!(solution, 128700)
    }
}
