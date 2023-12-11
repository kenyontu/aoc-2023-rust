use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Pos { x, y }
    }
}

#[derive(Debug)]
struct Galaxy {
    pos: Pos,
}

fn calc_shortest_dist(pos1: &Pos, pos2: &Pos) -> usize {
    pos1.x.abs_diff(pos2.x) + pos1.y.abs_diff(pos2.y)
}

pub fn solve(input: &str) -> u64 {
    let rows_count = input.lines().count();
    let column_count = input.lines().next().unwrap().len();

    let mut galaxies_by_row: HashMap<usize, usize> = HashMap::new();
    let mut galaxies_by_column: HashMap<usize, usize> = HashMap::new();

    let mut galaxies: Vec<Galaxy> =
        input
            .lines()
            .enumerate()
            .fold(Vec::new(), |mut acc, (y, line)| {
                line.chars().enumerate().for_each(|(x, c)| {
                    if c == '#' {
                        acc.push(Galaxy {
                            pos: Pos::new(x, y),
                        });
                        galaxies_by_row
                            .entry(y)
                            .and_modify(|count| *count = *count + 1)
                            .or_insert(1);

                        galaxies_by_column
                            .entry(x)
                            .and_modify(|count| *count = *count + 1)
                            .or_insert(1);
                    }
                });

                acc
            });

    let rows_to_expand: HashSet<usize> = (0..rows_count)
        .filter(|r| !galaxies_by_row.contains_key(&r))
        .collect();

    let cols_to_expand: HashSet<usize> = (0..column_count)
        .filter(|c| !galaxies_by_column.contains_key(&c))
        .collect();

    galaxies.iter_mut().for_each(|g| {
        let mut add_x = cols_to_expand.iter().filter(|c| g.pos.x > **c).count();
        let mut add_y = rows_to_expand.iter().filter(|c| g.pos.y > **c).count();

        let times_larger = 1_000_000;
        add_x = (add_x * times_larger) - add_x;
        add_y = (add_y * times_larger) - add_y;

        g.pos.x += add_x;
        g.pos.y += add_y;
    });

    let mut pairs = 0;
    let sum: u64 = (0..galaxies.len())
        .map(|i| {
            (i + 1..galaxies.len())
                .map(|j| {
                    pairs += 1;
                    calc_shortest_dist(&galaxies[i].pos, &galaxies[j].pos) as u64
                })
                .sum::<u64>()
        })
        .sum();

    sum as u64
}
