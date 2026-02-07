use std::env;
use std::fs::File; // For reading to the json
use std::io::Read; // For handling user input
use std::time::Instant; // For performance measurement
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

fn print_help() {
    println!("Blackjack AI - Usage:");
    println!("  -S, --sim              Run in simulation mode");
    println!("  -T, --train            Run in training mode");
    println!("  -C, --count <num>      Number of games to simulate/train");
    println!("  -d, --dataset <num>    Which trained weight file to use (e.g., 1000 for q-weights-1000.csv)");
    println!("  -v, --visual           Enable card visualizations (slower)");
    println!("  -V, --verbose          Show detailed performance statistics");
    println!("  -h, --help             Show this help menu");
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

    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 || args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print_help();
        return;
    }

    let mut mode = None; // 'S' for simulation, 'T' for training
    let mut game_count = None;
    let mut dataset_basis = None;
    let mut visual = false;
    let mut verbose = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-S" | "--sim" => mode = Some('S'),
            "-T" | "--train" => mode = Some('T'),
            "-C" | "--count" => {
                if i + 1 < args.len() {
                    game_count = Some(args[i + 1].parse::<usize>().expect("Invalid game count"));
                    i += 1;
                }
            }
            "-d" | "--dataset" => {
                if i + 1 < args.len() {
                    dataset_basis = Some(args[i + 1].parse::<usize>().expect("Invalid dataset number"));
                    i += 1;
                }
            }
            "-v" | "--visual" => visual = true,
            "-V" | "--verbose" => verbose = true,
            _ => {}
        }
        i += 1;
    }

    let mode = mode.expect("Please specify --sim (-S) or --train (-T). Use --help for usage.");
    
    // Determine game count
    let count = match mode {
        'T' => game_count.unwrap_or(settings.train_games),
        'S' => game_count.unwrap_or(settings.sim_games),
        _ => unreachable!(),
    };

    // Determine which weights file to load/save
    let weight_basis = if mode == 'T' { 
        count 
    } else { 
        dataset_basis.unwrap_or(settings.train_games)
    };
    let weights_file_string = format!("data/q-weights-{}.csv", weight_basis);
    let weights_file = weights_file_string.as_str();

    if mode == 'T' {
        // Training mode
        println!("Training for {} games...", count);
        let start_time = Instant::now();
        let mut last_state = 0;
        let mut last_action = 0;

        // Train for specified games
        for _ in 0..count {
            let (player_final, dealer_final, result) = blackjack::play_game(|p_val, d_val| {
                let state = encode_state(p_val, d_val);
                let action = ai.choose_action(state);
                last_state = state;
                last_action = action;
                if action == 0 { "h".to_string() } else { "s".to_string() }
            }, !visual);

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
        let duration = start_time.elapsed();

        // Save trained model to CSV file
        ai.save_to_csv(weights_file).expect("Failed to save Q-table to CSV");
        println!("Training complete. Q-weights saved to {}", weights_file);

        if verbose {
            println!("Total training time: {:.2?}", duration);
            println!("Average time per game: {:.2?}", duration / (count as u32));
        }
    } else if mode == 'S' {
        // Simulation mode
        if !visual {
            println!("Simulating {} games using {}...", count, weights_file);
        }

        let start_time = Instant::now();

        // Load previously trained model
        ai.load_from_csv(weights_file).expect("Failed to load Q-table from CSV");
        
        // Simulate games
        for _ in 0..count {
            let (_, _, result) =
                blackjack::play_game(|p_val, d_val| {
                    let state = encode_state(p_val, d_val);
                    let action = ai.choose_action(state);
                    if action == 0 { "h".to_string() } else { "s".to_string() }
                }, !visual);
            
            // Track game results
            match result.as_str() {
                "win" => player_wins += 1,
                "lose" => dealer_wins += 1,
                _ => ties += 1,
            }
        }
        let duration = start_time.elapsed();

        // Display simulation results and statistics
        println!("Simulation complete. Player wins: {}, Dealer wins: {}, Ties: {}", player_wins, dealer_wins, ties);
        println!("Percentage of player wins: {:.2}%", (player_wins as f64 / count as f64) * 100.0);
        println!("Percentage of dealer wins: {:.2}%", (dealer_wins as f64 / count as f64) * 100.0);
        println!("Percentage of ties: {:.2}%", (ties as f64 / count as f64) * 100.0);

        if verbose {
            println!("Total simulation time: {:.2?}", duration);
            println!("Average time per game: {:.2?}", duration / (count as u32));
        }
    }
}
