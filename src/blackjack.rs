use rand::Rng;

#[derive(Debug)]
pub enum CardSuit {
    HEARTS,
    DIAMONDS,
    CLUBS,
    SPADES,
}

pub struct Card {
    pub suit: CardSuit,
    pub value: i32,
}

pub fn build_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();
    for i in 1..14 {
        deck.push(Card {
            suit: CardSuit::HEARTS,
            value: i,
        });
        deck.push(Card {
            suit: CardSuit::DIAMONDS,
            value: i,
        });
        deck.push(Card {
            suit: CardSuit::CLUBS,
            value: i,
        });
        deck.push(Card {
            suit: CardSuit::SPADES,
            value: i,
        });
    }
    deck
}

pub fn deal_card(deck: &mut Vec<Card>) -> Card {
    let index = rand::thread_rng().gen_range(0..deck.len());
    deck.remove(index)
}

pub fn hand_value(hand: &Vec<Card>) -> i32 {
    let mut value = 0;
    let mut aces = 0;

    for card in hand {
        if card.value == 1 {
            aces += 1;
        } else if card.value > 10 {
            value += 10;
        } else {
            value += card.value;
        }
    }

    for _ in 0..aces {
        if value + 11 <= 21 {
            value += 11;
        } else {
            value += 1;
        }
    }

    value
}

pub fn display_hand_visual(hand: &Vec<Card>, hide_first: bool) {
    let mut card_lines: Vec<Vec<String>> = vec![vec![]; 7];

    if hide_first {
        card_lines[0].push("┌─────┐".to_string());
        card_lines[1].push("|░░░░░|".to_string());
        card_lines[2].push("|░░░░░|".to_string());
        card_lines[3].push("|░░░░░|".to_string());
        card_lines[4].push("└─────┘".to_string());

        for card in &hand[1..] {
            let lines = display_card_visual(card);
            for (i, line) in lines.iter().enumerate() {
                card_lines[i].push(line.clone());
            }
        }
    } else {
        for card in hand {
            let lines = display_card_visual(card);
            for (i, line) in lines.iter().enumerate() {
                card_lines[i].push(line.clone());
            }
        }
    }

    for line in card_lines {
        println!("{}", line.join(" "));
    }
}

pub fn display_card_visual(card: &Card) -> Vec<String> {
    let suit_visual = match card.suit {
        CardSuit::HEARTS => "♥",
        CardSuit::DIAMONDS => "♦",
        CardSuit::CLUBS => "♣",
        CardSuit::SPADES => "♠",
    };

    let value_visual = match card.value {
        1 => "A",
        11 => "J",
        12 => "Q",
        13 => "K",
        _ => &card.value.to_string(),
    };

    vec![
        "┌─────┐".to_string(),
        format!("|{:<5}|", value_visual),
        format!("| {:?} |", suit_visual),
        format!("|{:>5}|", value_visual),
        "└─────┘".to_string(),
    ]
}

pub fn play_game<F>(mut decide_action: F) -> (i32, i32, String)
where
    F: FnMut(i32, i32) -> String,
{
    let mut deck = build_deck();

    println!("Welcome to blackjack!");

    // creating player's hand
    let mut hand: Vec<Card> = Vec::new();

    // dealing two cards to player
    hand.push(deal_card(&mut deck));
    hand.push(deal_card(&mut deck));

    // creating dealer's hand
    let mut dealer_hand: Vec<Card> = Vec::new();

    // dealing two cards to dealer
    dealer_hand.push(deal_card(&mut deck));
    dealer_hand.push(deal_card(&mut deck));

    println!("Dealer's hand:");
    display_hand_visual(&dealer_hand, true);

    println!("Your hand: {}", hand_value(&hand));
    display_hand_visual(&hand, false);

    // player's turn
    loop {
        let player_val = hand_value(&hand);
        let dealer_val = hand_value(&dealer_hand);
        let input = decide_action(player_val, dealer_val);

        if input == "h" {
            hand.push(deal_card(&mut deck));

            println!("Your hand: {}", hand_value(&hand));
            display_hand_visual(&hand, false);

            if hand_value(&hand) > 21 {
                println!("You busted!");
                return (hand_value(&hand), hand_value(&dealer_hand), "lose".to_string());
            }
        } else if input == "s" {
            break;
        }
    }

    // dealer's turn
    while hand_value(&hand) <= 21 && hand_value(&dealer_hand) < 17 {
        dealer_hand.push(deal_card(&mut deck));
    }

    println!("Dealer's hand: {}", hand_value(&dealer_hand));
    display_hand_visual(&dealer_hand, false);

    // determine winner
    let result = if hand_value(&hand) > 21 {
        "lose".to_string()
    } else if hand_value(&dealer_hand) > 21 {
        "win".to_string()
    } else if hand_value(&hand) > hand_value(&dealer_hand) {
        "win".to_string()
    } else if hand_value(&hand) < hand_value(&dealer_hand) {
        "lose".to_string()
    } else {
        "tie".to_string()
    };

    (hand_value(&hand), hand_value(&dealer_hand), result)
}