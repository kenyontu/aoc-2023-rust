use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum BeamDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Beam {
    pos: (usize, usize),
    direction: BeamDirection,
}

impl Beam {
    fn next_pos(&self) -> Option<(usize, usize)> {
        let pos = match self.direction {
            BeamDirection::Up => (self.pos.0, self.pos.1.checked_sub(1)?),
            BeamDirection::Right => (self.pos.0 + 1, self.pos.1),
            BeamDirection::Down => (self.pos.0, self.pos.1 + 1),
            BeamDirection::Left => (self.pos.0.checked_sub(1)?, self.pos.1),
        };

        Some(pos)
    }
}

fn next_beams(tile: &char, pos: (usize, usize), direction: BeamDirection) -> Vec<Beam> {
    if tile == &'-' && (direction == BeamDirection::Up || direction == BeamDirection::Down) {
        return vec![
            Beam {
                pos: pos,
                direction: BeamDirection::Left,
            },
            Beam {
                pos: pos,
                direction: BeamDirection::Right,
            },
        ];
    }

    if tile == &'|' && (direction == BeamDirection::Left || direction == BeamDirection::Right) {
        return vec![
            Beam {
                pos: pos,
                direction: BeamDirection::Up,
            },
            Beam {
                pos: pos,
                direction: BeamDirection::Down,
            },
        ];
    }

    if tile == &'/' {
        let next_direction = match direction {
            BeamDirection::Up => BeamDirection::Right,
            BeamDirection::Right => BeamDirection::Up,
            BeamDirection::Down => BeamDirection::Left,
            BeamDirection::Left => BeamDirection::Down,
        };

        return vec![Beam {
            pos: pos,
            direction: next_direction,
        }];
    }

    if tile == &'\\' {
        let next_direction = match direction {
            BeamDirection::Up => BeamDirection::Left,
            BeamDirection::Right => BeamDirection::Down,
            BeamDirection::Down => BeamDirection::Right,
            BeamDirection::Left => BeamDirection::Up,
        };

        return vec![Beam {
            pos: pos,
            direction: next_direction,
        }];
    }

    vec![Beam { pos, direction }]
}

fn shoot_beam(
    grid: &Vec<Vec<char>>,
    start_pos: (usize, usize),
    start_direction: BeamDirection,
) -> usize {
    let grid_h = grid.len();
    let grid_w = grid[0].len();

    let mut energized_tiles: HashSet<(usize, usize, BeamDirection)> = HashSet::new();

    let mut beams = VecDeque::from(next_beams(
        &grid[start_pos.1][start_pos.0],
        start_pos,
        start_direction,
    ));

    while let Some(beam) = beams.pop_back() {
        let curr_tile = (beam.pos.0, beam.pos.1, beam.direction.clone());
        if energized_tiles.contains(&curr_tile) {
            continue;
        }
        energized_tiles.insert(curr_tile);

        let Some(next_pos) = beam.next_pos() else {
            continue;
        };

        if next_pos.0 >= grid_w || next_pos.1 >= grid_h {
            continue;
        }

        let next_tile = grid[next_pos.1][next_pos.0];

        next_beams(&next_tile, next_pos, beam.direction)
            .into_iter()
            .for_each(|b| beams.push_back(b));
    }

    energized_tiles
        .iter()
        .map(|tile| (tile.0, tile.1))
        .collect::<HashSet<(usize, usize)>>()
        .len()
}

pub fn solve(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let top = (0..grid.len())
        .map(|x| shoot_beam(&grid, (x, 0), BeamDirection::Down))
        .max()
        .unwrap();

    let right = (0..grid[0].len())
        .map(|y| shoot_beam(&grid, (grid[y].len() - 1, y), BeamDirection::Right))
        .max()
        .unwrap();

    let bottom = (0..grid.len())
        .map(|x| shoot_beam(&grid, (x, grid.len() - 1), BeamDirection::Down))
        .max()
        .unwrap();

    let left = (0..grid[0].len())
        .map(|y| shoot_beam(&grid, (0, y), BeamDirection::Right))
        .max()
        .unwrap();

    *[top, right, bottom, left].iter().max().unwrap()
}
