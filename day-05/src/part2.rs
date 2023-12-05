#[derive(Debug)]
struct Range {
    dest_start: u64,
    src_start: u64,
    len: u64,
}

impl Range {
    fn convert(&self, num: u64) -> Option<u64> {
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

pub fn solve(input: String) -> u64 {
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();

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

    let mut lowest_location = u64::MAX;

    first_line
        .split(' ')
        .skip(1)
        .collect::<Vec<_>>()
        .chunks(2)
        .for_each(|pair| {
            let start = pair[0].parse::<u64>().unwrap();
            let len = pair[1].parse::<u64>().unwrap();

            for seed in start..start + len {
                let mut mseed = seed;

                for cat in categories.iter() {
                    'inner: for r in cat.ranges.iter() {
                        if let Some(t) = r.convert(mseed) {
                            mseed = t;
                            break 'inner;
                        }
                    }
                }

                if mseed < lowest_location {
                    lowest_location = mseed;
                }
            }
        });

    lowest_location
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::read_input;

    #[test]
    fn solve_input_1() {
        let input = read_input("part2_input1.txt");
        let solution = solve(input);

        println!("{solution}");
        assert_eq!(solution, 46);
    }

    #[test]
    fn solve_input() {
        let input = read_input("input.txt");
        let solution = solve(input);

        println!("Solution: {solution}");
        assert_eq!(solution, 125742456)
    }
}
