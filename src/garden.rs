use std::collections::HashMap;

#[derive(Debug)]
struct Range {
    pub destination_start: usize,
    pub source_start: usize,
    pub len: usize,
}

impl Range {
    pub fn parse(line: &str) -> Option<Range> {
        let mut line_split = line.split_whitespace();

        let maybe_destination_start: Option<usize> =
            line_split.next().and_then(|s| s.parse::<usize>().ok());
        let maybe_source_start: Option<usize> =
            line_split.next().and_then(|s| s.parse::<usize>().ok());
        let maybe_len: Option<usize> = line_split.next().and_then(|s| s.parse::<usize>().ok());

        match (maybe_destination_start, maybe_source_start, maybe_len) {
            (Some(destination_start), Some(source_start), Some(len)) => {
                Some(Range {
                    destination_start,
                    source_start,
                    len,
                })
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Map {
    pub source_name: String,
    pub destination_name: String,
    pub ranges: Vec<Range>,
}

impl Map {
    pub fn parse(block: &str) -> Option<Map> {
        let mut block_split = block.split("\n");

        let maybe_name = block_split.next().and_then(|line| {
            line.split(" map:").next().and_then(|name_str| {
                let mut name_split = name_str.split("-");
                let maybe_source_name = name_split.next();
                name_split.next();
                let maybe_destination_name = name_split.next();

                match (maybe_source_name, maybe_destination_name) {
                    (Some(source_name), Some(destination_name)) => {
                        Some((source_name, destination_name))
                    }
                    _ => None,
                }
            })
        });

        let ranges = block_split.filter_map(Range::parse).collect::<Vec<Range>>();

        maybe_name.map(|(source_name, destination_name)| {
            Map {
                source_name: source_name.to_string(),
                destination_name: destination_name.to_string(),
                ranges,
            }
        })
    }

    pub fn get_destination_value(&self, source: usize) -> Option<usize> {
        let matching_range = self.ranges.iter().find(|range| source >= range.source_start && source < range.source_start + range.len);
        matching_range.map(|range| {
            let offset = source - range.source_start;
            range.destination_start + offset
        })
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: HashMap<String, Map>,
}

impl Almanac {
    pub fn parse(file: &str) -> Option<Almanac> {
        let mut blocks_split = file.split("\n\n");

        let maybe_seeds = blocks_split.next().and_then(|line| {
            let mut seeds_split = line.split("seeds: ");
            seeds_split.nth(1).map(|seeds_str| {
                seeds_str
                    .split_whitespace()
                    .filter_map(|seed_str| seed_str.parse::<usize>().ok())
                    .collect::<Vec<usize>>()
            })
        });

        let maps = blocks_split
            .filter_map(Map::parse)
            .map(|m| (m.source_name.clone(), m))
            .collect::<HashMap<String, Map>>();

        maybe_seeds.map(|seeds| Almanac { seeds, maps })
    }

    pub fn get_lowest_seed_destination_location(&self) -> usize {
        self.seeds
            .iter()
            .map(|seed| self.get_location_for_seed(seed))
            .min()
            .unwrap_or(0)
    }

    fn get_location_for_seed(&self, seed: &usize) -> usize {
        let mut category_name = "seed";
        let mut source = *seed;

        loop {
            let map_for_category = self
                .maps
                .get(category_name)
                .expect("Invalid category name!");
            let destination = map_for_category.get_destination_value(source).unwrap_or(source);

            if map_for_category.destination_name == "location" {
                break destination;
            } else {
                source = destination.clone();
                category_name = &map_for_category.destination_name;
            }
        }
    }
}

pub fn read_almanac(file: &str) -> usize {
    let almanac = Almanac::parse(file).expect("Failed to parse input file");

    almanac.get_lowest_seed_destination_location()
}
