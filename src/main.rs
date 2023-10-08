use nalgebra as na;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

fn generate_random_points(num_points: usize, seed: u64) -> Vec<Point3D> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut points = Vec::new();

    for _ in 0..num_points {
        points.push(Point3D {
            x: rng.gen::<f64>() * 100.0, // random value between 0 and 100
            y: rng.gen::<f64>() * 100.0,
            z: rng.gen::<f64>() * 100.0,
        });
    }

    points
}

fn polynomial_regression(points: &[Point3D]) -> na::DVector<f64> {
    let n = points.len();
    let mut x = na::DMatrix::<f64>::zeros(n, 10);
    let mut y = na::DVector::<f64>::zeros(n);

    for (i, point) in points.iter().enumerate() {
        x[(i, 0)] = point.x.powi(3);
        x[(i, 1)] = point.x.powi(2) * point.y;
        x[(i, 2)] = point.x * point.y.powi(2);
        x[(i, 3)] = point.y.powi(3);
        x[(i, 4)] = point.x.powi(2);
        x[(i, 5)] = point.x * point.y;
        x[(i, 6)] = point.y.powi(2);
        x[(i, 7)] = point.x;
        x[(i, 8)] = point.y;
        x[(i, 9)] = 1.0;
        y[i] = point.z;
    }

    // Return cofficients, not sure why we need to transpose
    let xt = x.transpose();
    (xt.clone() * x).try_inverse().unwrap() * xt * y
}

fn predict(point: &Point3D, coeffs: &na::DVector<f64>) -> f64 {
    coeffs[0] * point.x.powi(3)
        + coeffs[1] * point.x.powi(2) * point.y
        + coeffs[2] * point.x * point.y.powi(2)
        + coeffs[3] * point.y.powi(3)
        + coeffs[4] * point.x.powi(2)
        + coeffs[5] * point.x * point.y
        + coeffs[6] * point.y.powi(2)
        + coeffs[7] * point.x
        + coeffs[8] * point.y
        + coeffs[9]
}

fn mean_squared_error(points: &[Point3D], coeffs: &na::DVector<f64>) -> f64 {
    let n = points.len() as f64;
    points
        .iter()
        .map(|point| (point.z - predict(point, coeffs)).powi(2))
        .sum::<f64>()
        / n
}

fn main() {
    const NUM_POINTS: usize = 10;
    const SEED: u64 = 12345678;

    let data = generate_random_points(NUM_POINTS, SEED);

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

        let data = generate_random_points(NUM_POINTS, SEED);

        let coeffs = polynomial_regression(&data);

        // A simple test might be to ensure the MSE is below a certain threshold
        let mse = mean_squared_error(&data, &coeffs);
        println!("test_regression: mse={mse}");
        assert!(mse < 1.0); // Adjust this threshold as needed

        // Add more specific tests as needed
    }
}
