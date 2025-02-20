use std::fs::File; // For reading to the json
use std::io::{self, Read}; // For handling user input
use serde::Deserialize; // For JSON deserialization
use ai::QLearningAI; // Import our custom AI implementation
use rand::SeedableRng; // For creating seeded random number generators
use rand::rngs::StdRng; // For deterministic random number generation

mod ai; // Module containing AI implementation
mod blackjack; // Module containing blackjack game logic

#[derive(Deserialize)]
struct Settings {
    #[serde(rename = "train-games")]
    train_games: usize,
    #[serde(rename = "sim-games")]
    sim_games: usize,
    
}

// Converts player and dealer card values into a single state index
fn encode_state(player_val: i32, dealer_val: i32) -> usize {
    (player_val as usize) + (dealer_val as usize) * 32
}

fn main() {
    // Read settings from settings.json
    let mut file = File::open("settings.json").expect("Failed to open settings.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read settings.json");
    let settings: Settings = serde_json::from_str(&contents).expect("Failed to parse settings.json");
    
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

    // Determine which weights file to load based on the train_games setting
    println!("Train games: {}", settings.train_games);
    let weights_file = match settings.train_games {
        1000000 => "data/q-weights-1000000.csv",
        100000 => "data/q-weights-100000.csv",
        10000 => "data/q-weights-10000.csv",
        1000 => "data/q-weights-1000.csv",
        100 => "data/q-weights-100.csv",
        10 => "data/q-weights-10.csv",
        _ => "data/q-weights-0.csv", 
    };

    if choice == 1 {
        // Training mode
        let mut last_state = 0;
        let mut last_action = 0;

        // Train for 10000 games
        for _ in 0..settings.train_games {
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
        ai.save_to_csv(weights_file).expect("Failed to save Q-table to CSV");
        println!("Training complete. Player wins: {}, Dealer wins: {}, Ties: {}", player_wins, dealer_wins, ties);
    } else if choice == 2 {
        // Simulation mode

        

        // Load previously trained model
        ai.load_from_csv(weights_file).expect("Failed to load Q-table from CSV");
        
        // Simulate 1000 games
        for _ in 0..settings.sim_games {
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
        println!("Percentage of player wins: {:.2}%", (player_wins as f64 / settings.sim_games as f64) * 100.0);
        println!("Percentage of dealer wins: {:.2}%", (dealer_wins as f64 / settings.sim_games as f64) * 100.0);
        println!("Percentage of ties: {:.2}%", (ties as f64 / settings.sim_games as f64) * 100.0);
    } else {
        println!("Invalid choice");
    }
}