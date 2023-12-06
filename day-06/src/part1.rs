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

    let mut result = 0;

    let mut i = 0;
    while i < times.len() {
        let time_limit = times[i];
        let record = distances[i];

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

        if ways_to_win == 0 {
            continue;
        }

        if result == 0 {
            result = ways_to_win;
        } else {
            result *= ways_to_win
        }

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
