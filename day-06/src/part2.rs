fn parse_line(line: &str) -> u64 {
    line.chars()
        .fold(String::new(), |mut acc, c| {
            if c.is_ascii_digit() {
                acc.push(c);
            }
            acc
        })
        .parse()
        .unwrap()
}

pub fn solve(input: &str) -> u32 {
    let mut lines = input.lines();

    let time_limit = parse_line(lines.next().unwrap());
    let record = parse_line(lines.next().unwrap());

    (1..time_limit)
        .map(|pressed_time| (time_limit - pressed_time) * pressed_time)
        .filter(|d| d > &record)
        .count() as u32
}
