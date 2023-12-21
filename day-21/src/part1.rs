use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn walk(
    x: usize,
    y: usize,
    width: &usize,
    height: &usize,
    steps: u32,
    grid: &Vec<Vec<char>>,
    plots: &mut HashSet<(usize, usize)>,
    seen: &mut HashSet<(usize, usize, Direction, u32)>,
) {
    if steps == 64 {
        plots.insert((x, y));
        return;
    }

    // top
    if let Some(y) = y.checked_sub(1) {
        if grid[y][x] != '#' {
            let signature = (x, y, Direction::Up, steps + 1);
            if !seen.contains(&signature) {
                seen.insert(signature);
                walk(x, y, width, height, steps + 1, grid, plots, seen);
            }
        }
    }

    // right
    if x < width - 2 {
        if grid[y][x + 1] != '#' {
            let signature = (x + 1, y, Direction::Right, steps + 1);
            if !seen.contains(&signature) {
                seen.insert(signature);
                walk(x + 1, y, width, height, steps + 1, grid, plots, seen);
            }
        }
    }

    // bottom
    if y < height - 2 {
        if grid[y + 1][x] != '#' {
            let signature = (x, y + 1, Direction::Down, steps + 1);
            if !seen.contains(&signature) {
                seen.insert(signature);
                walk(x, y + 1, width, height, steps + 1, grid, plots, seen);
            }
        }
    }

    // left
    if let Some(x) = x.checked_sub(1) {
        if grid[y][x] != '#' {
            let signature = (x, y, Direction::Left, steps + 1);
            if !seen.contains(&signature) {
                seen.insert(signature);
                walk(x, y, width, height, steps + 1, grid, plots, seen);
            }
        }
    }
}

pub fn solve(input: &str) -> usize {
    let mut starting_pos = (0, 0);

    let grid = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'S' {
                        starting_pos = (i, j)
                    }
                    c
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    let mut plots: HashSet<(usize, usize)> = HashSet::new();

    let mut seen = HashSet::new();
    walk(
        starting_pos.0,
        starting_pos.1,
        &width,
        &height,
        0,
        &grid,
        &mut plots,
        &mut seen,
    );

    plots.len()
}
