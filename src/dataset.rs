use std::fs::File;
use std::io::{BufReader, BufRead};
use ndarray::{Array2, Array1};

pub fn load_dataset(path: &str) -> (Array2<f64>, Array1<usize>) {
    let file = File::open(path).expect("File not found!");
    let reader = BufReader::new(file);

    let mut features = Vec::new();
    let mut labels = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let values: Vec<f64> = line
            .split_whitespace()
            .map(|v| v.parse::<f64>().unwrap())
            .collect();

        features.push(values[..7].to_vec());
        labels.push((values[7] - 1.0) as usize);
    }

    let data = Array2::from_shape_vec((features.len(), 7), features.concat()).unwrap();
    let targets = Array1::from(labels);

    (data, targets)
}
