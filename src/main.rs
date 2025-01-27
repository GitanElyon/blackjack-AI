use rand::Rng;

#[derive(Debug)]
enum CardSuit {
    HEARTS,
    DIAMONDS,
    CLUBS,
    SPADES,
}

struct Card {
    suit: CardSuit,
    value: i32,
}

fn main() {
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

    println!("Would you like to hit or stay? (h/s)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();


    // player's turn
    while input == "h" {
        hand.push(deal_card(&mut deck));

        println!("Your hand: {}", hand_value(&hand));
        display_hand_visual(&hand, false);

        if hand_value(&hand) > 21 {
            println!("You busted!");
            break;
        }

        println!("Would you like to hit or stay? (h/s)");
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        if input == "s" {
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
    if hand_value(&hand) > 21 {
        println!("You busted!");
    } else if hand_value(&dealer_hand) > 21 {
        println!("Dealer busted! You win!");
    } else if hand_value(&hand) > hand_value(&dealer_hand) {
        println!("You win!");
    } else if hand_value(&hand) < hand_value(&dealer_hand) {
        println!("Dealer wins!");
    } else {
        println!("It's a tie!");
    }
}

fn build_deck() -> Vec<Card> {
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

fn deal_card(deck: &mut Vec<Card>) -> Card {
    let index = rand::thread_rng().gen_range(0..deck.len());
    deck.remove(index)
}

fn hand_value(hand: &Vec<Card>) -> i32 {
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

fn display_hand_visual(hand: &Vec<Card>, hide_first: bool) {
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

fn display_card_visual(card: &Card) -> Vec<String> {
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

