enum AocError {
    ParseGameError(String),
}

#[derive(Debug)]
struct Num {
    val: u32,
    x: usize,
    y: usize,
    len: usize,
}

#[derive(Debug)]
struct Sym {
    x: usize,
    y: usize,
}

fn solve(input: String) -> u32 {
    let mut nums: Vec<Num> = Vec::new();
    let mut syms: Vec<Sym> = Vec::new();

    for (line_idx, line) in input.lines().enumerate() {
        let mut num = String::new();
        for (char_idx, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                num.push(char);
                continue;
            }

            if num.len() > 0 {
                nums.push(Num {
                    val: num.parse().expect(&format!("Error parsing {num}")),
                    x: char_idx - num.len(),
                    y: line_idx,
                    len: num.len(),
                });
                num.clear();
            }

            if char == '.' {
                continue;
            }

            syms.push(Sym {
                x: char_idx,
                y: line_idx,
            });
        }

        if num.len() > 0 {
            nums.push(Num {
                val: num.parse().expect(&format!("Error parsing {num}")),
                x: line.len() - num.len(),
                y: line_idx,
                len: num.len(),
            });
            num.clear();
        }
    }

    let mut sum = 0;

    'outer: for num in nums.iter() {
        for sym in syms.iter() {
            let min_x = if num.x == 0 { 0 } else { num.x - 1 };
            let min_y = if num.y == 0 { 0 } else { num.y - 1 };

            if ((sym.y >= min_y) && sym.y <= num.y + 1)
                && (sym.x >= min_x && sym.x <= num.x + num.len)
            {
                sum += num.val;
                continue 'outer;
            }
        }
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
        assert_eq!(solution, 4361);
    }

    #[test]
    fn solve_input() {
        let input = read_input("input.txt");
        let solution = solve(input);

        println!("Solution: {solution}");
        assert_eq!(solution, 535078)
    }
}
