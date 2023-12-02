use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CubeColor {
    Red,
    Green,
    Blue,
}

impl CubeColor {
    pub fn parse(s: &str) -> Option<CubeColor> {
        match s {
            "red" => Some(CubeColor::Red),
            "green" => Some(CubeColor::Green),
            "blue" => Some(CubeColor::Blue),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct CubeSet {
    pub cubes: HashMap<CubeColor, usize>,
}

impl CubeSet {
    pub fn from_str(s: &str) -> CubeSet {
        let cubes = HashMap::from_iter(s.split(", ").filter_map(CubeSet::parse_entry));
        CubeSet { cubes }
    }

    fn parse_entry(s: &str) -> Option<(CubeColor, usize)> {
        let mut split_entry = s.split_whitespace();
        let maybe_number_of_cubes = split_entry
            .next()
            .and_then(|n_str| n_str.parse::<usize>().ok());
        let maybe_color = split_entry.next().and_then(CubeColor::parse);

        match (maybe_number_of_cubes, maybe_color) {
            (Some(number_of_cubes), Some(color)) => Some((color, number_of_cubes)),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct CubesGame {
    game_id: usize,
    cube_sets: Vec<CubeSet>,
}

impl CubesGame {
    pub fn parse(s: &str) -> Option<CubesGame> {
        let mut split_game = s.split(": ");
        let maybe_game_id = split_game.next().and_then(|s| {
            s.split_whitespace()
                .last()
                .and_then(|id_str| id_str.parse::<usize>().ok())
        });

        maybe_game_id.and_then(|game_id| match split_game.next() {
            Some(all_sets_str) => {
                let cube_sets: Vec<CubeSet> =
                    all_sets_str.split("; ").map(CubeSet::from_str).collect();
                Some(CubesGame { game_id, cube_sets })
            }
            _ => None,
        })
    }
}

pub fn find_possible_games(
    games_log: &str,
    max_red: usize,
    max_green: usize,
    max_blue: usize,
) -> usize {
    let games: Vec<CubesGame> = games_log.split("\n").filter_map(CubesGame::parse).collect();

    let valid_games = games.iter().filter(|game| {
        game.cube_sets.iter().all(|cube_set| {
            cube_set
                .cubes
                .iter()
                .all(|(color, num_of_cubes)| match color {
                    CubeColor::Red => num_of_cubes <= &max_red,
                    CubeColor::Green => num_of_cubes <= &max_green,
                    CubeColor::Blue => num_of_cubes <= &max_blue,
                })
        })
    });

    valid_games.fold(0, |acc, game| acc + game.game_id)
}
