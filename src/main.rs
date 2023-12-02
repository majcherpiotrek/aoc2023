use std::env;


pub mod puzzle_reader;
pub mod trebuchet;

use crate::puzzle_reader::{PuzzleIdentifier, read_puzzle};


fn main() {
    let args: Vec<String> = env::args().collect();
    
    match read_puzzle(&args) {
        Ok(puzzle) => {
            println!("Selected a puzzle for day {}, part {}", puzzle.identifier.day, puzzle.identifier.part);

            let program_result = match puzzle.identifier {
                PuzzleIdentifier { day: 1, part: 1 } => trebuchet::calibrate_using_digits_only(&puzzle.input_data).to_string(),
                PuzzleIdentifier { day: 1, part: 2 } => trebuchet::calibrate_using_spelled_digits(&puzzle.input_data).to_string(),
                _ => "Sorry, there is no solution for this puzzle yet ;(".to_string()
            };
            println!("{program_result}");
        },
        Err(e) => println!("Failed to load the puzzle: {}", e),
    }
}
