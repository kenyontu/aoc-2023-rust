use std::collections::HashMap;

pub fn solve(input: &str) -> u32 {
    let mut lines = input.lines();
    let directions = lines.next().unwrap().chars().collect::<Vec<char>>();

    let map: HashMap<&str, (&str, &str)> = lines.skip(1).fold(HashMap::new(), |mut map, line| {
        let pos = &line[0..3];
        let l = &line[7..10];
        let r = &line[12..15];

        map.insert(pos, (&l, &r));
        map
    });

    let mut pos = "AAA";
    let mut steps = 0;

    for dir in directions.iter().cycle() {
        if pos == "ZZZ" {
            return steps;
        }

        if *dir == 'L' {
            pos = map[pos].0;
        } else {
            pos = map[pos].1;
        }
        steps += 1
    }

    steps
}
