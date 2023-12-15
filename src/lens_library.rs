fn parse_input(file: &str) -> Vec<&str> {
    file.trim().split(",").collect::<Vec<&str>>()
}

fn calculate_hash(word: &str) -> u16 {
    let ascii_codes = word.chars().map(|c| c as u8).collect::<Vec<u8>>();
    let multiplier: u16 = 17;
    let divider: u16 = 256;
    
    ascii_codes.iter().fold(0 as u16, |acc, code| {
        let increased = acc + *code as u16;
        let multiplied = increased * multiplier;
        multiplied % divider
    })
}

pub fn calculate_hash_for_sequence(file: &str) -> usize {
    let sequence = parse_input(file);

    sequence.iter().fold(0 as usize, |acc, elem| {
        let hash = calculate_hash(elem);
        acc + hash as usize
    })
}
