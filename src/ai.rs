// Required external crates and dependencies
use ndarray::Array2; // For 2D array operations
use ndarray_rand::RandomExt; // For random array initialization
use rand::distributions::Uniform; // For uniform random distribution
use rand::Rng; // For random number generation traits
use rand::rngs::StdRng; // For seeded random number generation
use std::fs::File; // For file operations
use std::io::{self, BufReader, BufWriter}; // For buffered I/O operations
use csv::{ReaderBuilder, WriterBuilder}; // For CSV file handling

// Q-Learning AI implementation
pub struct QLearningAI {
    q_table: Array2<f64>, // Stores Q-values for state-action pairs
    learning_rate: f64, // Alpha: How much new information overrides old (0 to 1)
    discount_factor: f64, // Gamma: Importance of future rewards (0 to 1)
    epsilon: f64, // Exploration rate: Probability of choosing random action
}

impl QLearningAI {
    // Creates a new AI instance with randomly initialized Q-table
    #[allow(dead_code)]
    pub fn new(state_size: usize, action_size: usize, learning_rate: f64, discount_factor: f64, epsilon: f64) -> Self {
        let q_table = Array2::random((state_size, action_size), Uniform::new(0.0, 1.0));
        QLearningAI {
            q_table,
            learning_rate,
            discount_factor,
            epsilon,
        }
    }

    // Creates a new AI instance with a specified random number generator
    pub fn new_with_rng(
        num_states: usize,
        num_actions: usize,
        learning_rate: f64,
        discount_factor: f64,
        epsilon: f64,
        rng: &mut StdRng,
    ) -> Self {
        // Initialize Q-table with custom random values
        let mut q_table = vec![vec![0.0; num_actions]; num_states];
        for state in 0..num_states {
            for action in 0..num_actions {
                // Generate random Q-values between 0 and 1
                q_table[state][action] = rng.gen_range(0.0..1.0);
            }
        }

        QLearningAI {
            q_table: Array2::from_shape_vec((num_states, num_actions), q_table.into_iter().flatten().collect()).unwrap(),
            learning_rate,
            discount_factor,
            epsilon,
        }
    }

    // Selects an action using epsilon-greedy policy
    pub fn choose_action(&mut self, state: usize) -> usize {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(self.epsilon) {
            // Exploration: Choose random action
            rng.gen_range(0..self.q_table.ncols())
        } else {
            // Exploitation: Choose best known action
            self.q_table.row(state).iter().cloned().enumerate().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().0
        }
    }

    // Updates Q-value for a state-action pair based on reward and next state
    pub fn update_q_table(&mut self, state: usize, action: usize, reward: f64, next_state: usize) {
        // Find the maximum Q-value for the next state
        let best_next_action = self.q_table.row(next_state).iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let q_value = self.q_table[[state, action]];
        // Q-learning update formula: Q(s,a) = Q(s,a) + α[R + γ*max(Q(s',a')) - Q(s,a)]
        self.q_table[[state, action]] = q_value + self.learning_rate * (reward + self.discount_factor * best_next_action - q_value);
    }

    // Saves the Q-table to a CSV file
    pub fn save_to_csv(&self, file_path: &str) -> io::Result<()> {
        let file = File::create(file_path)?;
        let mut writer = WriterBuilder::new().from_writer(BufWriter::new(file));
        // Write each row of the Q-table to the CSV file
        for row in self.q_table.rows() {
            writer.serialize(row.to_vec())?;
        }
        writer.flush()?;
        Ok(())
    }

    // Loads the Q-table from a CSV file
    pub fn load_from_csv(&mut self, file_path: &str) -> io::Result<()> {
        let file = File::open(file_path)?;
        let mut reader = ReaderBuilder::new().from_reader(BufReader::new(file));
        // Read each row from the CSV file and update the Q-table
        for (i, result) in reader.deserialize().enumerate() {
            let row: Vec<f64> = result?;
            for (j, &value) in row.iter().enumerate() {
                self.q_table[[i, j]] = value;
            }
        }
        Ok(())
    }
}