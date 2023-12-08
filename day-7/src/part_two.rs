use std::cmp;
use std::collections::HashMap;
use std::fs;

fn parse_card(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
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

fn generate_combinations(n: usize, possible_chars: &str) -> Vec<String> {
    let mut combinations: Vec<String> = Vec::new();
    let chars: Vec<char> = possible_chars.chars().collect();
    for i in 0..possible_chars.len().pow(n as u32) {
        let mut temp = i;
        let mut string = String::new();
        for _j in 0..n {
            string.push(chars[temp % chars.len()]);
            temp /= chars.len();
        }
        combinations.push(string);
    }
    combinations
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    hand_type: HandType,
    cards: String,
    bid: u32,
}

impl Hand {
    fn new(cards: &str, bid: u32) -> Hand {
        let mut number_of_js = 0;
        for card in cards.chars() {
            if card == 'J' {
                number_of_js += 1;
            }
        }

        let mut best_hand = parse_hand(cards);
        if number_of_js > 0 {
            let combinations = generate_combinations(number_of_js, "23456789TQKA");
            for combination in combinations {
                let mut new_cards = cards.to_string();
                for card in combination.chars() {
                    new_cards = new_cards.replace("J", &card.to_string());
                }

                let new_hand = parse_hand(&new_cards);
                if new_hand > best_hand {
                    best_hand = new_hand;
                }
            }
        }

        Hand {
            hand_type: best_hand,
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

pub fn part_two() {
    let input = fs::read_to_string("input").expect("Something went wrong reading the file");
    let mut hands = Vec::new();

    for line in input.lines() {
        let mut split = line.split(" ");
        let hand = split.next().unwrap();
        let bid = split.next().unwrap();
        hands.push(Hand::new(hand, bid.parse::<u32>().unwrap()));
    }

    hands.sort();

    let mut total_winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        total_winnings += hand.bid * (i + 1) as u32;
    }

    println!("Total Winnings: {}", total_winnings);
}
