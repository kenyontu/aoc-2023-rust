fn parse_line(line: &str) -> u64 {
    line.split(':')
        .skip(1)
        .next()
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap()
}

pub fn solve(input: String) -> u64 {
    let mut lines = input.lines();

    let time_limit = parse_line(lines.next().unwrap());
    let record = parse_line(lines.next().unwrap());

    let distances = (1..time_limit).map(|pressed_time| (time_limit - pressed_time) * pressed_time);

    distances.fold(
        0_u64,
        |acc, distance| {
            if distance > record {
                acc + 1
            } else {
                acc
            }
        },
    )
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
        assert_eq!(solution, 71503);
    }

    #[test]
    fn solve_input() {
        let input = read_input("input.txt");
        let solution = solve(input);

        println!("Solution: {solution}");
        assert_eq!(solution, 39594072)
    }
}
