use std::cmp;

/// Struct tuple representing the bag of cubes, contains the number of cubes for the
/// colors red, green and blue respectively.
struct Bag(u32, u32, u32);

impl Bag {
    fn can_have_game(&self, game: &Game) -> bool {
        game.red <= self.0 && game.green <= self.1 && game.blue <= self.2
    }
}

struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

fn get_possible_games_sum(input: String, bag: Bag) -> u32 {
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
                    "red" => red = cmp::max(red, count),
                    "green" => green = cmp::max(green, count),
                    "blue" => blue = cmp::max(blue, count),
                    _ => unreachable!(),
                }
            });

            let game = Game {
                id: game_id,
                red,
                green,
                blue,
            };

            game_id += 1;

            game
        })
        .collect::<Vec<Game>>();

    let mut sum = 0;

    for game in games.iter() {
        if bag.can_have_game(game) {
            sum += game.id;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn read_input(path: &str) -> String {
        fs::read_to_string(path).expect(&format!("{path} file not found"))
    }

    #[test]
    fn solve_input_1() {
        let input = read_input("part1_input1.txt");

        let sum = get_possible_games_sum(input, Bag(12, 13, 14));

        assert_eq!(sum, 8);
    }

    #[test]
    fn solve_input() {
        let input = read_input("input.txt");

        let sum = get_possible_games_sum(input, Bag(12, 13, 14));

        println!("Sum: {sum}");
        assert_eq!(sum, 2541)
    }
}
