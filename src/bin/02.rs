advent_of_code::solution!(2);

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref GAME_REGEX: Regex = Regex::new(r"(\d+) (blue|red|green)").unwrap();
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct Game {
    red: u32,
    blue: u32,
    green: u32,
}

impl Game {
    pub fn power(&self) -> u32 {
        &self.blue * &self.red * &self.green
    }

    pub fn max(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            blue: self.blue.max(other.blue),
            green: self.green.max(other.green),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let all_games = input.lines().map(parse_line);
    Some(
        all_games
            .enumerate()
            .filter_map(|(_, (game_number, sets))| {
                if sets.iter().enumerate().all(|(_set_number, g)| {
                    if g.red > max_red {
                        // println!("Game {game_number}: Set {_set_number}.red = {}", g.red);
                        return false;
                    }
                    if g.blue > max_blue {
                        // println!("Game {game_number}: Set {_set_number}.blue = {}", g.blue);
                        return false;
                    }
                    if g.green > max_green {
                        // println!("Game {game_number}: Set {_set_number}.green = {}", g.green);
                        return false;
                    }
                    true
                }) {
                    // println!("Game {}: all good", game_number);
                    Some(game_number)
                } else {
                    None
                }
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let all_games = input.lines().map(parse_line);

    Some(
        all_games
            .map(|(_game_number, sets)| {
                let max = sets.iter().fold(
                    Game {
                        ..Default::default()
                    },
                    |acc, g| acc.max(g),
                );
                // println!("Game {}: {:?} ({})", _game_number, max, max.power());
                max
            })
            .map(|g| g.power())
            .sum::<u32>(),
    )
}

fn parse_line(input: &str) -> (u32, Vec<Game>) {
    let (game_number, sets) = input.split_once(':').unwrap();

    let game_number = game_number
        .split_once(" ")
        .expect("Game number not in correct format")
        .1
        .parse::<u32>()
        .unwrap();
    let sets = sets.split(';').map(parse_game).collect_vec();
    (game_number, sets)
}

fn parse_game(input: &str) -> Game {
    GAME_REGEX.captures_iter(input).fold(
        Game {
            ..Default::default()
        },
        |mut acc, captures| {
            let num = captures[1].parse::<u32>().unwrap();
            match &captures[2] {
                "red" => acc.red += num,
                "blue" => acc.blue += num,
                "green" => acc.green += num,
                _ => println!("Unknown color: {}", &captures[2]),
            }
            acc
        },
    )
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2286));
    }

    #[test]
    fn test_parse_game() {
        assert_eq!(
            parse_game("1 red"),
            Game {
                red: 1,
                ..Default::default()
            }
        );
        assert_eq!(
            parse_game("1 red, 2 blue, 13 green"),
            Game {
                red: 1,
                blue: 2,
                green: 13
            }
        );
    }
    #[test]
    fn test_parse_line() {
        assert_equal(
            parse_line("Game 1: 1 red; 2 blue,3 red; 15 green,183 blue")
                .1
                .iter(),
            vec![
                Game {
                    red: 1,
                    ..Default::default()
                },
                Game {
                    red: 3,
                    blue: 2,
                    ..Default::default()
                },
                Game {
                    green: 15,
                    blue: 183,
                    ..Default::default()
                },
            ]
            .iter(),
        );
    }
}
