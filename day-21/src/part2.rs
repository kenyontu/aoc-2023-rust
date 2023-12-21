use std::collections::{HashSet, VecDeque};

#[derive(Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
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
    let mut seen: HashSet<(usize, usize, isize, isize, Direction, u32)> = HashSet::new();

    //let needs_to_get_steps = 26501365;
    let needs_to_get_steps = 500;

    let mut queue: VecDeque<(usize, usize, isize, isize, u32)> = VecDeque::new();
    queue.push_back((starting_pos.0, starting_pos.1, 0, 0, 0));

    while let Some((x, y, grid_x, grid_y, steps)) = queue.pop_front() {
        if steps == needs_to_get_steps {
            plots.insert((x, y, grid_x, grid_y));
            continue;
        }

        // top
        {
            let grid_y = if y == 0 { grid_y - 1 } else { grid_y };
            let y = (y + width - 1) % height;
            if grid[y][x] != '#' {
                let signature = (x, y, grid_x, grid_y, Direction::Up, steps + 1);
                if !seen.contains(&signature) {
                    seen.insert(signature);
                    queue.push_back((x, y, grid_x, grid_y, steps + 1));
                }
            }
        }

        // right
        {
            let grid_x = if x + 1 == width { grid_x + 1 } else { grid_x };
            let x = (x + 1) % width;
            if grid[y][x] != '#' {
                let signature = (x, y, grid_x, grid_y, Direction::Right, steps + 1);
                if !seen.contains(&signature) {
                    seen.insert(signature);
                    queue.push_back((x, y, grid_x, grid_y, steps + 1));
                }
            }
        }

        // bottom
        {
            let grid_y = if y + 1 == height { grid_y + 1 } else { grid_y };
            let y = (y + 1) % height;
            if grid[y][x] != '#' {
                let signature = (x, y, grid_x, grid_y, Direction::Down, steps + 1);
                if !seen.contains(&signature) {
                    seen.insert(signature);
                    queue.push_back((x, y, grid_x, grid_y, steps + 1));
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
                    queue.push_back((x, y, grid_x, grid_y, steps + 1));
                }
            }
        }
    }

    plots.len()
}
