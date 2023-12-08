use std::collections::HashMap;

pub fn solve(input: &str) -> u32 {
    let mut lines = input.lines();
    let directions = lines.next().unwrap().chars().collect::<Vec<char>>();

    let map: HashMap<String, (String, String)> =
        lines.skip(1).fold(HashMap::new(), |mut map, line| {
            let pos = line[0..3].to_string();
            let l = line[7..10].to_string();
            let r = line[12..15].to_string();

            map.insert(pos, (l, r));
            map
        });

    let mut pos = String::from("AAA");
    let mut steps = 0;

    for dir in directions.iter().cycle() {
        if pos == "ZZZ" {
            return steps;
        }

        let (l, r) = &map[&pos];
        if *dir == 'R' {
            pos = r.clone();
        } else {
            pos = l.clone();
        }
        steps += 1
    }

    steps
}
