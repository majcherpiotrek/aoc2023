use std::{cmp::Ordering, collections::HashMap};

pub fn calculate_total_winning(file: &str) -> usize {
    let mut hands = parse_input(file);
    hands.sort_by(|a, b| {
        let (a_hand, _) = a;
        let (b_hand, _) = b;
        let ord = if std::mem::discriminant(a_hand) == std::mem::discriminant(b_hand) {
            Ordering::Equal
        } else {
            a_hand.partial_cmp(b_hand).unwrap_or(Ordering::Equal)
        };
        if ord == Ordering::Equal {
            let mut card_ord = Ordering::Equal;
            let cards_zipped = a_hand
                .get_cards()
                .iter()
                .zip(b_hand.get_cards().iter())
                .collect::<Vec<(&Card, &Card)>>();
            for (card_a, card_b) in cards_zipped {
                card_ord = card_b.partial_cmp(&card_a).unwrap_or(Ordering::Equal);
                if card_ord != Ordering::Equal {
                    break;
                }
            }
            card_ord
        } else {
            ord
        }
    });
    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        println!("{:?}", hand);
        let (_, bet) = hand;
        let strength = hands.len() - i;
        sum += strength * bet;
    }

    sum
}

fn parse_input(file: &str) -> Vec<(Hand, usize)> {
    file.split("\n")
        .filter_map(|line| {
            let mut line_split = line.split_whitespace();
            let maybe_hand = line_split.next().and_then(Hand::parse);
            let maybe_bet = line_split.next().and_then(|num| num.parse::<usize>().ok());

            match (maybe_hand, maybe_bet) {
                (Some(hand), Some(bet)) => Some((hand, bet)),
                _ => None,
            }
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl Card {
    pub fn parse(character: &str) -> Option<Card> {
        match character {
            "2" => Some(Card::Two),
            "3" => Some(Card::Three),
            "4" => Some(Card::Four),
            "5" => Some(Card::Five),
            "6" => Some(Card::Six),
            "7" => Some(Card::Seven),
            "8" => Some(Card::Eight),
            "9" => Some(Card::Nine),
            "T" => Some(Card::Ten),
            "J" => Some(Card::Jack),
            "Q" => Some(Card::Queen),
            "K" => Some(Card::King),
            "A" => Some(Card::Ace),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Hand {
    FiveOfAKind { cards: Vec<Card> },
    FourOfAKind { cards: Vec<Card> },
    FullHouse { cards: Vec<Card> },
    ThreeOfAKind { cards: Vec<Card> },
    TwoPair { cards: Vec<Card> },
    OnePair { cards: Vec<Card> },
    HighCard { cards: Vec<Card> },
}

impl Hand {
    pub fn parse(str: &str) -> Option<Hand> {
        let cards = str
            .chars()
            .filter_map(|c| Card::parse(&c.to_string()))
            .collect::<Vec<Card>>();
        Hand::from_cards(&cards)
    }

    pub fn get_cards(&self) -> &Vec<Card> {
        match self {
            Hand::FiveOfAKind { cards }
            | Hand::FourOfAKind { cards }
            | Hand::FullHouse { cards }
            | Hand::ThreeOfAKind { cards }
            | Hand::TwoPair { cards }
            | Hand::OnePair { cards }
            | Hand::HighCard { cards } => cards,
        }
    }

    fn from_cards(cards: &Vec<Card>) -> Option<Hand> {
        if cards.len() != 5 {
            return None;
        }
        let grouped_cards = Hand::group_cards(&cards);
        let first_card = cards.get(0).unwrap();

        if grouped_cards.len() == 1 {
            Some(Hand::FiveOfAKind {
                cards: cards.to_vec(),
            })
        } else if grouped_cards.len() == 2 {
            grouped_cards.get(first_card).and_then(|num_of_cards| {
                if *num_of_cards == 1 || *num_of_cards == 4 {
                    Some(Hand::FourOfAKind {
                        cards: cards.to_vec(),
                    })
                } else if *num_of_cards == 2 || *num_of_cards == 3 {
                    Some(Hand::FullHouse {
                        cards: cards.to_vec(),
                    })
                } else {
                    None
                }
            })
        } else if grouped_cards.len() == 3 {
            let is_three_of_a_kind = grouped_cards.iter().any(|(_, n)| *n == 3);
            if is_three_of_a_kind {
                Some(Hand::ThreeOfAKind {
                    cards: cards.to_vec(),
                })
            } else {
                Some(Hand::TwoPair {
                    cards: cards.to_vec(),
                })
            }
        } else if grouped_cards.len() == 4 {
            Some(Hand::OnePair {
                cards: cards.to_vec(),
            })
        } else if grouped_cards.len() == 5 {
            Some(Hand::HighCard {
                cards: cards.to_vec(),
            })
        } else {
            None
        }
    }

    fn group_cards(cards: &Vec<Card>) -> HashMap<Card, usize> {
        let mut grouped_cards = HashMap::new();

        for card in cards.iter() {
            match grouped_cards.get(card) {
                Some(cards_already_added) => {
                    grouped_cards.insert(card.clone(), cards_already_added + 1);
                }
                None => {
                    grouped_cards.insert(card.clone(), 1 as usize);
                }
            }
        }
        grouped_cards
    }
}
