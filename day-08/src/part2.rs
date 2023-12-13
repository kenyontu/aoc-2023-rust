use std::collections::HashMap;

fn gcd(a: &u64, b: &u64) -> u64 {
    let mut a = a.clone();
    let mut b = b.clone();

    while b != 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }

    a
}

fn lcm(a: &u64, b: &u64) -> u64 {
    (a / gcd(a, b)) * b
}

pub fn solve(input: &str) -> u64 {
    let mut lines = input.lines();
    let directions = lines.next().unwrap().chars().collect::<Vec<char>>();

    let mut starting_nodes: Vec<&str> = Vec::new();

    let map: HashMap<&str, (&str, &str)> = lines.skip(1).fold(HashMap::new(), |mut map, line| {
        let pos = &line[0..3];
        let l = &line[7..10];
        let r = &line[12..15];

        if pos.ends_with('A') {
            starting_nodes.push(pos);
        }

        map.insert(pos, (l, r));
        map
    });

    // how many steps it takes for each starting node to reach a node ending with Z:
    let steps_to_z_by_node: Vec<u64> = starting_nodes
        .iter()
        .map(|node| {
            let mut count = 0;
            let mut n = *node;
            for dir in directions.iter().cycle() {
                if n.ends_with('Z') {
                    return count;
                }

                if *dir == 'L' {
                    n = map[n].0;
                } else {
                    n = map[n].1;
                }
                count += 1
            }
            0
        })
        .collect();

    steps_to_z_by_node
        .iter()
        .copied()
        .reduce(|acc, e| lcm(&acc, &e))
        .unwrap()
}
