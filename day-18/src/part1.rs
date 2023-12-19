use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => unreachable!(),
        }
    }
}

pub fn solve(input: &str) -> u32 {
    let steps = input
        .lines()
        .map(|l| {
            let mut s = l.split_whitespace();

            let direction = Direction::from(s.next().unwrap());
            let meters = s.next().unwrap().parse::<u32>().unwrap();
            let color = s.next().unwrap();
            let color = &color[1..color.len() - 1];

            (direction, meters, color)
        })
        .collect::<Vec<_>>();

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    visited.insert((0, 0));

    // Allows us to know the limits of the area
    let mut min_x = isize::MAX;
    let mut min_y = isize::MAX;
    let mut max_x = isize::MIN;
    let mut max_y = isize::MIN;

    let offsets = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut curr_pos = (0, 0);

    // Dig around
    for step in steps.iter() {
        let offset_i = match step.0 {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        };

        let offset = offsets[offset_i];

        for _ in 0..step.1 {
            curr_pos = (curr_pos.0 + offset.0, curr_pos.1 + offset.1);
            visited.insert(curr_pos);

            min_x = isize::min(min_x, curr_pos.0);
            min_y = isize::min(min_y, curr_pos.1);
            max_x = isize::max(max_x, curr_pos.0);
            max_y = isize::max(max_y, curr_pos.1);
        }
    }

    // Incrase the area by 1 in each direction so when flooding
    // we don't get blocked by any wall
    min_x -= 1;
    min_y -= 1;
    max_x += 1;
    max_y += 1;

    let mut queue: VecDeque<(isize, isize)> = VecDeque::new();
    queue.push_back((min_x, min_y));
    visited.insert((min_x, min_y));

    let mut outside_count: u32 = 0;

    while let Some(curr_pos) = queue.pop_back() {
        outside_count += 1;
        offsets
            .iter()
            .map(|(x, y)| (curr_pos.0 + x, curr_pos.1 + y))
            .for_each(|(x, y)| {
                if x < min_x || x > max_x {
                    return;
                }

                if y < min_y || y > max_y {
                    return;
                }

                if visited.contains(&(x, y)) {
                    return;
                }

                visited.insert((x, y));
                queue.push_back((x, y));
            });
    }

    let height = min_y.abs_diff(max_y) + 1;
    let width = min_x.abs_diff(max_x) + 1;

    let total_area = height * width;

    total_area as u32 - outside_count
}
