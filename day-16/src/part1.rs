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

fn starting_beam(tile: &char) -> Vec<Beam> {
    if tile == &'/' {
        return vec![Beam {
            pos: (0, 0),
            direction: BeamDirection::Up,
        }];
    }

    if tile == &'\\' || tile == &'|' {
        return vec![Beam {
            pos: (0, 0),
            direction: BeamDirection::Down,
        }];
    }

    vec![Beam {
        pos: (0, 0),
        direction: BeamDirection::Right,
    }]
}

pub fn solve(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let grid_h = grid.len();
    let grid_w = grid[0].len();

    let mut energized_tiles: HashSet<(usize, usize, BeamDirection)> = HashSet::new();

    let mut beams = VecDeque::from(starting_beam(&grid[0][0]));

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

        if next_tile == '|'
            && (beam.direction == BeamDirection::Right || beam.direction == BeamDirection::Left)
        {
            beams.push_back(Beam {
                pos: next_pos,
                direction: BeamDirection::Up,
            });

            beams.push_back(Beam {
                pos: next_pos,
                direction: BeamDirection::Down,
            });

            continue;
        }

        if next_tile == '-'
            && (beam.direction == BeamDirection::Up || beam.direction == BeamDirection::Down)
        {
            beams.push_back(Beam {
                pos: next_pos,
                direction: BeamDirection::Left,
            });

            beams.push_back(Beam {
                pos: next_pos,
                direction: BeamDirection::Right,
            });

            continue;
        }

        if next_tile == '/' {
            let direction = match beam.direction {
                BeamDirection::Up => BeamDirection::Right,
                BeamDirection::Right => BeamDirection::Up,
                BeamDirection::Down => BeamDirection::Left,
                BeamDirection::Left => BeamDirection::Down,
            };

            beams.push_back(Beam {
                pos: next_pos,
                direction,
            });

            continue;
        }

        if next_tile == '\\' {
            let direction = match beam.direction {
                BeamDirection::Up => BeamDirection::Left,
                BeamDirection::Right => BeamDirection::Down,
                BeamDirection::Down => BeamDirection::Right,
                BeamDirection::Left => BeamDirection::Up,
            };

            beams.push_back(Beam {
                pos: next_pos,
                direction,
            });

            continue;
        }

        beams.push_back(Beam {
            pos: next_pos,
            direction: beam.direction,
        });
    }

    energized_tiles
        .iter()
        .map(|tile| (tile.0, tile.1))
        .collect::<HashSet<(usize, usize)>>()
        .len()
}
