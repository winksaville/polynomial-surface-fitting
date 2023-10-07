extern crate rand;
extern crate nalgebra as na;

use rand::Rng;

const NUM_POINTS: usize = 10;

struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

fn generate_random_points(num_points: usize) -> Vec<Point3D> {
    let mut rng = rand::thread_rng();
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

    let xt = x.transpose();
    let coefficients = (xt.clone() * x).try_inverse().unwrap() * xt * y;

    coefficients
}

fn main() {
    let data = generate_random_points(NUM_POINTS);

    let coeffs = polynomial_regression(&data);
    println!("{:?}", coeffs);
}
