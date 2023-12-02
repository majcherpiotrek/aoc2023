use regex::{Match, Regex, RegexSet};

pub fn calibrate_using_digits_only(calibration_file: &String) -> usize {
    calibration_file
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

pub fn calibrate_using_spelled_digits(calibration_file: &String) -> usize {
    calibration_file
        .split("\n")
        .map(|line| {
            let nums: Vec<usize> = find_candidates(&line)
                .iter()
                .map(|candidate| parse_num(candidate))
                .filter_map(|x| x)
                .collect();

            match (nums.first(), nums.last()) {
                (Some(first), Some(last)) => first * 10 + last,
                _ => 0,
            }
        })
        .fold(0, |acc, elem| acc + elem)
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
