use std::collections::HashMap;

type Record = (String, Vec<usize>);
type Cache = HashMap<(usize, usize, usize), u64>;

fn get_arrangements(
    springs: &str,
    groups: &[usize],
    i: usize,
    group_i: usize,
    group_len: usize,
    cache: &mut Cache,
) -> u64 {
    if cache.contains_key(&(i, group_i, group_len)) {
        return cache.get(&(i, group_i, group_len)).unwrap().clone();
    }

    if i == springs.len() {
        if group_i == groups.len() && group_len == 0 {
            return 1;
        }
        if group_i == groups.len() - 1 && group_len == groups[group_i] {
            return 1;
        }
        return 0;
    }

    let mut arrangements = 0;
    let current_spring = springs.chars().nth(i).unwrap();

    // Either this is a '.' or we are pretending a '?' is a '.'
    if current_spring != '#' {
        if group_len > 0 && group_i < groups.len() && groups[group_i] == group_len {
            arrangements += get_arrangements(springs, groups, i + 1, group_i + 1, 0, cache);
        } else if group_len == 0 {
            arrangements += get_arrangements(springs, groups, i + 1, group_i, 0, cache);
        }
    }

    // Either this is a '#' or we are pretending a '?' is a '#'
    if current_spring != '.' {
        if group_i < groups.len() && group_len < groups[group_i] {
            arrangements += get_arrangements(springs, groups, i + 1, group_i, group_len + 1, cache);
        }
    }

    cache.insert((i, group_i, group_len), arrangements);

    arrangements
}

pub fn solve(input: &str) -> u64 {
    let records: Vec<Record> = input
        .lines()
        .map(|l| {
            l.split_once(' ')
                .map(|(springs, groups_str)| {
                    let springs = [springs, springs, springs, springs, springs].join("?");
                    let groups_str =
                        [groups_str, groups_str, groups_str, groups_str, groups_str].join(",");
                    let groups: Vec<usize> =
                        groups_str.split(',').map(|n| n.parse().unwrap()).collect();
                    (springs, groups)
                })
                .unwrap()
        })
        .collect();

    let mut cache: Cache = HashMap::new();

    records
        .iter()
        .map(|r| {
            cache.clear();
            get_arrangements(&r.0, &r.1, 0, 0, 0, &mut cache)
        })
        .sum()
}
