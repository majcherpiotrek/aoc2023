use regex::{Match, Regex, RegexSet};
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

fn find_candidates(str: &str) -> Vec<&str> {
    let patterns = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", r"\d",
    ];
    let regex_set = RegexSet::new(patterns).expect("Incorrect regex patterns");
    let regexes: Vec<Regex> = regex_set
        .patterns()
        .iter()
        .map(|p| Regex::new(p).expect(format!("Failed to build regex for pattern {p}").as_str()))
        .collect();

    let mut matches = regex_set
        .matches(str)
        .into_iter()
        .filter_map(|index| regexes.get(index))
        .flat_map(|re| re.find_iter(str).collect::<Vec<Match>>())
        .collect::<Vec<Match>>();

    matches.sort_by(|a, b| a.start().partial_cmp(&b.start()).unwrap());

    matches.iter().map(|m| &str[m.start()..m.end()]).collect()
}

fn day_one_part_one(file_content: &String) -> usize {
    file_content
        .split("\n")
        .map(|line| {
            let nums: Vec<usize> = line
                .chars()
                .map(|c| c.to_digit(10).and_then(|digit| digit.try_into().ok()))
                .filter_map(|x| x)
                .collect();

            match (nums.first(), nums.last()) {
                (Some(first), Some(last)) => first * 10 + last,
                _ => 0,
            }
        })
        .fold(0, |acc, elem| acc + elem)
}

fn parse_num(str: &str) -> Option<usize> {
    match str {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        other => other.parse::<usize>().ok(),
    }
}

fn day_one_part_two(file_content: &String) -> usize {
    file_content
        .split("\n")
        .map(|line| {
            let nums: Vec<usize> = find_candidates(&line)
                .iter()
                .map(|candidate| parse_num(candidate))
                .filter_map(|x| x)
                .collect();

            println!("LINE {line}: {:?}", nums);

            match (nums.first(), nums.last()) {
                (Some(first), Some(last)) => {
                    let sum = first * 10 + last;
                    println!("SUM {first} + {last} = {sum}");
                    sum
                }
                _ => 0,
            }
        })
        .fold(0, |acc, elem| acc + elem)
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
            //println!("Result: {}", day_one_part_one(&contents));
            println!("Result: {}", day_one_part_two(&contents));
        }
        _ => println!("Please specify a day and part of the puzzle, e.g --day=1 --part=1"),
    }
}
