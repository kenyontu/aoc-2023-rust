use std::collections::{HashMap, HashSet, VecDeque};

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
    grid_x: isize,
    grid_y: isize,
    width: &usize,
    height: &usize,
    steps: u32,
    grid: &Vec<Vec<char>>,
    plots: &mut HashSet<(usize, usize, isize, isize)>,
    seen: &mut HashSet<(usize, usize, isize, isize, Direction, u32)>,
) {
    if steps == 50 {
        plots.insert((x, y, grid_x, grid_y));
        return;
    }

    // top
    {
        let grid_y = if y == 0 { grid_y - 1 } else { grid_y };
        let y = (y + width - 1) % height;
        if grid[y][x] != '#' {
            let signature = (x, y, grid_x, grid_y, Direction::Up, steps + 1);
            if !seen.contains(&signature) {
                seen.insert(signature);
                walk(
                    x,
                    y,
                    grid_x,
                    grid_y,
                    width,
                    height,
                    steps + 1,
                    grid,
                    plots,
                    seen,
                );
            }
        }
    }

    // right
    {
        let grid_x = if x + 1 == *width { grid_x + 1 } else { grid_x };
        let x = (x + 1) % width;
        if grid[y][x] != '#' {
            let signature = (x, y, grid_x, grid_y, Direction::Right, steps + 1);
            if !seen.contains(&signature) {
                seen.insert(signature);
                walk(
                    x,
                    y,
                    grid_x,
                    grid_y,
                    width,
                    height,
                    steps + 1,
                    grid,
                    plots,
                    seen,
                );
            }
        }
    }

    // bottom
    {
        let grid_y = if y + 1 == *height { grid_y + 1 } else { grid_y };
        let y = (y + 1) % height;
        if grid[y][x] != '#' {
            let signature = (x, y, grid_x, grid_y, Direction::Down, steps + 1);
            if !seen.contains(&signature) {
                seen.insert(signature);
                walk(
                    x,
                    y,
                    grid_x,
                    grid_y,
                    width,
                    height,
                    steps + 1,
                    grid,
                    plots,
                    seen,
                );
            }
        }
    }

    // left
    {
        let grid_x = if x == 0 { grid_x - 1 } else { grid_x };
        let x = (x + width - 1) % width;
        if grid[y][x] != '#' {
            let signature = (x, y, grid_x, grid_y, Direction::Left, steps + 1);
            if !seen.contains(&signature) {
                seen.insert(signature);
                walk(
                    x,
                    y,
                    grid_x,
                    grid_y,
                    width,
                    height,
                    steps + 1,
                    grid,
                    plots,
                    seen,
                );
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

    let mut plots: HashSet<(usize, usize, isize, isize)> = HashSet::new();
    let mut rocks: HashSet<(usize, usize, isize, isize)> = HashSet::new();
    let needs_to_get_steps = 6;

    let mut seen = HashSet::new();
    walk(
        starting_pos.0,
        starting_pos.1,
        0,
        0,
        &width,
        &height,
        0,
        &grid,
        &mut plots,
        &mut seen,
    );

    plots.len()
}
