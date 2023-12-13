fn measure_reflection(input: &[u32]) -> Option<u32> {
    for i in 0..input.len() - 1 {
        if input[i] == input[i + 1] {
            let mut start = i;
            let mut end = i + 1;

            while input[start] == input[end] {
                if start == 0 || end == input.len() - 1 {
                    return Some((i + 1) as u32);
                }

                start -= 1;
                end += 1;
            }
        }
    }

    None
}

pub fn solve(input: &str) -> u32 {
    let b: Vec<u32> = (0..32).map(|n| 2_u32.pow(n)).collect();

    let a = input
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

            if let Some(h) = measure_reflection(&horizontal) {
                return h * 100;
            };
            if let Some(v) = measure_reflection(&vertical) {
                return v;
            }
            unreachable!();
        })
        .sum();

    a
}
