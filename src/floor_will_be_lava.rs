use std::collections::HashMap;

pub fn calculate_energized_tiles(file: &str) -> usize {
    let lines = parse_input(file);
    let width = lines.first().unwrap().len();

    let mut energization_map = vec![vec![0 as u8; width]; lines.len()];

    let mut cache: HashMap<String, BeamDirection> = HashMap::new();

    follow_beam(&lines, &mut energization_map, (0, 0), BeamDirection::Right, &mut cache);

    for row in energization_map.iter() {
        println!("{:?}", row);
    }

    
    energization_map.iter().fold(0, |acc, row| {
        acc + row.iter().fold(0, |acc_2, tile| acc_2 + *tile as usize)
    })
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum BeamDirection {
    Up,
    Right,
    Down,
    Left,
}

fn follow_beam(
    tiles_schema: &Vec<Vec<char>>,
    energization_map: &mut Vec<Vec<u8>>,
    beam_start: (usize, usize),
    beam_direction: BeamDirection,
    visited_tiles: &mut HashMap<String, BeamDirection>,
) -> () {
    println!("\nStarting new beam");
    let mut current_tile_index = beam_start;
    let mut current_beam_direction = beam_direction;
    loop {
        let tile_key = build_key(&current_tile_index);
        let maybe_visited = visited_tiles.get(&tile_key);

        if maybe_visited.is_some() && *maybe_visited.unwrap() == current_beam_direction {
            break;
        } else {
            visited_tiles.insert(tile_key, current_beam_direction);
        }
        let current_tile = tiles_schema
            .get(current_tile_index.0)
            .and_then(|row| row.get(current_tile_index.1));
        println!("Current tile: {:?}", current_tile);
        println!("Current tile index: {:?}", current_tile_index);
        println!("Current direction: {:?}", current_beam_direction);

        let next_tile_index = match current_tile {
            None => None,
            Some(c) => {
                println!("Updating map: {:?}", current_tile_index);
                energization_map[current_tile_index.0][current_tile_index.1] = 1;

                match c {
                    '.' => get_next_tile(&current_tile_index, &current_beam_direction),
                    '/' => match current_beam_direction {
                        BeamDirection::Up => {
                            current_beam_direction = BeamDirection::Right;
                            get_next_tile(&current_tile_index, &current_beam_direction)
                        }
                        BeamDirection::Right => {
                            if current_tile_index.0 > 0 {
                                current_beam_direction = BeamDirection::Up;
                                get_next_tile(&current_tile_index, &current_beam_direction)
                            } else {
                                None
                            }
                        }
                        BeamDirection::Down => {
                            if current_tile_index.1 > 0 {
                                current_beam_direction = BeamDirection::Left;
                                get_next_tile(&current_tile_index, &current_beam_direction)
                            } else {
                                None
                            }
                        }
                        BeamDirection::Left => {
                            current_beam_direction = BeamDirection::Down;
                            get_next_tile(&current_tile_index, &current_beam_direction)
                        }
                    },
                    '\\' => match current_beam_direction {
                        BeamDirection::Up => {
                            if current_tile_index.1 > 0 {
                                current_beam_direction = BeamDirection::Left;
                                get_next_tile(&current_tile_index, &current_beam_direction)
                            } else {
                                None
                            }
                        }
                        BeamDirection::Right => {
                            current_beam_direction = BeamDirection::Down;
                            get_next_tile(&current_tile_index, &current_beam_direction)
                        }
                        BeamDirection::Down => {
                            current_beam_direction = BeamDirection::Right;
                            get_next_tile(&current_tile_index, &current_beam_direction)
                        }
                        BeamDirection::Left => {
                            if current_tile_index.0 > 0 {
                                current_beam_direction = BeamDirection::Up;
                                get_next_tile(&current_tile_index, &current_beam_direction)
                            } else {
                                None
                            }
                        }
                    },
                    '-' => match current_beam_direction {
                        BeamDirection::Left | BeamDirection::Right => {
                            get_next_tile(&current_tile_index, &current_beam_direction)
                        }
                        _ => {
                            follow_beam(
                                tiles_schema,
                                energization_map,
                                current_tile_index,
                                BeamDirection::Right,
                                visited_tiles,
                            );
                            follow_beam(
                                tiles_schema,
                                energization_map,
                                current_tile_index,
                                BeamDirection::Left,
                                visited_tiles,
                            );
                            None
                        }
                    },
                    '|' => match current_beam_direction {
                        BeamDirection::Up | BeamDirection::Down => {
                            get_next_tile(&current_tile_index, &current_beam_direction)
                        }
                        _ => {
                            follow_beam(
                                tiles_schema,
                                energization_map,
                                current_tile_index,
                                BeamDirection::Down,
                                visited_tiles,
                            );
                            follow_beam(
                                tiles_schema,
                                energization_map,
                                current_tile_index,
                                BeamDirection::Up,
                                visited_tiles,
                            );
                            None
                        }
                    },
                    _ => panic!("Invalid tile"),
                }
            }
        };

        if next_tile_index.is_none() {
            println!("Split the beam\n");
            break;
        } else {
            println!("Continue the beam!");
            current_tile_index = next_tile_index.unwrap();
            println!("updated index: {:?}", current_tile_index);
            println!("updated direction: {:?}", current_beam_direction);
            println!("");
        }
    }
}

fn build_key(tile_index: &(usize, usize)) -> String {
    format!("{}-{}", tile_index.0, tile_index.1)
}

fn get_next_tile(
    current: &(usize, usize),
    beam_direction: &BeamDirection,
) -> Option<(usize, usize)> {
    let (row_index, column_index) = current;
    match beam_direction {
        BeamDirection::Up => {
            if *row_index > 0 {
                Some((row_index - 1, *column_index))
            } else {
                None
            }
        }
        BeamDirection::Right => Some((*row_index, *column_index + 1)),
        BeamDirection::Down => Some((*row_index + 1, *column_index)),
        BeamDirection::Left => {
            if *column_index > 0 {
                Some((*row_index, *column_index - 1))
            } else {
                None
            }
        }
    }
}

fn parse_input(file: &str) -> Vec<Vec<char>> {
    file.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}
