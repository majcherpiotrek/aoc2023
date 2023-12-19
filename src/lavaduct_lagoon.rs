pub fn calculate_lagoon_surface(file: &str) -> i64 {
    let dig_plan = file
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(DigPlanEntry::parse_part_one)
        .collect::<Vec<DigPlanEntry>>();

    let contour_length: i64 = dig_plan.iter().fold(0 as i64, |acc, entry| 
      acc + i64::try_from(entry.steps).unwrap()
    );

    let corners = get_all_corners(&dig_plan);

    // Wtf, Pick's theorem says it should be -1 but for some reason it works for +1 ¯\_(ツ)_/¯
    // Need to do the abs because the surface might be negative if the corners ordered clockwise
    calculate_surface(&corners).abs() + contour_length / 2 + 1
}

pub fn calculate_lagoon_surface_part_two(file: &str) -> i64 {
    let dig_plan = file
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(DigPlanEntry::parse_part_two)
        .collect::<Vec<DigPlanEntry>>();

    let contour_length: i64 = dig_plan.iter().fold(0 as i64, |acc, entry| 
      acc + i64::try_from(entry.steps).unwrap()
    );

    let corners = get_all_corners(&dig_plan);

    // Wtf, Pick's theorem says it should be -1 but for some reason it works for +1 ¯\_(ツ)_/¯
    // Need to do the abs because the surface might be negative if the corners ordered clockwise
    calculate_surface(&corners).abs() + contour_length / 2 + 1
}

// Using the shoelace formula
fn calculate_surface(corners: &Vec<(i64, i64)>) -> i64 {
    let mut sum = 0;

    for i in 0..corners.len() {
        let j = i + 1;

        let (x_i, y_i) = corners[i];

        let (x_j, y_j) = if j < corners.len() {
            corners[j]
        } else {
            corners[0]
        };

        sum += (x_i * y_j) - (x_j * y_i)
    }

    sum / 2
}

fn get_all_corners(dig_plan: &Vec<DigPlanEntry>) -> Vec<(i64, i64)> {
    let mut corners: Vec<(i64, i64)> = Vec::new();
    let mut previous_direction = dig_plan.last().unwrap().direction;
    let mut current_position = (0, 0);
    for entry in dig_plan.iter() {
        let steps = i64::try_from(entry.steps).unwrap();

        if entry.direction != previous_direction {
            corners.push(current_position.clone());
        }
        previous_direction = entry.direction;
        current_position = match entry.direction {
            Direction::Up => (current_position.0, current_position.1 + steps),
            Direction::Right => (current_position.0 + steps, current_position.1),
            Direction::Down => (current_position.0, current_position.1 - steps),
            Direction::Left => (current_position.0 - steps, current_position.1),
        }
    }

    corners
}

struct DigPlanEntry {
    direction: Direction,
    steps: usize,
}

impl DigPlanEntry {
    pub fn parse_part_one(str: &str) -> DigPlanEntry {
        let mut line_split = str.split_whitespace();
        let direction = line_split.next().map(Direction::parse).unwrap();
        let steps = line_split
            .next()
            .and_then(|n| n.parse::<usize>().ok())
            .unwrap();

        DigPlanEntry {
            direction,
            steps,
        }
    }

    pub fn parse_part_two(str: &str) -> DigPlanEntry {
        str.split_whitespace().nth(2).map(|hex| {
            let encoded_distance = &hex[2..hex.len() - 2];
            let encoded_direction = &hex[hex.len() - 2..hex.len() -1];

            let steps = encoded_distance.chars().rev().enumerate().fold(0, |acc, (i, hex_digit)| {
                let pow: usize = (16 as usize).pow(i as u32);
                let digit_value_decimal = usize::from_str_radix(&hex_digit.to_string(), 16).unwrap();
                
                acc + pow * digit_value_decimal
            });

            let direction = match encoded_direction {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => panic!("Invalid direction encoding")
            };

            DigPlanEntry {
                direction,
steps
            }

        }).unwrap()

    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn parse(str: &str) -> Direction {
        match str {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => panic!("Invalid direction: {str}"),
        }
    }
}
