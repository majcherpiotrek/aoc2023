use std::collections::HashMap;

#[derive(Debug)]
struct RangeMapping {
    pub destination_start: usize,
    pub source_start: usize,
    pub len: usize,
}

impl RangeMapping {
    pub fn parse(line: &str) -> Option<RangeMapping> {
        let mut line_split = line.split_whitespace();

        let maybe_destination_start: Option<usize> =
            line_split.next().and_then(|s| s.parse::<usize>().ok());
        let maybe_source_start: Option<usize> =
            line_split.next().and_then(|s| s.parse::<usize>().ok());
        let maybe_len: Option<usize> = line_split.next().and_then(|s| s.parse::<usize>().ok());

        match (maybe_destination_start, maybe_source_start, maybe_len) {
            (Some(destination_start), Some(source_start), Some(len)) => Some(RangeMapping {
                destination_start,
                source_start,
                len,
            }),
            _ => None,
        }
    }

    // Returns:
    // - empty vector when no match found
    // - vector with one element when the range being mapped is completely contained withing the
    // mapping range
    // - vector with two elements when the range being mapped overlaps partially. First element is
    // the mapped range, second element is the unmapped rest of the initial range.
    pub fn map_range(&self, range: &Range) -> Vec<Range> {
        if self.overlaps(range) {
            let is_offset_positive = self.source_start <= self.destination_start;

            let offset = if is_offset_positive {
                self.destination_start - self.source_start
            } else {
                self.source_start - self.destination_start
            };

            if self.is_contained_within(range) {
                let mapped_range_start = if is_offset_positive {
                    range.start + offset
                } else {
                    range.start - offset
                };

                let mapped_range = Range {
                    start: mapped_range_start,
                    len: range.len,
                };
                vec![mapped_range]
            } else {
                if range.start < self.source_start {
                    // Range overlaps with tail
                    let overlapping_length = range.start + range.len - self.source_start;

                    let unchanged_range = Range {
                        start: range.start,
                        len: range.len - overlapping_length,
                    };
                    let mapped_range = Range {
                        start: if is_offset_positive {
                            self.source_start + offset
                        } else {
                            self.source_start - offset
                        },
                        len: overlapping_length,
                    };
                    vec![mapped_range, unchanged_range]
                } else {
                    // Range overlaps with beginning
                    let mapping_range_end = self.source_start + self.len;
                    let range_end = range.start + range.len;
                    let overlapping_length = mapping_range_end - range.start;

                    let mapped_range = Range {
                        start: if is_offset_positive {
                            range.start + offset
                        } else {
                            range.start - offset
                        },
                        len: overlapping_length,
                    };
                    let unchanged_range = Range {
                        start: self.source_start + self.len,
                        len: range_end - (self.source_start + self.len),
                    };
                    vec![mapped_range, unchanged_range]
                }
            }
        } else {
            Vec::new()
        }
    }

    pub fn is_contained_within(&self, range: &Range) -> bool {
        range.start >= self.source_start && range.start + range.len < self.source_start + self.len
    }

    fn overlaps(&self, range: &Range) -> bool {
        let range_end = range.start + range.len;
        let source_range_end = self.source_start + self.len;

        (range.start >= self.source_start && range.start < source_range_end)
            || (range.start < self.source_start && range_end > self.source_start)
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: usize,
    len: usize,
}

#[derive(Debug)]
struct Mappings {
    pub source_name: String,
    pub destination_name: String,
    pub range_mappings: Vec<RangeMapping>,
}

impl Mappings {
    pub fn parse(block: &str) -> Option<Mappings> {
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

        let ranges = block_split
            .filter_map(RangeMapping::parse)
            .collect::<Vec<RangeMapping>>();

        maybe_name.map(|(source_name, destination_name)| Mappings {
            source_name: source_name.to_string(),
            destination_name: destination_name.to_string(),
            range_mappings: ranges,
        })
    }

    pub fn get_destination_value(&self, source: usize) -> Option<usize> {
        let matching_range = self
            .range_mappings
            .iter()
            .find(|range| source >= range.source_start && source < range.source_start + range.len);
        matching_range.map(|range| {
            let offset = source - range.source_start;
            range.destination_start + offset
        })
    }

    fn map_single_range(&self, range: &Range) -> Vec<Range> {
        let mut results: Vec<Range> = Vec::new();
        let mut range_to_map = Some(range.clone());

        for mapping in self.range_mappings.iter() {
            match range_to_map {
                Some(r) => {
                    let mapping_result = mapping.map_range(&r);
                    if mapping_result.len() == 1 {
                        let mapped_range = mapping_result.get(0).expect("Unexpected error");
                        results.push(mapped_range.clone());
                        range_to_map = None;
                        break;
                    } else if mapping_result.len() == 2 {
                        let mapped_range = mapping_result.get(0).expect("Unexpected error");
                        results.push(mapped_range.clone());
                        range_to_map = mapping_result.get(1).copied();
                    }
                }
                None => break,
            }
        }

        match range_to_map {
            Some(r) => results.push(r.clone()),
            None => (),
        }

        results
    }

    pub fn map_ranges(&self, ranges: &Vec<Range>) -> Vec<Range> {
        ranges
            .iter()
            .flat_map(|range| self.map_single_range(range))
            .collect::<Vec<Range>>()
    }
}

struct Almanac {
    seeds: Vec<usize>,
    maps: HashMap<String, Mappings>,
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
            .filter_map(Mappings::parse)
            .map(|m| (m.source_name.clone(), m))
            .collect::<HashMap<String, Mappings>>();

        maybe_seeds.map(|seeds| Almanac { seeds, maps })
    }

    pub fn get_lowest_seed_destination_location(&self) -> usize {
        self.seeds
            .iter()
            .map(|seed| self.get_location_for_seed(seed))
            .min()
            .unwrap_or(0)
    }

    pub fn get_lowest_seed_destination_for_seed_ranges(&self) -> usize {
        let mut seed_ranges = self
            .seeds
            .chunks(2)
            .map(|chunk| Range {
                start: chunk[0],
                len: chunk[1],
            })
            .collect::<Vec<Range>>();
        let mut category_name = "seed";

        loop {
            let map_for_category = self
                .maps
                .get(category_name)
                .expect("Invalid category name!");

            let mapped_seed_ranges = map_for_category.map_ranges(&seed_ranges);

            if map_for_category.destination_name == "location" {
                break mapped_seed_ranges
                    .iter()
                    .map(|r| r.start)
                    .min()
                    .expect("Something went wrong - no result found");
            } else {
                seed_ranges = mapped_seed_ranges;
                category_name = &map_for_category.destination_name;
            }
        }
    }

    fn get_location_for_seed(&self, seed: &usize) -> usize {
        let mut category_name = "seed";
        let mut source = *seed;

        loop {
            let map_for_category = self
                .maps
                .get(category_name)
                .expect("Invalid category name!");
            let destination = map_for_category
                .get_destination_value(source)
                .unwrap_or(source);

            if map_for_category.destination_name == "location" {
                break destination;
            } else {
                source = destination.clone();
                category_name = &map_for_category.destination_name;
            }
        }
    }
}

pub fn read_almanac_seed_by_seed(file: &str) -> usize {
    let almanac = Almanac::parse(file).expect("Failed to parse input file");

    almanac.get_lowest_seed_destination_location()
}

pub fn read_almanac_by_seed_ranges(file: &str) -> usize {
    let almanac = Almanac::parse(file).expect("Failed to parse input file");

    almanac.get_lowest_seed_destination_for_seed_ranges()
}
