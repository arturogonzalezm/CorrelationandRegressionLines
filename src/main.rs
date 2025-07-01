// src/main.rs

// Define a trait for regression computations.
trait RegressionStrategy {
    // Given x and y data slices, compute a regression result.
    fn compute(&self, x: &[f64], y: &[f64]) -> f64;
}

// Concrete strategy that computes only the slope of the best-fit line.
struct SlopeRegression;

impl RegressionStrategy for SlopeRegression {
    fn compute(&self, x: &[f64], y: &[f64]) -> f64 {
        let n = x.len() as f64;
        let mean_x = x.iter().sum::<f64>() / n;
        let mean_y = y.iter().sum::<f64>() / n;

        let covariance: f64 = x
            .iter()
            .zip(y.iter())
            .map(|(xi, yi)| (xi - mean_x) * (yi - mean_y))
            .sum();

        let variance_x: f64 = x.iter().map(|xi| (xi - mean_x).powi(2)).sum();

        covariance / variance_x
    }
}

// Context that holds a boxed regression strategy.
struct RegressionContext {
    strategy: Box<dyn RegressionStrategy>,
}

impl RegressionContext {
    fn new(strategy: Box<dyn RegressionStrategy>) -> RegressionContext {
        RegressionContext { strategy }
    }

    fn run(&self, x: &[f64], y: &[f64]) -> f64 {
        self.strategy.compute(x, y)
    }
}

fn main() {
    // Data points
    let physics = [15.0, 12.0, 8.0, 8.0, 7.0, 7.0, 7.0, 6.0, 5.0, 3.0];
    let history = [10.0, 25.0, 17.0, 11.0, 13.0, 17.0, 20.0, 13.0, 9.0, 15.0];

    // Inject the slope strategy into the context.
    let context = RegressionContext::new(Box::new(SlopeRegression));

    // Compute the slope and print with three decimal places.
    let slope = context.run(&physics, &history);
    println!("{:.3}", slope);
}
