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
    is_gear: bool,
}

pub fn solve(input: &str) -> u32 {
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
                is_gear: char == '*',
            });
        }

        if num.len() > 0 {
            nums.push(Num {
                val: num.parse().expect(&format!("Error parsing {num}")),
                x: line.len() - num.len(),
                y: line_idx,
                len: num.len(),
            });
        }
    }

    let mut sum = 0;

    for sym in syms.iter() {
        if !sym.is_gear {
            continue;
        }

        let mut adjacent_count = 0;
        let mut gear_ratio = 1;

        for num in nums.iter() {
            let min_x = if num.x == 0 { 0 } else { num.x - 1 };
            let min_y = if num.y == 0 { 0 } else { num.y - 1 };

            if ((sym.y >= min_y) && sym.y <= num.y + 1)
                && (sym.x >= min_x && sym.x <= num.x + num.len)
            {
                adjacent_count += 1;
                gear_ratio *= num.val;
            }
        }

        if adjacent_count == 2 {
            sum += gear_ratio;
        }
    }

    sum
}
