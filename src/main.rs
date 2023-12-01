use std::env;
use std::fs;

fn read_number_arg(args: &Vec<String>, arg_name: &str) -> Option<usize> {
    let maybe_arg = args.iter().find(|arg| arg.starts_with(arg_name));
    maybe_arg.and_then(|arg| {
        let value_start = arg_name.len();
        let value_str = &arg[value_start..];
        value_str.parse::<usize>().ok()
    })
}

fn build_input_file_path(day: usize, part: usize) -> String {
    format!("./input_data/day_{}/part_{}", day, part)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let puzzle_identifier = (
        read_number_arg(&args, "--day="),
        read_number_arg(&args, "--part="),
    );
    match puzzle_identifier {
        (Some(day), Some(part)) => {
            println!("Selected day {}, part {}", day, part);
            let path = build_input_file_path(day, part);
            let contents =
                fs::read_to_string(path).expect("Should have been able to read the file");
            println!("With text:\n{contents}");
        }
        _ => println!("Please specify a day and part of the puzzle, e.g --day=1 --part=1"),
    }
}
