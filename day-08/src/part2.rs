use std::collections::HashMap;

pub fn solve(input: &str) -> u128 {
    let mut lines = input.lines();
    let directions = lines.next().unwrap().chars().collect::<Vec<char>>();

    let mut starting_nodes: Vec<String> = Vec::new();

    let map: HashMap<String, (String, String)> =
        lines.skip(1).fold(HashMap::new(), |mut map, line| {
            let pos = line[0..3].to_string();
            let l = line[7..10].to_string();
            let r = line[12..15].to_string();

            if pos.chars().nth(2).unwrap() == 'A' {
                starting_nodes.push(pos.clone());
            }

            map.insert(pos, (l, r));
            map
        });

    // how many steps it takes for each node to reach a node ending with Z:
    let steps: Vec<u128> = starting_nodes
        .iter()
        .map(|node| {
            let mut count = 0;
            let mut n = node.clone();
            for dir in directions.iter().cycle() {
                if n.chars().nth(2).unwrap() == 'Z' {
                    return count;
                }

                let (l, r) = &map[&n];
                if *dir == 'R' {
                    n = r.clone();
                } else {
                    n = l.clone();
                }
                count += 1
            }
            0
        })
        .collect();

    let l = steps[steps.len() - 1];

    steps[0..steps.len() - 1]
        .iter()
        .rev()
        .fold(l, |n, next| num::integer::lcm(n, next.clone()))
}
