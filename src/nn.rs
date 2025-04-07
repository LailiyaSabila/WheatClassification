use ndarray::{Array2, Array1};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::StandardNormal;

fn relu(x: f64) -> f64 {
    x.max(0.0)
}

fn softmax(x: &Array1<f64>) -> Array1<f64> {
    let max = x.iter().cloned().fold(f64::NAN, f64::max);
    let exp_vals = x.mapv(|v| (v - max).exp());
    let sum = exp_vals.sum();
    exp_vals.mapv(|v| v / sum)
}

pub struct NeuralNetwork {
    w1: Array2<f64>,
    w2: Array2<f64>,
}

impl NeuralNetwork {
    pub fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        let w1 = Array2::<f64>::random((hidden_size, input_size), StandardNormal);
        let w2 = Array2::<f64>::random((output_size, hidden_size), StandardNormal);
        NeuralNetwork { w1, w2 }
    }

    pub fn forward(&self, input: &Array1<f64>) -> Array1<f64> {
        let hidden = self.w1.dot(input);
        let hidden_activated = hidden.mapv(relu);
        let output = self.w2.dot(&hidden_activated);
        softmax(&output)
    }
}
