use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Kind {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Part {
    One,
    Two,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Hand {
    cards: Vec<char>,
    kind: Kind,
    bid: usize,
}


pub fn task_one(content: String) -> String {
    println!("Day 7! task 1");
    let mut total_number = get_total_value(content, Part::One);
    return total_number.to_string();
}

pub fn task_two(content: String) -> String {
    println!("Day 7! task 2");
    let mut total_number = get_total_value(content, Part::Two);
    return total_number.to_string();
}


fn get_total_value(content: String, part: Part) -> usize {
    let mut hands: Vec<Hand> = Vec::new();

    let lines = content.lines();
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let cards: Vec<char> = parts[0].chars().collect();
        let bid: usize = parts[1].parse::<usize>().unwrap();
        let mut kind = Kind::HighCard; //no idea how to initialize to null


        if part == Part::One {
            kind = get_hand_kind(cards.clone());
        } else {
            if parts[0].contains('J') {
                kind = get_hand_kind_with_joker(cards.clone());
            } else {
                kind = get_hand_kind(cards.clone());
            }
        }

        let hand = Hand {
            bid,
            cards,
            kind,
        };

        hands.push(hand.clone());
    }

    hands.sort_by(|a, b| {
        let kind_order = a.kind.cmp(&b.kind);
        if kind_order == std::cmp::Ordering::Equal {
            // If kinds are equal, sort by cards using custom comparison
            compare_cards(&a.cards, &b.cards, part)
        } else {
            kind_order
        }
    });


    return hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.bid)
        .sum();
}

fn compare_cards(a: &Vec<char>, b: &Vec<char>, part: Part) -> std::cmp::Ordering {
    for (ca, cb) in a.iter().zip(b.iter()) {
        let value_a = card_to_value(*ca, part);
        let value_b = card_to_value(*cb, part);

        match value_a.cmp(&value_b) {
            std::cmp::Ordering::Equal => continue,
            ordering => return ordering,
        }
    }

    std::cmp::Ordering::Equal
}


fn get_hand_kind_with_joker(cards: Vec<char>) -> Kind {
    let mut card_counts = HashMap::new();

    // Count the occurrences of each card (excluding jokers)
    for &c in &cards {
        if c != 'J' {
            let count = card_counts.entry(c).or_insert(0);
            *count += 1;
        }
    }

    // Find the most repeated card (excluding jokers)
    let most_repeated_card = card_counts.iter().max_by_key(|&(_, count)| count).map(|(&card, _)| card);

    // Replace jokers with the most repeated card
    let cards_with_joker = cards.iter().map(|&c| if c == 'J' { most_repeated_card.unwrap_or('J') } else { c });

    // Count occurrences after replacing jokers
    let mut updated_card_counts = HashMap::new();
    for c in cards_with_joker.clone() {
        let count = updated_card_counts.entry(c).or_insert(0);
        *count += 1;
    }

    let updated_counts = updated_card_counts.values().collect::<Vec<_>>();

    match updated_counts.len() {
        1 => Kind::FiveOfAKind,
        2 => match updated_counts.contains(&&4) {
            true => Kind::FourOfAKind,
            false => Kind::FullHouse,
        },
        3 => match updated_counts.contains(&&3) {
            true => Kind::ThreeOfAKind,
            false => Kind::TwoPair,
        },
        4 => Kind::OnePair,
        5 => Kind::HighCard,
        _ => panic!("the card is not valid"),
    }
}

fn get_hand_kind(cards: Vec<char>) -> Kind {
    let mut card_counts = HashMap::new();
    for c in cards {
        let count = card_counts.entry(c).or_insert(0);
        *count += 1;
    }
    let counts = card_counts.values().collect::<Vec<_>>();
    match counts.len() {
        1 => Kind::FiveOfAKind,
        2 => match counts.contains(&&4) {
            true => Kind::FourOfAKind,
            false => Kind::FullHouse,
        },
        3 => match counts.contains(&&3) {
            true => Kind::ThreeOfAKind,
            false => Kind::TwoPair,
        },
        4 => Kind::OnePair,
        5 => Kind::HighCard,
        _ => panic!("the card is not valid"),
    }
}


impl From<Kind> for u64 {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::FiveOfAKind => 7,
            Kind::FourOfAKind => 6,
            Kind::FullHouse => 5,
            Kind::ThreeOfAKind => 4,
            Kind::TwoPair => 3,
            Kind::OnePair => 2,
            Kind::HighCard => 1,
        }
    }
}

fn card_to_value(card_number: char, part: Part) -> i32 {
    match card_number {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => {
            if part == Part::One {
                return 11;
            }
            return 0;
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("it cant happen")
    }
}

