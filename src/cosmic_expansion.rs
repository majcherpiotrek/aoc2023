use std::collections::HashMap;

pub fn calculate_distances_between_galaxies(file: &str) -> usize {
    let galaxies_map = parse_galaxies_map(file);

    for row in galaxies_map.iter() {
        println!(
            "{}",
            row.iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }

    let mut galaxy_index = 1;
    let mut galaxies_hashmap: HashMap<usize, (usize, usize)> = HashMap::new();

    for (row_index, row) in galaxies_map.iter().enumerate() {
        for (col, element) in row.iter().enumerate() {
            if *element == '#' {
                galaxies_hashmap.insert(galaxy_index, (row_index, col));
                galaxy_index += 1;
            }
        }
    }
 
    let mut distance_sum: usize = 0;
    for galaxy_a in 1..galaxy_index {
        for galaxy_b in 1..galaxy_index {
            if galaxy_a == galaxy_b { continue; }
            let (galaxy_a_row, galaxy_a_column) = galaxies_hashmap.get(&galaxy_a).expect("Galaxy coordinates must exist in hashmap");
            let (galaxy_b_row, galaxy_b_column) = galaxies_hashmap.get(&galaxy_b).expect("Galaxy coordinates must exist in hashmap");

            let row_distance = if galaxy_a_row > galaxy_b_row { galaxy_a_row - galaxy_b_row } else { galaxy_b_row - galaxy_a_row };
            let column_distance = if galaxy_a_column > galaxy_b_column { galaxy_a_column - galaxy_b_column } else { galaxy_b_column - galaxy_a_column };

            let distance = row_distance + column_distance;
            distance_sum += distance
        }
    }
    
    distance_sum / 2
}

fn parse_galaxies_map(file: &str) -> Vec<Vec<char>> {
    let matrix = file
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .filter(|row| !row.is_empty())
        .collect::<Vec<Vec<char>>>();

    println!("Raw map:\n");
    for row in matrix.iter() {
        println!(
            "{}",
            row.iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }

    let mut matrix_expanded_rows: Vec<Vec<char>> = Vec::new();

    for row in matrix.iter() {
        matrix_expanded_rows.push(row.clone());
        if !row.contains(&'#') {
            matrix_expanded_rows.push(row.clone());
        }
    }

    println!("Expanded rows map:\n");
    for row in matrix_expanded_rows.iter() {
        println!(
            "{}",
            row.iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }

    let mut matrix_expanded_columns: Vec<Vec<char>> = matrix_expanded_rows.clone();

    let matrix_width = matrix_expanded_rows
        .first()
        .map(|row| row.len())
        .expect("Matrix must not be empty");
    let matrix_height = matrix_expanded_rows.len();

    println!(
        "\nDimensions: width: {}, height: {}",
        matrix_width, matrix_height
    );

    let mut added_columns: usize = 0;
    for col in 0..matrix_width {
        println!("Col: {col}\n");
        let column_contains_galaxies = (0..matrix_height).into_iter().any(|row_index| {
            println!("  Row: {row_index}");
            let matrix_element = matrix_expanded_rows
                .get(row_index)
                .and_then(|row| row.get(col))
                .expect("Element must exist in matrix");

            *matrix_element == '#'
        });

        if !column_contains_galaxies {
            println!("Empty column!");
            let insert_index = col + added_columns;
            println!("Added columns: {added_columns}, insert index: {insert_index}");
            for row_index in 0..matrix_height {
                println!("    Inserting in row: {row_index}");
                let row = matrix_expanded_columns
                    .get_mut(row_index)
                    .expect("Row must exist in matrix");
                println!("Row to be updated: {:?}", row);

                row.insert(insert_index, '.');
            }

            added_columns += 1;
        }

        println!("\n");
    }

    matrix_expanded_columns
}
