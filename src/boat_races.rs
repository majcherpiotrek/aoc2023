pub fn calculate_race_winning_margin(file: &str) -> usize {
    let race_stats = decode_race_stats_part_one(file);
    race_stats
        .into_iter()
        .fold(0, |acc, (race_time, record_distance)| {
            let winning_possibilities =
                get_all_possible_winning_charging_times(&race_time, &record_distance).len();
            if winning_possibilities > 0 {
                if acc == 0 {
                    winning_possibilities
                } else {
                    acc * winning_possibilities
                }
            } else {
                acc
            }
        })
}

fn decode_race_stats_part_one(file: &str) -> Vec<(usize, usize)> {
    let mut file_split = file.split("\n");
    let race_times = file_split.next().map(parse_line_part_one).unwrap_or(vec![]);
    let record_distances = file_split.next().map(parse_line_part_one).unwrap_or(vec![]);

    race_times
        .into_iter()
        .zip(record_distances.into_iter())
        .collect::<Vec<(usize, usize)>>()
}

fn parse_line_part_one(line: &str) -> Vec<usize> {
    line.split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<usize>>()
}

fn get_all_possible_winning_charging_times(
    race_time: &usize,
    record_distance: &usize,
) -> Vec<usize> {
    if *race_time < 2 {
        vec![]
    } else if *race_time == 2 {
        if *record_distance < 1 {
            vec![1]
        } else {
            vec![]
        }
    } else {
        let mut winning_charging_times = Vec::new();

        for candidate in 1..race_time - 1 {
            let distance_covered = calculate_distance(&race_time, &candidate);
            if distance_covered > *record_distance {
                winning_charging_times.push(candidate);
            }
        }

        winning_charging_times
    }
}

fn calculate_distance(race_time: &usize, charging_time: &usize) -> usize {
    if charging_time >= race_time {
        0
    } else {
        let travel_time = race_time - charging_time;
        travel_time * charging_time
    }
}
