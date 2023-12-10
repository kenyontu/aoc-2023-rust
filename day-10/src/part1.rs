#[derive(Debug)]
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

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
struct Tile {
    pos: Pos,
    c: char,
}

impl Tile {
    fn new(x: usize, y: usize, c: char) -> Self {
        Self {
            pos: Pos { x, y },
            c,
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

    fn get_loop_length<'a>(&'a self, start_pos: &Pos) -> u32 {
        let mut prev = self.get_tile(start_pos).unwrap();
        let end = &get_pipe_ends(prev).unwrap()[0];
        let mut curr = self.get_tile(end).unwrap();

        let mut loop_length = 1;
        while &curr.pos != start_pos {
            let temp = curr;
            curr = next_pos(&self, &prev, &curr);
            prev = temp;
            loop_length += 1;
        }

        loop_length
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
    let connected_to = [
        start_pos.top(),
        start_pos.right(),
        start_pos.bottom(),
        start_pos.left(),
    ]
    .map(|pos| {
        println!("{:?}", pos);
        if let Some(ends) = pos
            .and_then(|pos| maze.get_tile(&pos))
            .and_then(|tile| get_pipe_ends(tile))
        {
            return ends.iter().any(|end| end == start_pos);
        };

        false
    });

    let pipe_type = match connected_to {
        [true, false, true, false] => '|',
        [false, true, false, true] => '-',
        [true, true, false, false] => 'L',
        [true, false, false, true] => 'J',
        [false, false, true, true] => '7',
        [false, true, true, false] => 'F',
        _ => unreachable!(),
    };

    Tile::new(start_pos.x, start_pos.y, pipe_type)
}

fn next_pos<'a>(maze: &'a Maze, prev_tile: &Tile, curr_tile: &Tile) -> &'a Tile {
    let [end1, end2] = get_pipe_ends(curr_tile).unwrap();
    if end1 != prev_tile.pos {
        maze.get_tile(&end1).unwrap()
    } else {
        maze.get_tile(&end2).unwrap()
    }
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

    let loop_length = maze.get_loop_length(&start_pos);

    loop_length / 2
}
