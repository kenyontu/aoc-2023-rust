const STR_NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_match(value: &str) -> Option<u32> {
    if let Some(first_char) = value.chars().next() {
        if first_char.is_ascii_digit() {
            return first_char.to_digit(10);
        }
    }

    for (i, str_num) in STR_NUMBERS.iter().enumerate() {
        if value.starts_with(str_num) {
            return Some((i as u32) + 1);
        }
    }

    None
}

fn get_calibration_value(value: &str) -> Option<u32> {
    let numbers: Vec<u32> = (0..value.len())
        .filter_map(|i| find_match(&value[i..]))
        .collect();

    match numbers.len() >= 1 {
        true => Some(numbers[0] * 10 + numbers.last().unwrap()),
        _ => None,
    }
}

pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| get_calibration_value(line))
        .sum()
}
