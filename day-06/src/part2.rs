fn parse_line(line: &str) -> u64 {
    line.split(':')
        .skip(1)
        .next()
        .unwrap()
        .split(' ')
        .filter(|x| !x.is_empty())
        .fold(String::new(), |mut s, n| {
            s.push_str(n);
            s
        })
        .parse()
        .unwrap()
}

pub fn solve(input: String) -> u64 {
    let mut lines = input.lines();

    let time_limit = parse_line(lines.next().unwrap());
    let record = parse_line(lines.next().unwrap());

    let mut result = 0;
    let mut ways_to_win = 0;

    let mut pressed_time = 1;
    loop {
        let distance_per_ms = pressed_time;
        let distance = (time_limit - pressed_time) * distance_per_ms;
        if distance <= 0 {
            break;
        }
        if distance > record {
            ways_to_win += 1;
        }
        pressed_time += 1;
    }

    if result == 0 {
        result = ways_to_win;
    } else {
        result *= ways_to_win
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
