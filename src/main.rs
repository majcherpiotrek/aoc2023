use std::env;


mod puzzle_reader;
mod trebuchet;
mod cube_conundrum;
mod gear_ratios;
mod scratchcards;
mod garden;
mod boat_races;
mod camel_cards;
mod haunted_wasteland;
mod mirage_maintenance;
mod pipe_maze;
mod cosmic_expansion;
mod hot_springs;
mod lens_library;

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
                PuzzleIdentifier { day: 5, part: 1 } => garden::read_almanac_seed_by_seed(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 5, part: 2 } => garden::read_almanac_by_seed_ranges(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 6, part: 1 } => boat_races::calculate_race_winning_margin(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 6, part: 2 } => boat_races::calculate_winning_possibilities(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 7, part: 1 } => camel_cards::calculate_total_winning(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 7, part: 2 } => camel_cards::calculate_total_winning_with_jokers(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 8, part: 1 } => haunted_wasteland::find_way(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 8, part: 2 } => haunted_wasteland::find_way_ghosts(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 9, part: 1 } => mirage_maintenance::oasis_report(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 9, part: 2 } => mirage_maintenance::oasis_report_backwards(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 10, part: 1 } => pipe_maze::steps_to_farthest_loop_end(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 10, part: 2 } => pipe_maze::surface_inside_loop(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 11, part: 1 } => cosmic_expansion::calculate_distances_between_galaxies(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 12, part: 1 } => hot_springs::calculate_arrangements(&puzzle.input_data, false).to_string(),
                PuzzleIdentifier { day: 12, part: 2 } => hot_springs::calculate_arrangements(&puzzle.input_data, true).to_string(),
                PuzzleIdentifier { day: 15, part: 1 } => lens_library::calculate_hash_for_sequence(&puzzle.input_data).to_string(),
                _ => "Sorry, there is no solution for this puzzle yet ;(".to_string()
            };
            println!("{program_result}");
        },
        Err(e) => println!("Failed to load the puzzle: {}", e),
    }
}
