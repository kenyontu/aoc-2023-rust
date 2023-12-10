use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Pos { x, y }
    }

    pub fn top(&self) -> Option<Self> {
        self.y.checked_sub(1).map(|y| Self::new(self.x, y))
    }

    pub fn right(&self) -> Option<Self> {
        self.x.checked_add(1).map(|x| Self::new(x, self.y))
    }

    pub fn bottom(&self) -> Option<Self> {
        self.y.checked_add(1).map(|y| Self::new(self.x, y))
    }

    pub fn left(&self) -> Option<Self> {
        self.x.checked_sub(1).map(|x| Self::new(x, self.y))
    }
}

#[derive(Debug)]
struct Tile {
    pos: Pos,
    c: char,
    is_main_loop: bool,
}

impl Tile {
    fn new(x: usize, y: usize, c: char) -> Self {
        Self {
            pos: Pos { x, y },
            c,
            is_main_loop: false,
        }
    }
}

type Grid = Vec<Vec<Tile>>;

#[derive(Debug)]
struct Maze {
    pub width: usize,
    pub height: usize,
    pub grid: Grid,
}

impl Maze {
    pub fn new(grid: Grid) -> Self {
        let width = grid[0].len();
        let height = grid.len();

        Self {
            grid,
            width,
            height,
        }
    }

    fn check_is_pos_valid(&self, pos: &Pos) -> bool {
        let max_x = self.width - 1;
        let max_y = self.height - 1;

        pos.x <= max_x && pos.y <= max_y
    }

    pub fn get_tile(&self, pos: &Pos) -> Option<&Tile> {
        self.check_is_pos_valid(pos)
            .then(|| &self.grid[pos.y][pos.x])
    }

    pub fn set_part_of_main_loop(&mut self, pos: &Pos) {
        if self.check_is_pos_valid(pos) {
            self.grid[pos.y][pos.x].is_main_loop = true;
        }
    }

    fn get_loop_positions(&self, start_pos: &Pos) -> Vec<Pos> {
        let mut prev = self.get_tile(start_pos).unwrap();
        let end = &get_pipe_ends(prev).unwrap()[0];

        let mut loop_positions: Vec<Pos> = Vec::new();
        loop_positions.push(prev.pos.clone());

        let mut curr = self.get_tile(end).unwrap();

        while &curr.pos != start_pos {
            loop_positions.push(curr.pos.clone());
            let temp = curr;
            curr = next_pos(&self, &prev, &curr);
            prev = temp;
        }

        loop_positions
    }
}

fn get_pipe_ends(tile: &Tile) -> Option<[Pos; 2]> {
    let Tile { pos, c, .. } = tile;

    let ends = match c {
        '|' => [pos.top()?, pos.bottom()?],
        '-' => [pos.left()?, pos.right()?],
        'L' => [pos.top()?, pos.right()?],
        'J' => [pos.top()?, pos.left()?],
        '7' => [pos.left()?, pos.bottom()?],
        'F' => [pos.right()?, pos.bottom()?],
        _ => return None,
    };

    Some(ends)
}

fn identify_starting_pipe(maze: &Maze, start_pos: &Pos) -> Tile {
    let mut has_connection = (false, false, false, false);

    // Check top

    if let Some(ends) = start_pos
        .top()
        .and_then(|pos| maze.get_tile(&pos))
        .and_then(|tile| get_pipe_ends(tile))
    {
        if ends.iter().any(|end| end == start_pos) {
            has_connection.0 = true;
        }
    }

    // Check right
    if let Some(ends) = start_pos
        .right()
        .and_then(|pos| maze.get_tile(&pos))
        .and_then(|tile| get_pipe_ends(tile))
    {
        if ends.iter().any(|end| end == start_pos) {
            has_connection.1 = true;
        }
    }

    // Check bottom
    if let Some(ends) = start_pos
        .bottom()
        .and_then(|pos| maze.get_tile(&pos))
        .and_then(|tile| get_pipe_ends(tile))
    {
        if ends.iter().any(|end| end == start_pos) {
            has_connection.2 = true;
        }
    }

    // Check left
    if let Some(ends) = start_pos
        .left()
        .and_then(|pos| maze.get_tile(&pos))
        .and_then(|pos| get_pipe_ends(pos))
    {
        if ends.iter().any(|end| end == start_pos) {
            has_connection.3 = true;
        }
    }

    let pipe_type = match has_connection {
        (true, false, true, false) => '|',
        (false, true, false, true) => '-',
        (true, true, false, false) => 'L',
        (true, false, false, true) => 'J',
        (false, false, true, true) => '7',
        (false, true, true, false) => 'F',
        _ => unreachable!(),
    };

    let mut tile = Tile::new(start_pos.x, start_pos.y, pipe_type);
    tile.is_main_loop = true;
    tile
}

