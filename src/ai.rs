use ndarray::Array2;
use ndarray_rand::RandomExt;
use rand::distributions::Uniform;
use rand::Rng;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use csv::{ReaderBuilder, WriterBuilder};

pub struct QLearningAI {
    q_table: Array2<f64>,
    learning_rate: f64,
    discount_factor: f64,
    epsilon: f64,
}

impl QLearningAI {
    pub fn new(state_size: usize, action_size: usize, learning_rate: f64, discount_factor: f64, epsilon: f64) -> Self {
        let q_table = Array2::random((state_size, action_size), Uniform::new(0.0, 1.0));
        QLearningAI {
            q_table,
            learning_rate,
            discount_factor,
            epsilon,
        }
    }

    pub fn choose_action(&mut self, state: usize) -> usize {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(self.epsilon) {
            rng.gen_range(0..self.q_table.ncols())
        } else {
            self.q_table.row(state).iter().cloned().enumerate().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().0
        }
    }

    pub fn update_q_table(&mut self, state: usize, action: usize, reward: f64, next_state: usize) {
        let best_next_action = self.q_table.row(next_state).iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let q_value = self.q_table[[state, action]];
        self.q_table[[state, action]] = q_value + self.learning_rate * (reward + self.discount_factor * best_next_action - q_value);
    }

    pub fn save_to_csv(&self, file_path: &str) -> io::Result<()> {
        let file = File::create(file_path)?;
        let mut writer = WriterBuilder::new().from_writer(BufWriter::new(file));
        for row in self.q_table.genrows() {
            writer.serialize(row.to_vec())?;
        }
        writer.flush()?;
        Ok(())
    }

    pub fn load_from_csv(&mut self, file_path: &str) -> io::Result<()> {
        let file = File::open(file_path)?;
        let mut reader = ReaderBuilder::new().from_reader(BufReader::new(file));
        for (i, result) in reader.deserialize().enumerate() {
            let row: Vec<f64> = result?;
            for (j, &value) in row.iter().enumerate() {
                self.q_table[[i, j]] = value;
            }
        }
        Ok(())
    }
}