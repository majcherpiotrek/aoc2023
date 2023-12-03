use regex::{Match, Regex};

const NUMBER_REGEX_PATTERN: &str = r"\b\d+\b";
const SYMBOL_REGEX_PATTERN: &str = r"[^\d^\.^\n]";
const INVALID_REGEX_PATTERN: &str = "Invalid regex pattern";

pub fn count_engine_parts(engine_schematic: &str) -> usize {
    let mut sum: usize = 0;
    let parts = get_engine_parts(engine_schematic);

    for (line_number, (number_matches, symbol_matches)) in parts.iter().enumerate() {
        let empty_symbols = Vec::new();
        let prev_line_symbols = if line_number >= 1 {
            parts
                .get(line_number - 1)
                .map(|(_, s)| s)
                .unwrap_or(&empty_symbols)
        } else {
            &empty_symbols
        };
        let next_line_symbols = parts
            .get(line_number + 1)
            .map(|(_, s)| s)
            .unwrap_or(&empty_symbols);

        for number_match in number_matches.iter() {
            let is_engine_part = !find_all_adjacent_matches(
                number_match,
                prev_line_symbols,
                symbol_matches,
                next_line_symbols,
            )
            .is_empty();

            if is_engine_part {
                let part_number = number_match.as_str().parse::<usize>().unwrap_or(0);
                sum += part_number
            }
        }
    }

    sum
}

pub fn count_gear_ratio(engine_schematic: &str) -> usize {
    let mut gear_ratios_sum: usize = 0;
    let parts = get_engine_parts(engine_schematic);

    for (line_number, (number_matches, symbol_matches)) in parts.iter().enumerate() {
        let empty_vector = Vec::new();
        let prev_line_numbers = if line_number >= 1 {
            parts
                .get(line_number - 1)
                .map(|(n, _)| n)
                .unwrap_or(&empty_vector)
        } else {
            &empty_vector
        };
        let next_line_numbers = parts
            .get(line_number + 1)
            .map(|(n, _)| n)
            .unwrap_or(&empty_vector);

        for symbol_match in symbol_matches.iter() {
            let adjacent_matches = find_all_adjacent_matches(
                symbol_match,
                prev_line_numbers,
                number_matches,
                next_line_numbers,
            );

            let adjacent_numbers: Vec<usize> = adjacent_matches
                .iter()
                .filter_map(|m| m.as_str().parse::<usize>().ok())
                .collect();

            let gear_ratio: usize = match &adjacent_numbers[..] {
                [a, b] => a * b,
                _ => 0,
            };

            gear_ratios_sum += gear_ratio;
        }
    }
    gear_ratios_sum
}

fn get_engine_parts(engine_schematic: &str) -> Vec<(Vec<Match>, Vec<Match>)> {
    let number_regex = Regex::new(NUMBER_REGEX_PATTERN).expect(INVALID_REGEX_PATTERN);
    let symbol_regex = Regex::new(SYMBOL_REGEX_PATTERN).expect(INVALID_REGEX_PATTERN);

    engine_schematic
        .split("\n")
        .into_iter()
        .map(|line| {
            let number_matches = number_regex
                .captures_iter(line)
                .filter_map(|cap| cap.get(0))
                .collect();
            let symbol_matches = symbol_regex
                .captures_iter(line)
                .filter_map(|cap| cap.get(0))
                .collect();

            (number_matches, symbol_matches)
        })
        .collect()
}

fn find_all_adjacent_matches<'a>(
    target_match: &Match,
    prev_line_matches: &Vec<Match<'a>>,
    this_line_matches: &Vec<Match<'a>>,
    next_line_matches: &Vec<Match<'a>>,
) -> Vec<Match<'a>> {
    vec![
        find_adjacent_matches(target_match, prev_line_matches),
        find_adjacent_matches(target_match, this_line_matches),
        find_adjacent_matches(target_match, next_line_matches),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<Match>>()
}

fn find_adjacent_matches<'a>(target_match: &Match, matches: &Vec<Match<'a>>) -> Vec<Match<'a>> {
    matches
        .iter()
        .filter(|m| are_matches_adjacent(target_match, m))
        .copied()
        .collect::<Vec<Match>>()
}

fn are_matches_adjacent(match_a: &Match, match_b: &Match) -> bool {
    // ...AAA...
    // .BBXXXXXX
    match_b.start() <= match_a.start() && match_b.end() >= match_a.start() ||
    // ...AAA...
    // ....BXXXX
    match_b.start() >= match_a.start() && match_b.start() <= match_a.end()
}
