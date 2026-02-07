use rand::Rng;

// Card suit enumeration representing the four possible suits in a deck
#[derive(Debug)]
pub enum CardSuit {
    HEARTS,
    DIAMONDS,
    CLUBS,
    SPADES,
}

// Card structure containing suit and numerical value
pub struct Card {
    pub suit: CardSuit,
    pub value: i32,
}

// Creates and returns a complete deck of 52 cards
pub fn build_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();
    // Create cards for each suit (1-13 for Ace through King)
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

// Removes and returns a random card from the deck
pub fn deal_card(deck: &mut Vec<Card>) -> Card {
    let index = rand::thread_rng().gen_range(0..deck.len());
    deck.remove(index)
}

// Calculates the total value of a hand, handling aces appropriately
pub fn hand_value(hand: &Vec<Card>) -> i32 {
    let mut value = 0;
    let mut aces = 0;

    // First pass: count aces and sum other cards
    for card in hand {
        if card.value == 1 {
            aces += 1;
        } else if card.value > 10 {
            value += 10;  // Face cards are worth 10
        } else {
            value += card.value;
        }
    }

    // Second pass: optimize ace values (1 or 11)
    for _ in 0..aces {
        if value + 11 <= 21 {
            value += 11;
        } else {
            value += 1;
        }
    }

    value
}

// Displays a visual representation of the cards in a hand
pub fn display_hand_visual(hand: &Vec<Card>, hide_first: bool) {
    let mut card_lines: Vec<Vec<String>> = vec![vec![]; 7];

    // If hiding first card (dealer's hole card)
    if hide_first {
        // Add hidden card representation
        card_lines[0].push("┌─────┐".to_string());
        card_lines[1].push("|░░░░░|".to_string());
        card_lines[2].push("|░░░░░|".to_string());
        card_lines[3].push("|░░░░░|".to_string());
        card_lines[4].push("└─────┘".to_string());

        // Show remaining cards
        for card in &hand[1..] {
            let lines = display_card_visual(card);
            for (i, line) in lines.iter().enumerate() {
                card_lines[i].push(line.clone());
            }
        }
    } else {
        // Show all cards
        for card in hand {
            let lines = display_card_visual(card);
            for (i, line) in lines.iter().enumerate() {
                card_lines[i].push(line.clone());
            }
        }
    }

    // Print the complete hand visualization
    for line in card_lines {
        if !line.is_empty() {
             // We don't want to print if we are in silent mode, 
             // but this function itself doesn't know about silent mode.
             // I'll update play_game to skip calling this.
             println!("{}", line.join(" "));
        }
    }
}

// Creates visual representation for a single card
pub fn display_card_visual(card: &Card) -> Vec<String> {
    // Convert suit to Unicode symbol
    let suit_visual = match card.suit {
        CardSuit::HEARTS => "♥",
        CardSuit::DIAMONDS => "♦",
        CardSuit::CLUBS => "♣",
        CardSuit::SPADES => "♠",
    };

    // Convert card value to display character
    let value_visual = match card.value {
        1 => "A",
        11 => "J",
        12 => "Q",
        13 => "K",
        _ => &card.value.to_string(),
    };

    // Return card's visual representation as ASCII art
    vec![
        "┌─────┐".to_string(),
        format!("|{:<5}|", value_visual),
        format!("| {:?} |", suit_visual),
        format!("|{:>5}|", value_visual),
        "└─────┘".to_string(),
    ]
}

// Main game logic function that plays one complete game
pub fn play_game<F>(mut decide_action: F, quiet: bool) -> (i32, i32, String)
where
    F: FnMut(i32, i32) -> String,
{
    let mut deck = build_deck();

    if !quiet {
        println!("Welcome to blackjack!");
    }

    // Initialize hands
    let mut hand: Vec<Card> = Vec::new();
    let mut dealer_hand: Vec<Card> = Vec::new();

    // Initial deal: two cards each
    hand.push(deal_card(&mut deck));
    hand.push(deal_card(&mut deck));
    dealer_hand.push(deal_card(&mut deck));
    dealer_hand.push(deal_card(&mut deck));

    // Display initial hands (dealer's first card hidden)
    if !quiet {
        println!("Dealer's hand:");
        display_hand_visual(&dealer_hand, true);
        println!("Your hand: {}", hand_value(&hand));
        display_hand_visual(&hand, false);
    }

    // Player's turn
    loop {
        let player_val = hand_value(&hand);
        let dealer_val = hand_value(&dealer_hand);
        let input = decide_action(player_val, dealer_val);

        if input == "h" {
            // Player hits
            hand.push(deal_card(&mut deck));
            
            if !quiet {
                println!("Your hand: {}", hand_value(&hand));
                display_hand_visual(&hand, false);
            }

            if hand_value(&hand) > 21 {
                if !quiet {
                    println!("You busted!");
                }
                return (hand_value(&hand), hand_value(&dealer_hand), "lose".to_string());
            }
        } else if input == "s" {
            break;  // Player stands
        }
    }

    // Dealer's turn (must hit on 16 and below)
    while hand_value(&hand) <= 21 && hand_value(&dealer_hand) < 17 {
        dealer_hand.push(deal_card(&mut deck));
    }

    if !quiet {
        println!("Dealer's hand: {}", hand_value(&dealer_hand));
        display_hand_visual(&dealer_hand, false);
    }

    // Determine winner based on final hand values
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