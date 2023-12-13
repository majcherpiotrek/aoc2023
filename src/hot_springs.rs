struct Row(Vec<SpringState>, Vec<usize>);

impl Row {
    pub fn parse(line: &str) -> Row {
        let mut line_split = line.split_whitespace();
        let springs = line_split
            .next()
            .map(|s| {
                s.chars()
                    .map(|c| SpringState::from_char(&c))
                    .collect::<Vec<SpringState>>()
            })
            .expect("Invalid input");

        let groups = line_split
            .next()
            .map(|s| {
                s.split(",")
                    .map(|n| n.parse::<usize>().expect("Invalid input"))
                    .collect::<Vec<usize>>()
            })
            .expect("Invalid input");

        Row(springs, groups)
    }

    pub fn to_tuple(&self) -> (&Vec<SpringState>, &Vec<usize>) {
        (&self.0, &self.1)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SpringState {
    Operational,
    Broken,
    Unknown,
}

impl SpringState {
    pub fn from_char(c: &char) -> SpringState {
        match c {
            '.' => SpringState::Operational,
            '#' => SpringState::Broken,
            '?' => SpringState::Unknown,
            _ => panic!("Invalid spring state"),
        }
    }
}

impl std::fmt::Display for SpringState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SpringState::Broken => "#",
            SpringState::Unknown => "?",
            SpringState::Operational => ".",
        })
    }
}

pub fn sum_possible_arrangements(file: &str) -> usize {
    let springs_records = parse_input(file);
    let mut sum: usize = 0;
    for (i, record) in springs_records.iter().enumerate() {
        println!(
            "\n\nRecord {i}, broken blocks: {}\n\n",
            record
                .1
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
        println!("Unfiltered");
        println!(
            "{}",
            record
                .0
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("")
        );

        let arrangements = find_possibilities_per_block(record);

        for (block_num, arrangements_for_block) in arrangements.iter().enumerate() {
            println!(
                "Block num: {block_num} - size {}",
                record.1.get(block_num).expect("This value must be there")
            );
            for arr in arrangements_for_block.iter() {
                let (start, end) = arr;
                let springs = &record.0;
                let head = &springs[0..*start]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join("");
                let placement = (0..(end - start))
                    .map(|_| "X")
                    .collect::<Vec<&str>>()
                    .join("");
                let tail = &springs[*end..]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join("");

                println!("{head}{placement}{tail}");
            }
            println!("\n");
        }
        let possibilities = find_all(&record.0, &Vec::new(), &arrangements);
        for (n, p) in possibilities.iter().enumerate() {
            for arr in p.iter() {
                let (start, end) = arr;
                let springs = &record.0;
                let head = &springs[0..*start]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join("");
                let placement = (0..(end - start))
                    .map(|_| "X")
                    .collect::<Vec<&str>>()
                    .join("");
                let tail = &springs[*end..]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join("");

                println!("{}: {head}{placement}{tail}", n + 1);
            }
        }
        sum += possibilities.len();
    }

    sum
}

fn find_possibilities_per_block(row: &Row) -> Vec<Vec<(usize, usize)>> {
    let (springs, broken_blocks_sizes) = row.to_tuple();

    let possible_arrangements_per_block: Vec<Vec<(usize, usize)>> = broken_blocks_sizes
        .iter()
        .enumerate()
        .map(|(i, size)| {
            //println!("Block {i}, size {size}");
            let mut possible_positions: Vec<(usize, usize)> = Vec::new();

            let mut lowest_possible_slice_start = i + &broken_blocks_sizes[0..i]
                .iter()
                .fold(0, |acc, size| acc + size);
            //println!("Lowest possible start: {lowest_possible_slice_start}");

            loop {
                if lowest_possible_slice_start > springs.len() - size {
                    panic!("This should never happen, impossible block");
                }

                let maybe_next_spring = springs.get(lowest_possible_slice_start + size);

                let is_succeeded_by_broken_spring = maybe_next_spring
                    .map(|s| *s == SpringState::Broken)
                    .unwrap_or(false);

                if lowest_possible_slice_start == 0 {
                    if is_succeeded_by_broken_spring {
                        lowest_possible_slice_start += 1;
                        continue;
                    } else {
                        break;
                    }
                }

                let prev_spring = springs
                    .get(lowest_possible_slice_start - 1)
                    .expect("This should always be within bounds");

                let is_preceeded_by_broken_spring = *prev_spring == SpringState::Broken;

                if is_preceeded_by_broken_spring || is_succeeded_by_broken_spring {
                    lowest_possible_slice_start += 1;
                    continue;
                }

                break;
            }

            let highest_possible_end = springs.len()
                - &broken_blocks_sizes[(i + 1)..]
                    .iter()
                    .fold(0, |acc, size| acc + size);

            for start in lowest_possible_slice_start..=(highest_possible_end - size) {
                let end: usize = start + size;
                let slice = &springs[start..end];

                let are_all_springs_broken_or_unknown = slice
                    .iter()
                    .all(|s| *s == SpringState::Unknown || *s == SpringState::Broken);

                if are_all_springs_broken_or_unknown {
                    possible_positions.push((start, end));
                }
            }

            possible_positions
        })
        .collect();

    possible_arrangements_per_block
}

fn find_all(
    springs: &Vec<SpringState>,
    head: &Vec<(usize, usize)>,
    tail_possibilities: &Vec<Vec<(usize, usize)>>,
) -> Vec<Vec<(usize, usize)>> {
    if tail_possibilities.is_empty() {
        let is_valid_combination = springs.iter().enumerate().all(|(i, spring)| match spring {
            SpringState::Broken => head.iter().any(|(start, end)| i >= *start && i < *end),
            _ => true,
        });
        return if is_valid_combination {
            vec![head.clone()]
        } else {
            vec![]
        };
    }

    match tail_possibilities.first() {
        None => vec![head.clone()],
        Some(next_level_possibilities) => {
            let valid_possibilities = match head.last() {
                None => next_level_possibilities
                    .iter()
                    .collect::<Vec<&(usize, usize)>>(),
                Some(last_block) => next_level_possibilities
                    .iter()
                    .filter(|block| block.0 > last_block.1)
                    .collect::<Vec<&(usize, usize)>>(),
            };

            let mut all_possibilities: Vec<Vec<(usize, usize)>> = Vec::new();
            for possibility in valid_possibilities.into_iter() {
                let mut new_head = head.clone();
                new_head.push(*possibility);
                let following_levels = &tail_possibilities[1..].to_vec();
                let mut following_possibilities = find_all(springs, &new_head, following_levels);
                all_possibilities.append(&mut following_possibilities);
            }
            all_possibilities
        }
    }
}

fn parse_input(file: &str) -> Vec<Row> {
    file.split("\n")
        .filter(|line| !line.is_empty())
        .map(Row::parse)
        .collect::<Vec<Row>>()
}