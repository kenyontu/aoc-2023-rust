#[derive(Debug)]
struct Range {
    dest_start: u32,
    src_start: u32,
    len: u32,
}

impl Range {
    fn convert(&self, num: u32) -> Option<u32> {
        if num >= self.src_start && num < self.src_start + self.len {
            Some(num - self.src_start + self.dest_start)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Category {
    ranges: Vec<Range>,
}

pub fn solve(input: &String) -> u32 {
    let mut lines = input.lines();

    let mut seeds: Vec<u32> = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|seed| seed.parse::<u32>().unwrap())
        .collect();

    let mut categories: Vec<Category> = Vec::new();
    lines.for_each(|line| {
        if line.is_empty() {
            return;
        }

        if line.chars().last().unwrap() == ':' {
            categories.push(Category { ranges: Vec::new() });
            return;
        }

        let mut s = line.split(' ');

        categories.last_mut().unwrap().ranges.push(Range {
            dest_start: s.next().unwrap().parse().unwrap(),
            src_start: s.next().unwrap().parse().unwrap(),
            len: s.next().unwrap().parse().unwrap(),
        });
    });

    let mut lowest_location = u32::MAX;

    categories.iter().for_each(|cat| {
        seeds = seeds
            .iter()
            .map(|s| {
                for r in cat.ranges.iter() {
                    if let Some(n) = r.convert(*s) {
                        return n;
                    }
                }
                *s
            })
            .collect();
    });

    for seed in seeds.iter() {
        if seed < &lowest_location {
            lowest_location = *seed;
        }
    }

    lowest_location
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::read_input;

    #[test]
    fn solve_input_1() {
        let input = read_input("part1_input1.txt");
        let solution = solve(input);

        println!("{solution}");
        assert_eq!(solution, 35);
    }

    #[test]
    fn solve_input() {
        let input = read_input("input.txt");
        let solution = solve(input);

        println!("Solution: {solution}");
        assert_eq!(solution, 196167384)
    }
}
