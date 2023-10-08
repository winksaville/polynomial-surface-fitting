extern crate nalgebra as na;
extern crate rand;

use rand::prelude::*;
use rand::rngs::StdRng;

struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

fn generate_random_points(num_points: usize, seed: u64, min: f64, max: f64) -> Vec<Point3D> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut points = Vec::new();

    let scale = |v: f64| (v * (max - min)) + min;

    for _ in 0..num_points {
        points.push(Point3D {
            x: scale(rng.gen::<f64>()),
            y: scale(rng.gen::<f64>()),
            z: scale(rng.gen::<f64>()),
        });
    }

    points
}

fn polynomial_regression(points: &[Point3D]) -> na::DVector<f64> {
    let n = points.len();
    let mut design_matrix = na::DMatrix::<f64>::zeros(n, 10);
    let mut dependent_var = na::DVector::<f64>::zeros(n);

    for (i, point) in points.iter().enumerate() {
        design_matrix[(i, 0)] = point.x.powi(3);
        design_matrix[(i, 1)] = point.x.powi(2) * point.z;
        design_matrix[(i, 2)] = point.x * point.z.powi(2);
        design_matrix[(i, 3)] = point.z.powi(3);
        design_matrix[(i, 4)] = point.x.powi(2);
        design_matrix[(i, 5)] = point.x * point.z;
        design_matrix[(i, 6)] = point.z.powi(2);
        design_matrix[(i, 7)] = point.x;
        design_matrix[(i, 8)] = point.z;
        design_matrix[(i, 9)] = 1.0;
        dependent_var[i] = point.y;
    }

    let xt = design_matrix.transpose();
    #[allow(clippy::let_and_return)]
    let coefficients = (xt.clone() * design_matrix).try_inverse().unwrap() * xt * dependent_var;
    //println!("polynomial_regression:- coefficients: {coefficients:?}");
    coefficients
}

// Predict Y value
fn predict_y(point: &Point3D, coeffs: &na::DVector<f64>) -> f64 {
    coeffs[0] * point.x.powi(3)
        + coeffs[1] * point.x.powi(2) * point.z
        + coeffs[2] * point.x * point.z.powi(2)
        + coeffs[3] * point.z.powi(3)
        + coeffs[4] * point.x.powi(2)
        + coeffs[5] * point.x * point.z
        + coeffs[6] * point.z.powi(2)
        + coeffs[7] * point.x
        + coeffs[8] * point.z
        + coeffs[9]
}

fn mean_squared_error(points: &[Point3D], coeffs: &na::DVector<f64>) -> f64 {
    let n = points.len() as f64;
    points
        .iter()
        .map(|point| (point.y - predict_y(point, coeffs)).powi(2))
        .sum::<f64>()
        / n
}

fn main() {
    const NUM_POINTS: usize = 10;
    const SEED: u64 = 12345678;

    let data = generate_random_points(NUM_POINTS, SEED, -10.0, 10.0);

    let coeffs = polynomial_regression(&data);
    println!("coeffs: {:?}", coeffs);

    let mse = mean_squared_error(&data, &coeffs);
    println!("mse: {mse}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regression() {
        const NUM_POINTS: usize = 10;
        const SEED: u64 = 987654321;

        let data = generate_random_points(NUM_POINTS, SEED, -10.0, 10.0);

        let coeffs = polynomial_regression(&data);

        // A simple test might be to ensure the MSE is below a certain threshold
        let mse = mean_squared_error(&data, &coeffs);
        println!("test_regression: mse={mse}");
        assert!(mse < 1.0); // Adjust this threshold as needed

        // Add more specific tests as needed
    }
}
