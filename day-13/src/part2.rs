use std::collections::HashSet;

fn measure_reflection(input: &[u32], b_set: &HashSet<&u32>) -> Option<usize> {
    for i in 0..input.len() - 1 {
        let mut start = i;
        let mut end = i + 1;
        let mut cleaned_count = 0;

        loop {
            let start_val = input[start];
            let end_val = input[end];

            if start_val != end_val {
                if cleaned_count == 0 && b_set.contains(&(start_val ^ end_val)) {
                    cleaned_count += 1;
                } else {
                    break;
                }
            }

            if start == 0 || end == input.len() - 1 {
                if cleaned_count > 0 {
                    return Some(i + 1);
                } else {
                    break;
                }
            }

            start -= 1;
            end += 1;
        }
    }

    None
}

pub fn solve(input: &str) -> u32 {
    let b: Vec<u32> = (0..32).map(|n| 2_u32.pow(n)).collect();
    let b_set: HashSet<&u32> = HashSet::from_iter(b.iter());

    input
        .split_terminator("\n\n")
        .map(|pattern| {
            let line_count = pattern.lines().count();
            let mut vertical: Vec<u32> = vec![0; pattern.lines().nth(0).unwrap().len()];

            let horizontal: Vec<u32> = pattern
                .lines()
                .enumerate()
                .map(|(li, l)| {
                    l.chars().enumerate().fold(0, |mut acc, (ci, c)| {
                        if c == '#' {
                            acc += &b[l.len() - 1 - ci];
                            vertical[ci] += &b[line_count - 1 - li];
                        }
                        acc
                    })
                })
                .collect();

            if let Some(h) = measure_reflection(&horizontal, &b_set) {
                return (h * 100) as u32;
            };

            if let Some(v) = measure_reflection(&vertical, &b_set) {
                return v as u32;
            }
            unreachable!();
        })
        .sum()
}
