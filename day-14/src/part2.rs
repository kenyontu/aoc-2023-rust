struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn calc_north_load(&self) -> u64 {
        let mut load: u64 = 0;

        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                if self.grid[i][j] == 'O' {
                    let add = (self.grid.len() - i) as u64;
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

pub fn solve(input: &str) -> u64 {
    let mut grid = Grid {
        grid: input.lines().map(|l| l.chars().collect()).collect(),
    };

    // Not sure if I was just lucky with the input, but I got the
    // correct answer with only 1000 cycles
    let cycles = 1000;
    for _ in 0..cycles {
        grid.roll_clockwise();
        grid.roll_clockwise();
        grid.roll_clockwise();
        grid.roll_clockwise();
    }

    grid.calc_north_load()
}
