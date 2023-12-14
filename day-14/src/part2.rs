struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn calc_north_load(&self) -> u32 {
        let mut load: u32 = 0;

        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                if self.grid[i][j] == 'O' {
                    let add = (self.grid.len() - i) as u32;
                    load += add;
                }
            }
        }

        load
    }

    fn roll_clockwise(&mut self) {
        let line_len = self.grid[0].len();

        let mut new_grid: Vec<Vec<char>> = vec![vec!['.'; line_len]; self.grid.len()];

        for i in 0..line_len {
            let mut f = 0;
            for j in 0..self.grid.len() {
                let c = self.grid[j][i];
                if c == '#' {
                    new_grid[j][i] = '#';
                    f = j + 1;
                    continue;
                }

                if c == 'O' {
                    new_grid[f][i] = 'O';
                    f += 1;
                    continue;
                }
            }
        }

        for i in 0..line_len {
            for j in 0..self.grid.len() {
                let j_rev = self.grid.len() - 1 - j;
                self.grid[i][j] = new_grid[j_rev][i];
            }
        }
    }
}

// The idea is to take the last 3 items and walk backwards until we find
// the same trio, then by calculating the difference of the position of the
// original trio we can figure out the pattern and the position it starts.
pub fn find_pattern(nums: &[u32]) -> Option<(&[u32], usize)> {
    let last_three = &nums[nums.len() - 3..];
    let prev_match_index = nums.len() - 3;

    for i in (0..nums.len() - 6).rev() {
        if &nums[i..i + 3] == last_three {
            let pattern_len = prev_match_index.abs_diff(i);

            // Making sure there's a previous repeating pattern
            if &nums[i..prev_match_index] == &nums[i - pattern_len..i] {
                return Some((&nums[i - pattern_len..i], i - pattern_len));
            }
        }
    }

    None
}

pub fn solve(input: &str) -> u32 {
    let mut grid = Grid {
        grid: input.lines().map(|l| l.chars().collect()).collect(),
    };

    let mut loads: Vec<u32> = Vec::new();
    let cycles = 500;
    for _ in 0..cycles {
        grid.roll_clockwise();
        grid.roll_clockwise();
        grid.roll_clockwise();
        grid.roll_clockwise();
        loads.push(grid.calc_north_load());
    }

    if let Some((pattern, index)) = find_pattern(&loads) {
        let res_index = (1_000_000_000 - index) % pattern.len() - 1;
        return pattern[res_index];
    }

    unreachable!();
}
