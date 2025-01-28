use std::io;
use ai::QLearningAI;
use rand::SeedableRng;
use rand::rngs::StdRng;

mod ai;
mod blackjack;

fn encode_state(player_val: i32, dealer_val: i32) -> usize {
    (player_val as usize) + (dealer_val as usize) * 32
}

fn main() {
    let seed = 42u64;
    let mut rng = StdRng::seed_from_u64(seed);
    let mut player_wins = 0;
    let mut dealer_wins = 0;
    let mut ties = 0;

    // Encode states up to 32 * 32 = 1024
    let mut ai = QLearningAI::new_with_rng(1024, 2, 0.1, 0.99, 0.1, &mut rng);

    println!("Do you want to (1) train the AI or (2) simulate games with the AI?");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim().parse::<u32>().expect("Please enter a number");

    if choice == 1 {
        let mut last_state = 0;
        let mut last_action = 0;

        for _ in 0..10000 {
            let (player_final, dealer_final, result) = blackjack::play_game(|p_val, d_val| {
                let state = encode_state(p_val, d_val);
                let action = ai.choose_action(state);
                last_state = state;
                last_action = action;
                if action == 0 { "h".to_string() } else { "s".to_string() }
            });

            let reward = match result.as_str() {
                "win" => 10.0,
                "lose" => -1.0,
                _ => -1.0,
            };

            let final_state = encode_state(player_final, dealer_final);
            ai.update_q_table(last_state, last_action, reward, final_state);
        }
        ai.save_to_csv("src/data.csv").expect("Failed to save Q-table to CSV");
        println!("Training complete. Player wins: {}, Dealer wins: {}, Ties: {}", player_wins, dealer_wins, ties);
    } else if choice == 2 {
        ai.load_from_csv("src/data.csv").expect("Failed to load Q-table from CSV");
        for _ in 0..1000 {
            let (player_hand_value, dealer_hand_value, result) =
                blackjack::play_game(|p_val, d_val| {
                    let state = encode_state(p_val, d_val);
                    let action = ai.choose_action(state);
                    if action == 0 { "h".to_string() } else { "s".to_string() }
                });
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