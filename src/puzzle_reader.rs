use std::fs;

const DAY_ARG_PREFIX: &str = "--day=";
const PART_ARG_PREFIX: &str = "--part=";

pub struct Puzzle {
    pub identifier: PuzzleIdentifier,
    pub input_data: String,
}

pub struct PuzzleIdentifier {
    pub day: usize,
    pub part: usize,
}

pub fn read_puzzle(program_args: &Vec<String>) -> Result<Puzzle, String> {
    read_puzzle_identifier(program_args).and_then(|identifier| {
        read_puzzle_input_file(identifier.day, identifier.part).map(|input_data| Puzzle {
            identifier,
            input_data,
        })
    })
}

fn read_puzzle_identifier(program_args: &Vec<String>) -> Result<PuzzleIdentifier, String> {
    match (
        read_arg(&program_args, DAY_ARG_PREFIX),
        read_arg(&program_args, PART_ARG_PREFIX),
    ) {
        (Ok(day), Ok(part)) => Ok(PuzzleIdentifier { day, part }),
        (day_result, part_result) => {
            let day_error = get_error_string(&day_result);
            let space = if day_result.is_err() { ", " } else { "" };
            let part_error = get_error_string(&part_result);
            Err(format!("{:?}{}{:?}", day_error, space, part_error))
        }
    }
}

fn read_arg(args: &Vec<String>, arg_name: &str) -> Result<usize, String> {
    let maybe_arg = args
        .iter()
        .find(|arg| arg.starts_with(arg_name))
        .ok_or(format!("Missing argument: {arg_name}"));

    maybe_arg.and_then(|arg| {
        let value_start = arg_name.len();
        let value_str = &arg[value_start..];
        value_str.parse::<usize>().map_err(|e| {
            format!(
                "Failed to read argument value for '{arg_name}. Error: {:?}",
                e
            )
        })
    })
}

fn get_error_string<T>(res: &Result<T, String>) -> String {
    match res {
        Ok(_) => "".to_string(),
        Err(e) => e.to_string(),
    }
}

fn read_puzzle_input_file(day: usize, part: usize) -> Result<String, String> {
    let path = format!("./input_data/day_{}/part_{}", day, part);
    fs::read_to_string(path).map_err(|e| {
        format!(
            "Failed to read the puzzle input for day {day}, part {part}. Error was: {:?}",
            e
        )
    })
}
