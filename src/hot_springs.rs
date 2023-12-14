use std::collections::HashMap;

pub fn parse(line: &str, unfold_records: bool) -> (String, Vec<usize>) {
    let mut line_split = line.split_whitespace();
    let springs = line_split
        .next()
        .map(|s| {
            if unfold_records {
                (0..5)
                    .map(|_| s.to_string())
                    .collect::<Vec<String>>()
                    .join("?")
            } else {
                s.to_string()
            }
        })
        .expect("Invalid input");

    let groups = line_split
        .next()
        .map(|s| {
            let groups = if unfold_records {
                (0..5)
                    .map(|_| s.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            } else {
                s.to_string()
            };
            groups
                .split(",")
                .map(|n| n.parse::<usize>().expect("Invalid input"))
                .collect::<Vec<usize>>()
        })
        .expect("Invalid input");

    (springs, groups)
}

pub fn calculate_arrangements(file: &str, unfold_records: bool) -> usize {
    let records = parse_input(file, unfold_records);
    records.iter().fold(0, |acc, record| {
        let (springs, groups) = record;
        let mut cache: HashMap<String, usize> = HashMap::new();
        let arrangements = solve(springs, groups, 0, &mut cache);
        acc + arrangements
    })
}

fn parse_input(file: &str, unfold_records: bool) -> Vec<(String, Vec<usize>)> {
    file.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| parse(line, unfold_records))
        .collect::<Vec<(String, Vec<usize>)>>()
}

fn get_cache_key(springs: &str, groups: &[usize], group_counter: usize) -> String {
    format!("{}-{:?}-{}", springs, groups, group_counter)
}

fn solve(
    springs: &str,
    groups: &[usize],
    group_counter: usize,
    cache: &mut HashMap<String, usize>,
) -> usize {
    let cache_key = get_cache_key(springs, groups, group_counter);

    if let Some(cached_result) = cache.get(&cache_key) {
        return *cached_result;
    }

    if springs.is_empty() {
        if groups.is_empty() {
            cache.insert(cache_key, 1);
            return 1;
        } else if groups.len() == 1 && group_counter == groups[0] {
            cache.insert(cache_key, 1);
            return 1;
        } else {
            cache.insert(cache_key, 0);
            return 0;
        }
    }

    let current_spring = springs.chars().next().expect("This should never be empty");

    if groups.is_empty() && springs.chars().all(|c| c == '.' || c == '?') {
        cache.insert(cache_key, 1);
        return 1;
    } else if groups.is_empty() {
        cache.insert(cache_key, 0);
        return 0;
    }


    match current_spring {
        '.' => {
            let current_group = groups.first().expect("Groups cannot be empty here");
            if *current_group == group_counter {
                solve(&springs[1..], &groups[1..], 0, cache)
            } else if group_counter == 0 {
                solve(&springs[1..], groups, 0, cache)
            } else {
                cache.insert(cache_key, 0);
                0
            }
        }
        '#' => {
            let updated_counter = group_counter + 1;
            let current_group = groups.first().expect("Groups cannot be empty here");

            if updated_counter > *current_group {
                cache.insert(cache_key, 0);
                0
            } else {
                solve(&springs[1..], groups, updated_counter, cache)
            }
        }
        '?' => {
            let with_operational_spring = ".".to_string() + &springs[1..].to_string();

            let with_broken_spring = "#".to_string() + &springs[1..].to_string();
            let sum_as_if_operational_spring =
                solve(&with_operational_spring, groups, group_counter, cache);
            let sum_as_if_broken_spring = solve(&with_broken_spring, groups, group_counter, cache);

            let sum = sum_as_if_operational_spring + sum_as_if_broken_spring;

            cache.insert(cache_key, sum);

            sum
        }
        _ => panic!("Invalid spring symbol"),
    }
}
