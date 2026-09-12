use std::io;
use rand::RngExt;

struct Point {
    x: f64, 
    y: f64,
}

impl Point {
    fn distance_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }
}

fn main() {

    // Ask user for number of iterations
    println!("Enter number of desired Monte Carlo iterations: ");
    let num_iter: u64 = get_positive_integer_from_user();

    // Execute Monte-Carlo analysis
    let mut count_in: u64 = 0;
    for _ in 0..num_iter {
        // Generate random point and compute d2
        let point: Point = generate_random_point();
        let d2: f64 = point.distance_squared();

        // Count if point is inside the unit circle
        if d2 <= 1.0 {
            count_in += 1;
        }
    }

    // Estimate pi
    let pi_est: f64 = 4.0 * (count_in as f64) / (num_iter as f64);
    let pi_error: f64 = (pi_est - std::f64::consts::PI).abs();

    // Print results
    println!("Actual value of pi:    {:.9}", std::f64::consts::PI);
    println!("Estimated value of pi: {:.9}", pi_est);
    println!("Absolute error:        {:.6e}", pi_error);

}

fn generate_random_point() -> Point {
    let mut rng: rand::prelude::ThreadRng = rand::rng();

    let point: Point = Point {
        x: rng.random_range(-1.0..1.0),
        y: rng.random_range(-1.0..1.0),
    };

    return point;
}

fn get_positive_integer_from_user()-> u64 {
    let mut buffer: String = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    let num: u64 = match buffer.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    return num;
}