fn next_pos<'a>(maze: &'a Maze, prev_tile: &Tile, curr_tile: &Tile) -> &'a Tile {
    let [end1, end2] = get_pipe_ends(curr_tile).unwrap();

    if end1 != prev_tile.pos {
        maze.get_tile(&end1).unwrap()
    } else {
        maze.get_tile(&end2).unwrap()
    }
}

enum SimpleTile<'a> {
    Block,
    Clear(&'a Pos),
}

fn generate_tiles<'a>(pattern: &str, pos: &'a Pos) -> Vec<SimpleTile<'a>> {
    pattern
        .chars()
        .map(|c| {
            if c == '.' {
                SimpleTile::Block
            } else {
                SimpleTile::Clear(pos)
            }
        })
        .collect()
}

fn upscale(line: &Vec<Tile>) -> Vec<Vec<SimpleTile>> {
    let mut new_lines: Vec<Vec<SimpleTile>> = Vec::new();
    new_lines.push(Vec::new());
    new_lines.push(Vec::new());
    new_lines.push(Vec::new());

    line.iter().for_each(|tile| {
        if !tile.is_main_loop {
            new_lines[0].extend(generate_tiles("   ", &tile.pos));
            new_lines[1].extend(generate_tiles("   ", &tile.pos));
            new_lines[2].extend(generate_tiles("   ", &tile.pos));
            return;
        }

        match tile.c {
            '|' => {
                new_lines[0].extend(generate_tiles(" . ", &tile.pos));
                new_lines[1].extend(generate_tiles(" . ", &tile.pos));
                new_lines[2].extend(generate_tiles(" . ", &tile.pos));
            }
            '-' => {
                new_lines[0].extend(generate_tiles("   ", &tile.pos));
                new_lines[1].extend(generate_tiles("...", &tile.pos));
                new_lines[2].extend(generate_tiles("   ", &tile.pos));
            }
            'L' => {
                new_lines[0].extend(generate_tiles(" . ", &tile.pos));
                new_lines[1].extend(generate_tiles(" ..", &tile.pos));
                new_lines[2].extend(generate_tiles("   ", &tile.pos));
            }
            'J' => {
                new_lines[0].extend(generate_tiles(" . ", &tile.pos));
                new_lines[1].extend(generate_tiles(".. ", &tile.pos));
                new_lines[2].extend(generate_tiles("   ", &tile.pos));
            }
            '7' => {
                new_lines[0].extend(generate_tiles("   ", &tile.pos));
                new_lines[1].extend(generate_tiles(".. ", &tile.pos));
                new_lines[2].extend(generate_tiles(" . ", &tile.pos));
            }
            'F' => {
                new_lines[0].extend(generate_tiles("   ", &tile.pos));
                new_lines[1].extend(generate_tiles(" ..", &tile.pos));
                new_lines[2].extend(generate_tiles(" . ", &tile.pos));
            }
            _ => unreachable!(),
        }
    });

    new_lines
}

pub fn solve(input: &str) -> u32 {
    let mut start_pos = Pos { x: 0, y: 0 };

    let grid = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start_pos = Pos { x, y };
                    }

                    Tile::new(x, y, c)
                })
                .collect()
        })
        .collect::<Grid>();

    let mut maze = Maze::new(grid);
    maze.grid[start_pos.y][start_pos.x] = identify_starting_pipe(&maze, &start_pos);

    let loop_positions = maze.get_loop_positions(&start_pos);

    loop_positions
        .iter()
        .for_each(|pos| maze.set_part_of_main_loop(&pos));

    let upscaled: Vec<Vec<SimpleTile>> = maze.grid.iter().fold(Vec::new(), |mut acc, line| {
        acc.extend(upscale(&line));
        acc
    });

    let mut visited_orig_positions: HashSet<&Pos> = HashSet::new();
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut to_visit: VecDeque<(usize, usize)> = VecDeque::new();

    to_visit.push_back((0, 0));

    while to_visit.len() > 0 {
        let (x, y) = to_visit.pop_back().unwrap();

        match &upscaled[y][x] {
            SimpleTile::Clear(pos) => {
                if visited_positions.contains(&(x, y)) {
                    continue;
                } else {
                    visited_orig_positions.insert(pos);
                    visited_positions.insert((x, y));
                }
            }
            _ => continue,
        }

        // Top
        if let Some(y) = y.checked_sub(1) {
            to_visit.push_back((x, y));
        }

        // Right
        if x + 1 <= upscaled[0].len() - 1 {
            to_visit.push_back((x + 1, y));
        }

        // Bottom
        if y + 1 <= upscaled.len() - 1 {
            to_visit.push_back((x, y + 1));
        }

        // Left
        if let Some(x) = x.checked_sub(1) {
            to_visit.push_back((x, y))
        }
    }

    let total = maze.grid.len() * maze.grid[0].len();

    (total - visited_orig_positions.len()) as u32
}
