pub fn get_calibration_value(value: &str) -> Option<u32> {
    let digits: Vec<u32> = value
        .chars()
        .filter(|c| c.is_ascii_digit())
        .filter_map(|c| c.to_digit(10))
        .collect();

    match digits.len() >= 1 {
        true => Some(digits[0] * 10 + digits.last().unwrap()),
        _ => None,
    }
}

pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| get_calibration_value(line))
        .sum()
}
