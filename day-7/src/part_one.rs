use std::cmp;
use std::collections::HashMap;
use std::fs;

fn parse_card(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap(),
    }
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
enum HandType {
    FiveOfaKind = 8,
    FourOfaKind = 7,
    FullHouse = 6,
    ThreeOfaKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn parse_hand(hand: &str) -> HandType {
    let mut hand_map = HashMap::new();
    for card in hand.chars() {
        let count = hand_map.entry(card).or_insert(0);
        *count += 1;
    }

    let mut current_hand = HandType::HighCard;

    for (_, count) in hand_map {
        if count == 5 {
            return HandType::FiveOfaKind;
        } else if count == 4 {
            return HandType::FourOfaKind;
        } else if count == 3 {
            if current_hand == HandType::OnePair {
                return HandType::FullHouse;
            } else {
                current_hand = HandType::ThreeOfaKind;
            }
        } else if count == 2 {
            if current_hand == HandType::ThreeOfaKind {
                return HandType::FullHouse;
            } else if current_hand == HandType::OnePair {
                return HandType::TwoPair;
            } else {
                current_hand = HandType::OnePair;
            }
        }
    }

    current_hand
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    hand_type: HandType,
    cards: String,
    bid: u32,
}

impl Hand {
    fn new(cards: &str, bid: u32) -> Hand {
        Hand {
            hand_type: parse_hand(cards),
            cards: cards.to_string(),
            bid,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.hand_type == other.hand_type {
            for (self_card, other_card) in self.cards.chars().zip(other.cards.chars()) {
                let self_card = parse_card(self_card);
                let other_card = parse_card(other_card);

                if self_card > other_card {
                    return Some(cmp::Ordering::Greater);
                } else if self_card < other_card {
                    return Some(cmp::Ordering::Less);
                }
            }

            Some(cmp::Ordering::Equal)
        } else {
            Some(self.hand_type.cmp(&other.hand_type))
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.hand_type == other.hand_type {
            for (self_card, other_card) in self.cards.chars().zip(other.cards.chars()) {
                let self_card = parse_card(self_card);
                let other_card = parse_card(other_card);

                if self_card > other_card {
                    return cmp::Ordering::Greater;
                } else if self_card < other_card {
                    return cmp::Ordering::Less;
                }
            }

            cmp::Ordering::Equal
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

pub fn part_one() {
    let input = fs::read_to_string("input").expect("Something went wrong reading the file");
    let mut hands = Vec::new();

    for line in input.lines() {
        let mut split = line.split(" ");
        let hand = split.next().unwrap();
        let bid = split.next().unwrap();
        hands.push(Hand::new(hand, bid.parse::<u32>().unwrap()));
    }

    println!("Before:");
    for hand in &hands {
        println!("{:?}", hand);
    }

    hands.sort();

    let mut total_winnings = 0;

    println!("After:");
    for (i, hand) in hands.iter().enumerate() {
        total_winnings += hand.bid * (i + 1) as u32;
        println!("{:?}", hand);
    }

    println!("Total Winnings: {}", total_winnings);
}
