use std::{cmp, str::FromStr};

enum AocError {
    ParseGameError(String),
}

struct Game {
    id: i32,
    red_cubes: i32,
    green_cubes: i32,
    blue_cubes: i32,
}

impl FromStr for Game {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, AocError> {
        let parts = s.split(": ").collect::<Vec<&str>>();

        let game_id = parts
            .get(0)
            .ok_or(AocError::ParseGameError(String::from("Invalid input")))?
            .split(" ")
            .collect::<Vec<&str>>()
            .get(1)
            .ok_or(AocError::ParseGameError(String::from(
                "Game number not found",
            )))?
            .parse::<i32>()
            .map_err(|e| AocError::ParseGameError(format!("Error parsing game number: {}", e)))?;

        let mut red_cubes = 0;
        let mut green_cubes = 0;
        let mut blue_cubes = 0;

        parts
            .get(1)
            .ok_or(AocError::ParseGameError(String::from("Invalid input")))?
            .split("; ")
            .for_each(|set| {
                set.split(", ").for_each(|set_part| {
                    let cube_parts = set_part.split(" ").collect::<Vec<&str>>();

                    if let Ok(qnty) = cube_parts.get(0).unwrap_or(&"0").parse::<i32>() {
                        if let Some(color) = cube_parts.get(1) {
                            match *color {
                                "red" => red_cubes = cmp::max(red_cubes, qnty),
                                "green" => green_cubes = cmp::max(green_cubes, qnty),
                                "blue" => blue_cubes = cmp::max(blue_cubes, qnty),
                                _ => {}
                            }
                        }
                    }
                })
            });

        Ok(Game {
            id: game_id,
            red_cubes,
            green_cubes,
            blue_cubes,
        })
    }
}

fn get_power_sum(input: String) -> i32 {
    let games = input
        .lines()
        .filter_map(|line| Game::from_str(line).ok())
        .collect::<Vec<Game>>();

    let mut sum = 0;

    for game in games.iter() {
        sum += game.red_cubes * game.green_cubes * game.blue_cubes;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn read_input(path: &str) -> string {
        fs::read_to_string(path).expect(&format!("{path} file not found"))
    }

    #[test]
    fn solve_input_1() {
        let input = read_input("part2_input1.txt");
        let sum = get_power_sum(input);

        assert_eq!(sum, 2286);
    }

    #[test]
    fn solve_input() {
        let input = read_input("input.txt");
        let sum = get_power_sum(input);

        println!("sum: {sum}");
    }
}