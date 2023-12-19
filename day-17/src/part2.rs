use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    None,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    priority: i64,
    pos: Pos,
    direction: Direction,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_next_positions(
    pos: &Pos,
    dir: &Direction,
    width: usize,
    height: usize,
) -> Vec<(Pos, Direction)> {
    let mut positions = Vec::new();

    if dir == &Direction::Up || dir == &Direction::Down || dir == &Direction::None {
        for m in 4..11 {
            if let Some(x) = pos.x.checked_sub(m) {
                positions.push((Pos::new(x, pos.y), Direction::Left));
            };

            if pos.x + m < width {
                positions.push((Pos::new(pos.x + m, pos.y), Direction::Right));
            }
        }
    }

    if dir == &Direction::Left || dir == &Direction::Right || dir == &Direction::None {
        for m in 4..11 {
            if let Some(y) = pos.y.checked_sub(m) {
                positions.push((Pos::new(pos.x, y), Direction::Up));
            };

            if pos.y + m < height {
                positions.push((Pos::new(pos.x, pos.y + m), Direction::Down));
            }
        }
    }

    positions
}

pub fn solve(input: &str) -> i64 {
    let height = input.lines().count();
    let width = input.lines().nth(0).unwrap().len();

    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - 48).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let heuristic = |pos: &Pos| -> usize { pos.x.abs_diff(width - 1) + pos.y.abs_diff(height - 1) };

    let mut frontier: BinaryHeap<Node> = BinaryHeap::new();
    frontier.push(Node {
        priority: 0,
        pos: Pos::new(0, 0),
        direction: Direction::None,
    });

    let mut cost: HashMap<(Pos, Direction), i64> = HashMap::new();
    cost.insert((Pos::new(0, 0), Direction::None), 0);

    while let Some(node) = frontier.pop() {
        if node.pos.x == width - 1 && node.pos.y == height - 1 {
            break;
        }

        let next_positions = get_next_positions(&node.pos, &node.direction, width, height);

        for (pos, dir) in next_positions.iter() {
            let mut cost_so_far = *cost
                .get(&(node.pos.clone(), node.direction.clone()))
                .unwrap();

            // Get positions between node.pos and pos
            if pos.x < node.pos.x {
                for x in pos.x + 1..node.pos.x {
                    cost_so_far += grid[pos.y][x] as i64;
                }
            } else if pos.x > node.pos.x {
                for x in node.pos.x + 1..pos.x {
                    cost_so_far += grid[pos.y][x] as i64;
                }
            }

            if pos.y < node.pos.y {
                for y in pos.y + 1..node.pos.y {
                    cost_so_far += grid[y][pos.x] as i64;
                }
            } else if pos.y > node.pos.y {
                for y in node.pos.y + 1..pos.y {
                    cost_so_far += grid[y][pos.x] as i64;
                }
            }

            let new_cost = cost_so_far + grid[pos.y][pos.x] as i64;
            let prev_cost = cost.get(&(pos.clone(), dir.clone())).unwrap_or(&i64::MAX);

            if new_cost < *prev_cost {
                cost.insert((pos.clone(), dir.clone()), new_cost);

                frontier.push(Node {
                    // Rust's BinaryHeap is a max heap, to make it act as a min heap, we use
                    // negative numbers
                    priority: (new_cost + heuristic(&pos) as i64) as i64 * -1,
                    pos: pos.clone(),
                    direction: dir.clone(),
                });
            }
        }
    }

    *cost
        .iter()
        .filter(|(k, _)| k.0.x == width - 1 && k.0.y == height - 1)
        .map(|(_, v)| v)
        .min()
        .unwrap()
}
