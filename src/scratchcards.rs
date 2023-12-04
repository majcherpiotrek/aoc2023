#[derive(Debug)]
struct ScratchCard {
    pub id: usize,
    pub num_of_winning_numbers: usize,
}

impl ScratchCard {
    pub fn parse_many(cards_stack: &str) -> Vec<ScratchCard> {
        cards_stack
            .split("\n")
            .filter_map(ScratchCard::parse)
            .collect::<Vec<ScratchCard>>()
    }

    pub fn parse(line: &str) -> Option<ScratchCard> {
        let mut line_split = line.split(": ");
        let maybe_card_id = line_split
            .next()
            .and_then(|s| s.split_whitespace().nth(1))
            .and_then(|s| s.parse::<usize>().ok());

        maybe_card_id
            .and_then(|id| line_split.next().map(|tail| (id, tail)))
            .map(|(id, tail)| {
                let mut numbers_split = tail.split(" | ");

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

                let num_of_winning_numbers: usize = elfs_numbers.into_iter().fold(0, |acc, num| {
                    let is_winning = winning_numbers.contains(&num);

                    if is_winning {
                        acc + 1
                    } else {
                        acc
                    }
                });

                ScratchCard {
                    id,
                    num_of_winning_numbers
                }
            })
    }
}

pub fn sum_scratchcard_points(cards_stack: &str) -> usize {
    cards_stack
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

fn count_card_tree_size(all_cards: &Vec<ScratchCard>, card: &ScratchCard) -> usize {
    if card.num_of_winning_numbers > 0 {
       let start = card.id;
       let end = card.id + card.num_of_winning_numbers;
       
       let cards_to_copy = &all_cards[start..end]; 
       let children: usize = cards_to_copy.iter().fold(0, |acc, copied_card| acc + count_card_tree_size(all_cards, copied_card));
       children + 1
    } else {
        1
    }
}

pub fn process_scratchcards(cards_stack: &str) -> usize {
  let cards = ScratchCard::parse_many(cards_stack);
  cards.iter().map(|card| count_card_tree_size(&cards, card)).fold(0, |acc, elem| acc + elem) 
}
