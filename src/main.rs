mod dataset;
mod nn;

use dataset::load_dataset;
use nn::NeuralNetwork;

fn main() {
    let (x_data, y_data) = load_dataset("seeds_dataset.txt");

    println!("Jumlah sample: {}", x_data.shape()[0]);
    println!("Fitur pertama: {:?}", x_data.row(0));
    println!("Label pertama: {}", y_data[0]);

    let mut nn = NeuralNetwork::new(7, 10, 3);

    let sample = x_data.row(0).to_owned();
    let prediction = nn.forward(&sample);

    println!("Output prediksi (probabilitas): {:?}", prediction);

    let predicted_label = prediction
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(idx, _)| idx)
        .unwrap();

    println!("Prediksi Label: {}", predicted_label);
    println!("Label Sebenarnya: {}", y_data[0]);
}
