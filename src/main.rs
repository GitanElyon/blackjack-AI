use std::io;
use ai::QLearningAI;
use rand::SeedableRng;
use rand::rngs::StdRng;

mod ai;
mod blackjack;

fn main() {
    let seed = 42u64;
    let mut rng = StdRng::seed_from_u64(seed);
    let mut player_wins = 0;
    let mut dealer_wins = 0;
    let mut ties = 0;

    let mut ai = QLearningAI::new_with_rng(100, 2, 0.1, 0.99, 0.1, &mut rng);

    println!("Do you want to (1) train the AI or (2) simulate games with the AI?");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim().parse::<u32>().expect("Please enter a number");

    if choice == 1 {
        for _ in 0..1000 {
            let (player_hand_value, dealer_hand_value, result) = blackjack::play_game(&mut || {
                let state = 0; // Define your state representation
                let action = ai.choose_action(state);
                if action == 0 {
                    "h".to_string()
                } else {
                    "s".to_string()
                }
            });

            // Update the AI based on the game result
            let reward = match result.as_str() {
                "win" => 1.0,
                "lose" => -1.0,
                _ => 0.0,
            };
            let next_state = 0; // Define your next state representation
            ai.update_q_table(0, 0, reward, next_state); // Update with actual state and action

            // Update the win/loss/tie counters based on the game result
            match result.as_str() {
                "win" => player_wins += 1,
                "lose" => dealer_wins += 1,
                _ => ties += 1,
            }
        }
        ai.save_to_csv("src/data.csv").expect("Failed to save Q-table to CSV");
        println!("Training complete. Player wins: {}, Dealer wins: {}, Ties: {}", player_wins, dealer_wins, ties);
    } else if choice == 2 {
        ai.load_from_csv("src/data.csv").expect("Failed to load Q-table from CSV");
        for _ in 0..1000 {
            let (player_hand_value, dealer_hand_value, result) = blackjack::play_game(&mut || {
                let state = 0; // Define your state representation
                let action = ai.choose_action(state);
                if action == 0 {
                    "h".to_string()
                } else {
                    "s".to_string()
                }
            });

            // Update the win/loss/tie counters based on the game result
            match result.as_str() {
                "win" => player_wins += 1,
                "lose" => dealer_wins += 1,
                _ => ties += 1,
            }
        }
        println!("Simulation complete. Player wins: {}, Dealer wins: {}, Ties: {}", player_wins, dealer_wins, ties);
    } else {
        println!("Invalid choice");
    }
}