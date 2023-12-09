pub fn oasis_report(file: &str) -> i64 {
    let history_rows = file
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .flat_map(|s| s.parse::<i64>())
                .collect::<Vec<i64>>()
        })
        .filter(|row| !row.is_empty())
        .collect::<Vec<Vec<i64>>>();

    println!("Decoded rows");
    for row in history_rows.iter() {
        println!("{:?}", row);
    }
    println!("\n");

    history_rows.iter().fold(0, |acc, row| {
        let value_prediciton = extrapolate_value_prediction(row.to_vec());
        acc + value_prediciton
    })
}

fn extrapolate_value_prediction(values: Vec<i64>) -> i64 {
    let mut values_matrix = vec![values];
    let mut current_row_index = 0;
    loop {
        let current_row = values_matrix
            .get(current_row_index)
            .expect("Invalid values matrix");
        let mut next_row = Vec::new();
        println!("Row {}", current_row_index);
        for (i, elem) in current_row.iter().enumerate() {
            if i >= 1 {
                let diff = elem - current_row.get(i - 1).expect("Unexpected error");
                next_row.push(diff);
            }
        }
        let has_all_zeroes = next_row.iter().all(|elem| *elem == 0);
        println!("Pushing next row {:?}", next_row);
        values_matrix.push(next_row);
        if has_all_zeroes {
            break;
        } else {
            current_row_index += 1;
        }
    }

    for r in values_matrix.iter() {
        println!("{:?}", r);
    }

    println!("\nCalculating placeholders ... \n");
    let mut next_row_last_element: i64 = 0;
    for row_index in (0..=values_matrix.len() - 1).into_iter().rev() {
        let current_row = values_matrix.get_mut(row_index).expect("");
        println!("Current row: {:?}", current_row);
        let last_element_from_current_row = current_row.last().expect("Row must contain elements");
        println!(
            "{} + {}",
            last_element_from_current_row, next_row_last_element
        );
        next_row_last_element = last_element_from_current_row + next_row_last_element;
    }
    next_row_last_element
}
