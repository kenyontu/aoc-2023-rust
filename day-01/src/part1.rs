pub fn get_calibration_value(value: &str) -> Option<i32> {
    let mut digits: Vec<i32> = Vec::new();

    for c in value.chars() {
        if let Some(digit) = c.to_digit(10) {
            digits.push(digit as i32);
        }
    }

    if digits.len() >= 1 {
        Some(digits[0] * 10 + digits.last().unwrap())
    } else {
        None
    }
}

pub fn get_calibration_sum(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        if let Some(calibration_value) = get_calibration_value(line) {
            sum += calibration_value;
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
        let input = read_input("part1_input1.txt");
        let sum = get_calibration_sum(&input);
        assert_eq!(sum, 142);
    }

    #[test]
    fn solve_input() {
        let input = read_input("input.txt");
        let sum = get_calibration_sum(&input);
        println!("Sum: {sum}");
    }
}
