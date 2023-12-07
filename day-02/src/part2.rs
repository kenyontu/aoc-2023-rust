struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn solve(input: &str) -> u32 {
    let mut game_id = 1;

    let games = input
        .lines()
        .map(|line| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            line.split(&[':', ';', ',']).skip(1).for_each(|cube| {
                let mut cube_parts = cube.split(' ').skip(1);
                let count = cube_parts
                    .next()
                    .expect(&format!("Error getting cube count from {cube}"))
                    .parse::<u32>()
                    .expect(&format!("Error parsing cube count from {cube}"));

                let color = cube_parts
                    .next()
                    .expect(&format!("Error getting cube color from {cube}"));

                match color {
                    "red" => red = u32::max(red, count),
                    "green" => green = u32::max(green, count),
                    "blue" => blue = u32::max(blue, count),
                    _ => unreachable!(),
                }
            });

            let game = Game { red, green, blue };

            game_id += 1;

            game
        })
        .collect::<Vec<Game>>();

    games.iter().map(|g| g.red * g.green * g.blue).sum()
}
