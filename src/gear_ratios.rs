use regex::{Match, Regex};

pub fn count_engine_parts(engine_schematic: &str) -> usize {
    let number_regex = Regex::new(r"\b\d+\b").expect("Invalid regex");
    let symbol_regex = Regex::new(r"[^\d^\.^\n]").expect("Invalid regex");


    let parts: Vec<(Vec<Match>, Vec<Match>)> = engine_schematic.split("\n").into_iter().map(|line| {

      let number_matches = number_regex.captures_iter(line).filter_map(|cap| cap.get(0)).collect();
      let symbol_matches = symbol_regex.captures_iter(line).filter_map(|cap| cap.get(0)).collect();

      (number_matches, symbol_matches)

    }).collect();

    let mut sum: usize = 0;

    for (line_number, (number_matches, symbol_matches)) in parts.iter().enumerate() {
      let empty_symbols = Vec::new();
      let prev_line_symbols = if line_number >= 1 { parts.get(line_number - 1).map(|(_, s)| s).unwrap_or(&empty_symbols) } else { &empty_symbols }; 
      let next_line_symbols = parts.get(line_number + 1).map(|(_, s)| s).unwrap_or(&empty_symbols); 
      
       for number_match in number_matches.iter() {
         let is_engine_part = has_adjacent_symbols(number_match, prev_line_symbols, symbol_matches, next_line_symbols);
         
            if is_engine_part {
                let part_number = number_match.as_str().parse::<usize>().unwrap_or(0);
                sum += part_number
            }
       }
    }
    
    sum
}

fn has_adjacent_symbols(number_match: &Match, prev_line_symbols: &Vec<Match>, this_line_symbols: &Vec<Match>, next_line_symbols: &Vec<Match>) -> bool {
  prev_line_symbols.iter().any(|symbol_match| is_adjacent(number_match, symbol_match)) ||
  this_line_symbols.iter().any(|symbol_match| is_adjacent(number_match, symbol_match)) ||
  next_line_symbols.iter().any(|symbol_match| is_adjacent(number_match, symbol_match)) 
}

fn is_adjacent(number_match: &Match, symbol_match: &Match) -> bool {
    let left_boundary = if number_match.start() >= 1 { number_match.start() - 1 } else { number_match.start() };
  symbol_match.start() >= left_boundary && symbol_match.start() <= number_match.end()
}
