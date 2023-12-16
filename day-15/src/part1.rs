pub fn solve(input: &str) -> u32 {
    input
        .split(',')
        .map(|s| {
            s.trim().chars().fold(0_u32, |mut acc, c| {
                acc += c as u32;
                acc *= 17;
                acc %= 256;
                acc
            })
        })
        .sum()
}
