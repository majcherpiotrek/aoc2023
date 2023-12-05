use std::env;


mod puzzle_reader;
mod trebuchet;
mod cube_conundrum;
mod gear_ratios;
mod scratchcards;
mod garden;

use crate::puzzle_reader::{PuzzleIdentifier, read_puzzle};


fn main() {
    let args: Vec<String> = env::args().collect();
    
    match read_puzzle(&args) {
        Ok(puzzle) => {
            println!("Selected a puzzle for day {}, part {}", puzzle.identifier.day, puzzle.identifier.part);

            let program_result = match puzzle.identifier {
                PuzzleIdentifier { day: 1, part: 1 } => trebuchet::calibrate_using_digits_only(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 1, part: 2 } => trebuchet::calibrate_using_spelled_digits(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 2, part: 1 } => cube_conundrum::find_possible_games(&puzzle.input_data, 12, 13, 14).to_string(),
                PuzzleIdentifier { day: 2, part: 2 } => cube_conundrum::power_of_minimal_possible_games(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 3, part: 1 } => gear_ratios::count_engine_parts(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 3, part: 2 } => gear_ratios::count_gear_ratio(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 4, part: 1 } => scratchcards::sum_scratchcard_points(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 4, part: 2 } => scratchcards::process_scratchcards(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 5, part: 1 } => garden::read_almanac(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 5, part: 2 } => garden::read_almanac(&puzzle.input_data).to_string(),
                _ => "Sorry, there is no solution for this puzzle yet ;(".to_string()
            };
            println!("{program_result}");
        },
        Err(e) => println!("Failed to load the puzzle: {}", e),
    }
}
