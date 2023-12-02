const STR_NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_match(value: &str) -> Option<i32> {
    for (i, str_num) in STR_NUMBERS.iter().enumerate() {
        if value.starts_with(str_num) {
            return Some((i as i32) + 1);
        }
    }

    if let Some(first_char) = value.chars().next() {
        first_char.to_digit(10).map(|d| d as i32)
    } else {
        None
    }
}

fn get_calibration_value(value: &str) -> Option<i32> {
    let mut numbers: Vec<i32> = Vec::new();

    for i in 0..value.len() {
        if let Some(n) = find_match(&value[i..]) {
            numbers.push(n);
        };
    }

    if numbers.len() >= 1 {
        Some(numbers[0] * 10 + numbers.last().unwrap())
    } else {
        None
    }
}

pub fn get_calibration_sum(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        if let Some(value) = get_calibration_value(line) {
            sum += value;
        }
    }

    return sum;
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
        let sum = get_calibration_sum(&input);
        assert_eq!(sum, 281);
    }

    #[test]
    fn solve_input() {
        let input = read_input("input.txt");
        let sum = get_calibration_sum(&input);
        println!("Sum: {sum}");
    }
}
