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

fn day_one_part_one(file_content: &String) -> usize {
    file_content.split("\n").map(|line| {
        let nums: Vec<usize> = line.chars().map(|c| {
            c.to_digit(10).and_then(|digit| 
                digit.try_into().ok()
            )
        }).filter_map(|x| x).collect();
        
        match (nums.first(), nums.last()) {
            (Some(first), Some(last)) => first * 10 + last,
            _ => 0
        }
    }).fold(0, |acc, elem| acc + elem)
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
            println!("Result: {}", day_one_part_one(&contents));
        }
        _ => println!("Please specify a day and part of the puzzle, e.g --day=1 --part=1"),
    }
}
