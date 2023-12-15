fn parse_input(file: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    file.split("\n\n")
        .filter(|b| !b.is_empty())
        .map(|block| parse_pattern(block))
        .collect::<Vec<(Vec<u32>, Vec<u32>)>>()
}

fn parse_pattern(block: &str) -> (Vec<u32>, Vec<u32>) {
    let pattern_rows = block
        .split("\n")
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    let rows = pattern_rows
        .iter()
        .map(|line| line_to_number(line))
        .collect::<Vec<u32>>();
    let mut columns: Vec<u32> = Vec::new();

    let width = pattern_rows.first().expect("Block must not be empty").len();

    for col in 0..width {
        let mut column_str: Vec<&str> = Vec::new();

        for row in pattern_rows.iter() {
            column_str.push(&row[col..(col + 1)]);
        }

        let column_as_number = line_to_number(&column_str.join(""));
        columns.push(column_as_number);
    }

    (rows, columns)
}

fn line_to_number(line: &str) -> u32 {
    let mut result: u32 = 0;

    for c in line.chars() {
        match c {
            '.' => result = result << 1,
            '#' => result = (result << 1) | 1,
            _ => panic!("Invalid character!"),
        };
    }

    result
}

fn find_point_of_reflection(lines: &Vec<u32>) -> Option<usize> {
    let mut potential_reflection_point: usize = 1;

    loop {
        if potential_reflection_point >= lines.len() {
            break None;
        }

        let mut i: usize = 1;
        let mut is_reflection = false;

        loop {
            let right_index = potential_reflection_point + i - 1;
            if i > potential_reflection_point || right_index >= lines.len() {
                break;
            }
            let left_index = potential_reflection_point - i;

            let left = lines[left_index];
            let right = lines[right_index];

            if left == right {
                is_reflection = true;
                i += 1;
            } else {
                is_reflection = false;
                break;
            }
        }

        if is_reflection {
            break Some(potential_reflection_point);
        } else {
            potential_reflection_point += 1;
        }
    }
}

pub fn count_reflections(file: &str) -> usize {
    let patterns = parse_input(file);
    let mut sum: usize = 0;

    for p in patterns.iter() {
        let (rows, columns) = p;
        let point_of_reflection_horizontal = find_point_of_reflection(rows);
        let point_of_reflection_vertical = find_point_of_reflection(columns);

        let pattern_sum = match (point_of_reflection_vertical, point_of_reflection_horizontal) {
            (Some(vertical_reflection_point), None) => vertical_reflection_point,
            (None, Some(horizontal_reflection)) => 100 * (horizontal_reflection),
            (Some(vertical_reflection_point), Some(horizontal_reflection)) => {
                vertical_reflection_point + 100 * horizontal_reflection
            }
            _ => 0,
        };
        println!("Pattern sum: {pattern_sum}");
        sum += pattern_sum;
        println!("Horizontal: {:?}", point_of_reflection_horizontal);
        println!("Rows: {:?}\n", rows);
        println!("Vertical: {:?}", point_of_reflection_vertical);
        println!("Columns: {:?}", columns);
        println!("\n\n");
    }
    sum
}
