#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "3" => Direction::Up,
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            _ => unreachable!(),
        }
    }
}

pub fn solve(input: &str) -> u64 {
    let steps = input
        .lines()
        .map(|l| {
            let mut s = l.split_whitespace();

            let code = s.nth(2).unwrap();
            let meters = u64::from_str_radix(&code[2..7], 16).unwrap();
            let direction = Direction::from(&code[7..8]);

            (direction, meters)
        })
        .collect::<Vec<_>>();

    let mut vertices: Vec<(isize, isize)> = Vec::new();
    vertices.push((0, 0));

    let offsets: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut curr_pos = (0, 0);
    let mut border_sum: u64 = 0;

    // Dig around
    for step in steps.iter() {
        let offset_i: usize = match step.0 {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        };

        let offset = offsets[offset_i];

        let step_len = step.1;
        border_sum += step_len;

        curr_pos = (
            curr_pos.0 + (offset.0 * step_len as isize),
            curr_pos.1 + (offset.1 * step_len as isize),
        );
        vertices.push(curr_pos);
    }

    let mut sum: u64 = 0;

    // Calculating the area using the Shoelace algorithm
    for i in 0..vertices.len() - 1 {
        sum += vertices[i].0 as u64 * vertices[i + 1].1 as u64
            - vertices[i + 1].0 as u64 * vertices[i].1 as u64;
    }

    let area = (sum + vertices[vertices.len() - 1].0 as u64 * vertices[0].1 as u64)
        .abs_diff(vertices[0].0 as u64 * vertices[vertices.len() - 1].1 as u64)
        / 2;

    // The area obtained using Shoelace (952404941483) was still a bit off from the
    // example input, the difference was 3202632.
    // After a while I wondered if Shoelace included the borders so I added the border
    // length and got 952411346745, once again I checked the difference from the expected
    // result and got 3202630 which is only off 2 from the first difference (I've used google
    // to calculate it so I immediatelly saw that only the last digit changed).
    // So I added half of the border length and added 1 to match the expected result.
    // Tested it on the problem's input and it was correct.
    area + border_sum / 2 + 1
}
