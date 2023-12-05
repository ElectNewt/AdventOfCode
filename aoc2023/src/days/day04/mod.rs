#[derive(Clone)]
struct Card {
    card_identifier: i32,
    winning_numbers: Vec<i32>,
    numbers_i_have: Vec<i32>,
}


pub fn task_one(content: String) -> String {
    println!("Day 4! task 1");
    let mut total_number = 0;
    let cards = parse_file_into_cards(content);
    total_number = cards.into_iter().map(|c| get_card_points(c.clone())).sum();
    return total_number.to_string();
}


pub fn task_two(content: String) -> String {
    println!("Day 4! task 2");
    let mut won_cards: Vec<Card> = Vec::new();
    let cards = parse_file_into_cards(content);
    let mut won_cards: Vec<Card> = Vec::new();
    won_cards = cards.clone();
    let how_many_cards = cards.len();
    for (index, card) in cards.iter().enumerate() {
        let card_points = get_card_winning_numbers(card.clone());

        let block_of_cards_with_id: Vec<Card> = won_cards
            .iter()
            .filter(|&c| c.card_identifier == card.card_identifier)
            .cloned()
            .collect();


        for block_of_card in block_of_cards_with_id {
            for card_iterator in 0..card_points as usize {
                //  println!("Iteration {}", card_iterator);
                if index + card_iterator <= how_many_cards {
                    if let Some(card_at_position) = cards.get(index + card_iterator + 1) {
                        won_cards.push(card_at_position.clone());
                    } else {
                    }
                }
            }
        }
    }


    return won_cards.len().to_string();
}


fn parse_file_into_cards(content: String) -> Vec<Card> {
    let mut all_cards: Vec<Card> = Vec::new();
    let lines = content.lines();
    for line in lines {
        let card = parse_line_into_card(line);
        all_cards.push(card);
    }
    return all_cards;
}

fn parse_line_into_card(line: &str) -> Card {
    let mut card = Card {
        card_identifier: 0,
        winning_numbers: Vec::new(),
        numbers_i_have: Vec::new(),
    };

    let parts = line.split_once("|");
    match parts {
        Some((game_side, my_numbers)) => {
            card.numbers_i_have = my_numbers.split_whitespace()
                .map(|n| n.trim().parse::<i32>().unwrap())
                .collect();

            let left_side: (i32, Vec<i32>) = split_left_side(game_side);
            card.winning_numbers = left_side.1;
            card.card_identifier = left_side.0;
        }
        _ => {}
    }

    return card;
}

fn split_left_side(left_side: &str) -> (i32, Vec<i32>) {
    let parts = left_side.split_once(":");
    match parts {
        Some((card_id, winning_numbers_string)) => {
            let identifier_string = card_id.replace("Card ", "");
            let game_identifier = identifier_string.trim().parse::<i32>().unwrap();

            let winning_numbers = winning_numbers_string.split_whitespace()
                .map(|n| n.trim().parse::<i32>().unwrap())
                .collect();

            return (game_identifier, winning_numbers);
        }
        _ => {}
    }

    return (0, Vec::new());
}

fn get_card_points(card: Card) -> i32 {
    let mut card_points = 0;

    for number_i_have in card.numbers_i_have {
        if card.winning_numbers.contains(&number_i_have) {
            if card_points == 0 {
                card_points = 1;
            } else {
                card_points = card_points * 2;
            }
        }
    }

    return card_points;
}

fn get_card_winning_numbers(card: Card) -> i32 {
    let mut card_points = 0;

    for number_i_have in card.numbers_i_have {
        if card.winning_numbers.contains(&number_i_have) {
            card_points = card_points + 1;
        }
    }

    return card_points;
}
