pub fn solve(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut load: u64 = 0;

    for i in 0..grid[0].len() {
        let mut f = 0;
        for j in 0..grid.len() {
            let c = grid[j][i];
            if c == '#' {
                f = j + 1;
                continue;
            }

            if c == 'O' {
                let add = (grid.len() - f) as u64;
                load += add;
                f += 1;
                continue;
            }
        }
    }

    load
}
