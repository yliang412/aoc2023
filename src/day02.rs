//! https://adventofcode.com/2023/day/2

use std::u32;

use crate::util::readlines;


#[derive(Default, Debug, PartialEq)]
struct Game {
    pub id: u32,
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl Game {
    pub fn new(id: u32) -> Self {
        Game { id, ..Default::default() }
    }
}

/// Parses a subset of cubes
fn parse_subset(game: &mut Game, s: &str) {
    s.split(", ").into_iter().for_each(|s| {
        let (n, color) = s.split_once(" ").expect("error parse subset");
        let n = n.parse::<usize>().expect("error parse cube count");
        match color {
            "red" => game.red = game.red.max(n),
            "blue" => game.blue =game.blue.max(n),
            "green" => game.green = game.green.max(n),
            _ => unreachable!()
        }
    })
}

/// Parses the game.
/// e.g. `Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue`
fn parse_game(text: String) -> Option<Game> {
    let (id, subsets) = text.split_once(": ")?;
    let id = id.replace("Game ", "").parse::<u32>().ok()?;
    let mut game = Game::new(id);
    subsets.split("; ").into_iter().for_each(|s| parse_subset(&mut game, s));
    Some(game)
}

pub fn cube_conundrum(filename: &str, red: usize, green: usize, blue: usize) -> Result<(u32, usize), &'static str> {
    let lines = readlines(filename).expect("Error reading file");
    let mut id_sum = 0;
    let mut power_sum = 0;
    for line in lines {
        if let Ok(text) = line {
            let game = parse_game(text).expect("error parsing the game");
            if game.red <= red && game.blue <= blue && game.green <= green {
                id_sum += game.id;
            }
            power_sum += game.red * game.blue * game.green;
        }
    }
    Ok((id_sum, power_sum))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02_parse_game() {
        let result = parse_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string());
        assert_eq!(result, Some(Game { id: 2, red: 1, blue: 4, green: 3}));
    }

    #[test]
    fn test_day2_example() {
        let sum = cube_conundrum("data/day02.example", 12, 13, 14);
        assert_eq!(sum, Ok((8, 2286)));
    }

    #[test]
    fn test_day2_input() {
        let sum = cube_conundrum("data/day02.input", 12, 13, 14);
        assert_eq!(sum, Ok((2406, 78375)));
    }
}