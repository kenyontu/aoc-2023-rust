use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn excluding(&self, other: &Range) -> Option<Vec<Range>> {
        let mut ranges = Vec::new();

        if self.start < other.start {
            ranges.push(Range {
                start: self.start,
                end: other.start - 1,
            });
        }

        if self.end > other.end {
            ranges.push(Range {
                start: other.end,
                end: self.end,
            })
        }

        (ranges.len() > 0).then(|| ranges)
    }
}

#[derive(Debug)]
struct CatRange {
    dest_start: u32,
    src_start: u32,
    len: u32,
}

impl CatRange {
    fn overlapping_range(&self, range: &Range) -> Option<Range> {
        let start = u32::max(range.start, self.src_start);
        let end = u32::min(range.end, self.src_start + self.len);

        (end > start && (start >= self.src_start || end <= self.src_start + self.len))
            .then(|| Range { start, end })
    }

    fn convert_range(&self, range: &Range) -> Range {
        let start = self.dest_start + range.start - self.src_start;
        let end = start + range.end - range.start;
        Range { start, end }
    }
}

#[derive(Debug)]
struct Category {
    ranges: Vec<CatRange>,
}

pub fn solve(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut categories: Vec<Category> = Vec::new();

    let first_line = lines.next().unwrap();

    // parse ranges
    let mut ranges: VecDeque<Range> = first_line
        .split(' ')
        .skip(1)
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|pair| {
            let start = pair[0].parse::<u32>().unwrap();
            let len = pair[1].parse::<u32>().unwrap();

            Range {
                start,
                end: start + len,
            }
        })
        .collect();

    // parse categories
    for line in lines {
        if line.is_empty() {
            continue;
        }

        if line.chars().last().unwrap() == ':' {
            categories.push(Category { ranges: Vec::new() });
            continue;
        }

        let s = line.split(' ').collect::<Vec<_>>();

        categories.last_mut().unwrap().ranges.push(CatRange {
            dest_start: s[0].parse().unwrap(),
            src_start: s[1].parse().unwrap(),
            len: s[2].parse().unwrap(),
        });
    }

    for cat in categories.iter() {
        let mut new_ranges: VecDeque<Range> = VecDeque::new();

        'outer: while !ranges.is_empty() {
            let r = ranges.pop_back().unwrap();

            for cr in cat.ranges.iter() {
                if let Some(overlapping) = cr.overlapping_range(&r) {
                    match r.excluding(&overlapping) {
                        Some(excluded_ranges) => ranges.extend(excluded_ranges),
                        _ => {}
                    }

                    let converted = cr.convert_range(&overlapping);
                    new_ranges.push_back(converted);

                    continue 'outer;
                }
            }

            new_ranges.push_back(r);
        }

        ranges = new_ranges;
    }

    ranges.iter().map(|range| range.start).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_range() {
        let cr = CatRange {
            dest_start: 52,
            src_start: 50,
            len: 48,
        };

        assert_eq!(
            cr.overlapping_range(&Range { start: 79, end: 93 }),
            Some(Range { start: 79, end: 93 })
        );

        assert_eq!(
            cr.overlapping_range(&Range {
                start: 79,
                end: 120
            }),
            Some(Range { start: 79, end: 98 })
        );

        assert_eq!(
            cr.overlapping_range(&Range { start: 40, end: 60 }),
            Some(Range { start: 50, end: 60 })
        );

        assert_eq!(
            cr.overlapping_range(&Range { start: 50, end: 98 }),
            Some(Range { start: 50, end: 98 })
        );

        assert_eq!(cr.overlapping_range(&Range { start: 10, end: 20 }), None);
        assert_eq!(
            cr.overlapping_range(&Range {
                start: 99,
                end: 100
            }),
            None
        );
    }

    #[test]
    fn convert_range() {
        let cr = CatRange {
            dest_start: 52,
            src_start: 50,
            len: 48,
        };

        assert_eq!(
            cr.convert_range(&Range { start: 51, end: 91 }),
            Range { start: 53, end: 93 }
        );
    }
}
