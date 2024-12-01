use crate::DataFrame;

// Helper function
fn vec_mul(vec_1: &Vec<f32>, vec_2: &Vec<f32>) -> f32 {
    if vec_1.len() != vec_2.len() {
        panic!("Must have vecs of same length!")
    }

    let mut sum: f32 = 0.0;
    for i in 0..vec_1.len() {
        // println!("Sum: {}", vec_1[i] * vec_2[i]);
        sum += vec_1[i] * vec_2[i];
    }
    sum
}
pub struct LinearRegressor {
    weights: Vec<f32>
}

impl LinearRegressor {
    pub fn new() -> Self {
        Self {weights: Vec::new()}
    }

    pub fn predict(&self, feature_vector: &Vec<f32>) -> f32{
        vec_mul(&self.weights, feature_vector)
    }

    fn print_progress(&self, epoch: usize, step: usize, error: f32) {
        println!("Epoch: {:<10} Step: {:<10} Mean Squared Error: {:<10}", epoch, step, error);
    }

    pub fn fit(&mut self, data: DataFrame, epochs: usize, learning_rate: f32) {
        let feature_len: usize = data.get_row(0).len();

        // create weight vector of same length with random starting values (0.1 for now)
        self.weights = vec![0.1;feature_len];

        // loop over data for every epoch
        println!("Training Start:");
        for epoch in 0..epochs {
            let mut errors: Vec<f32> = Vec::new();
            for row in 0..data.data.len() {
                // forward pass
                let x: &Vec<f32> = data.get_row(row);
                let y_actual: f32 = data.target[row];
                let y_pred: f32 = self.predict(x);
                /*** 
                    Get weight derivative:
                    In order to make the model learn, we perform "gradient descent".
                    Basically all this means is we take the derivative of the function and
                    go in the negative direction, or "descend the gradient".
                    This is why updating our weight vector looks like (weight - derivative).
                ***/
                for weight_index in 0..self.weights.len() {
                    // dertivative of weight = 2 * (error * feature) / weights.len()
                    let d_w: f32 = 2.0 * ((y_pred - y_actual) * x[weight_index]) / self.weights.len() as f32;
                    // update weights
                    // curr_weight - derivative * learning_rate
                    self.weights[weight_index] = self.weights[weight_index] - (d_w * learning_rate);
                }
                // add squared error to error sum
                errors.push((y_pred - y_actual).powi(2));
                
                // print training progress
                if row % 100 == 0 {
                    let error: f32= errors.iter().sum();
                    self.print_progress(epoch, row, error/(row + 1) as f32);
                }
            }
        }
    }
}