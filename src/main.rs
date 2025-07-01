// Project: regression-slope
// File: src/main.rs

fn main() {
    // Given scores
    let physics = [15.0, 12.0, 8.0, 8.0, 7.0, 7.0, 7.0, 6.0, 5.0, 3.0];
    let history = [10.0, 25.0, 17.0, 11.0, 13.0, 17.0, 20.0, 13.0, 9.0, 15.0];

    let n = physics.len() as f64;
    let mean_x = physics.iter().sum::<f64>() / n;
    let mean_y = history.iter().sum::<f64>() / n;

    let cov: f64 = physics
        .iter()
        .zip(history.iter())
        .map(|(x, y)| (x - mean_x) * (y - mean_y))
        .sum();

    let var_x: f64 = physics.iter().map(|x| (x - mean_x).powi(2)).sum();

    let slope = cov / var_x;

    // Print the slope rounded to three decimal places
    println!("{:.3}", slope);
}
