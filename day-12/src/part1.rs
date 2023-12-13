use std::collections::VecDeque;

type Record<'a> = (&'a str, Vec<usize>);

fn get_arrangements(springs: &str, groups: &[usize]) -> u32 {
    // spring index, group index, length of current group of #'s
    let mut stack: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut arrangements = 0;

    let springs: Vec<char> = springs.chars().collect();

    stack.push_back((0, 0, 0));

    while stack.len() > 0 {
        let (i, group_i, group_len) = stack.pop_back().unwrap();

        // If at the last position
        if i == springs.len() {
            if groups.len() == group_i && group_len == 0 {
                arrangements += 1;
            } else if groups.len() - 1 == group_i && groups[group_i] == group_len {
                arrangements += 1;
            }

            continue;
        }

        // Either this is a '.' or we are pretending a '?' is a '.'
        if springs[i] != '#' {
            if group_len > 0 && group_i < groups.len() && groups[group_i] == group_len {
                stack.push_back((i + 1, group_i + 1, 0));
            } else if group_len == 0 {
                stack.push_back((i + 1, group_i, 0));
            }
        }

        // Either this is a '#' or we are pretending a '?' is a '#'
        if springs[i] != '.' {
            if group_i < groups.len() && group_len < groups[group_i] {
                stack.push_back((i + 1, group_i, group_len + 1));
            }
        }
    }

    arrangements
}

pub fn solve(input: &str) -> u32 {
    let records: Vec<Record> = input
        .lines()
        .map(|l| {
            l.split_once(' ')
                .map(|(springs, groups_str)| {
                    let groups: Vec<usize> =
                        groups_str.split(',').map(|n| n.parse().unwrap()).collect();
                    (springs, groups)
                })
                .unwrap()
        })
        .collect();

    records
        .iter()
        .map(|r| {
            let count = get_arrangements(&r.0, &r.1);
            count
        })
        .sum()
}
