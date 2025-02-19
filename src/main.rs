use std::io; // For handling user input
use ai::QLearningAI; // Import our custom AI implementation
use rand::SeedableRng; // For creating seeded random number generators
use rand::rngs::StdRng; // For deterministic random number generation

mod ai; // Module containing AI implementation
mod blackjack; // Module containing blackjack game logic

// Converts player and dealer card values into a single state index
fn encode_state(player_val: i32, dealer_val: i32) -> usize {
    (player_val as usize) + (dealer_val as usize) * 32
}

fn main() {
    // Initialize random number generator with a fixed seed for reproducibility
    let seed = 42u64;
    let mut rng = StdRng::seed_from_u64(seed);
    
    // Track game statistics
    let mut player_wins = 0;
    let mut dealer_wins = 0;
    let mut ties = 0;

    // Initialize AI with 1024 states (32x32 possible combinations) and 2 actions (hit/stand)
    let mut ai = QLearningAI::new_with_rng(1024, 2, 0.1, 0.99, 0.1, &mut rng);

    // Get user choice for training or simulation
    println!("Do you want to (1) train the AI or (2) simulate games with the AI?");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim().parse::<u32>().expect("Please enter a number");

    if choice == 1 {
        // Training mode
        let mut last_state = 0;
        let mut last_action = 0;

        // Train for 10000 games
        for _ in 0..10000 {
            let (player_final, dealer_final, result) = blackjack::play_game(|p_val, d_val| {
                let state = encode_state(p_val, d_val);
                let action = ai.choose_action(state);
                last_state = state;
                last_action = action;
                if action == 0 { "h".to_string() } else { "s".to_string() }
            });

            // Assign rewards based on game outcome
            let reward = match result.as_str() {
                "win" => 10.0,
                "lose" => -1.0,
                _ => -1.0,
            };

            // Update Q-table with final game state
            let final_state = encode_state(player_final, dealer_final);
            ai.update_q_table(last_state, last_action, reward, final_state);
        }
        // Save trained model to CSV file
        ai.save_to_csv("src/data.csv").expect("Failed to save Q-table to CSV");
        println!("Training complete. Player wins: {}, Dealer wins: {}, Ties: {}", player_wins, dealer_wins, ties);
    } else if choice == 2 {
        // Simulation mode
        // Load previously trained model
        ai.load_from_csv("src/data.csv").expect("Failed to load Q-table from CSV");
        
        // Simulate 1000 games
        for _ in 0..1000 {
            let (_, _, result) =
                blackjack::play_game(|p_val, d_val| {
                    let state = encode_state(p_val, d_val);
                    let action = ai.choose_action(state);
                    if action == 0 { "h".to_string() } else { "s".to_string() }
                });
            
            // Track game results
            match result.as_str() {
                "win" => player_wins += 1,
                "lose" => dealer_wins += 1,
                _ => ties += 1,
            }
        }
        // Display simulation results and statistics
        println!("Simulation complete. Player wins: {}, Dealer wins: {}, Ties: {}", player_wins, dealer_wins, ties);
        println!("Percentages: Player: {:.2}%, Dealer: {:.2}%, Ties: {:.2}%", 
            (player_wins as f64 / 1000.0) * 100.0, 
            (dealer_wins as f64 / 1000.0) * 100.0, 
            (ties as f64 / 1000.0) * 100.0);
    } else {
        println!("Invalid choice");
    }
}