pub fn sum_scratchcard_points(numbers_table: &str) -> usize {
    numbers_table
        .split("\n")
        .filter_map(|line| {
            line.split(": ").nth(1).map(|numbers| {
                let mut numbers_split = numbers.split(" | ");

                let winning_numbers: Vec<usize> = numbers_split
                    .next()
                    .iter()
                    .flat_map(|nums_str| {
                        let nums: Vec<usize> = nums_str
                            .split_whitespace()
                            .filter_map(|n_str| n_str.parse::<usize>().ok())
                            .collect();
                        nums
                    })
                    .collect();

                let elfs_numbers: Vec<usize> = numbers_split
                    .next()
                    .iter()
                    .flat_map(|nums_str| {
                        let nums: Vec<usize> = nums_str
                            .split_whitespace()
                            .filter_map(|n_str| n_str.parse::<usize>().ok())
                            .collect();
                        nums
                    })
                    .collect();

                elfs_numbers.into_iter().fold(0, |acc, num| {
                    let is_winning = winning_numbers.contains(&num);

                    if is_winning {
                        if acc == 0 {
                            1
                        } else {
                            acc * 2
                        }
                    } else {
                        acc
                    }
                })
            })
        })
        .fold(0, |sum, line_points| sum + line_points)
}
