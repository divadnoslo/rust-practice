use core::num;
use std::io;

fn main() {
    
    println!("Collatz Propagator");

    // Get positive integer from user to start
    println!("Enter a positive integer: ");
    let x: u64 = get_positive_integer_from_user();
    println!("Enter number of iterations: ");
    let n: u64 = get_positive_integer_from_user();

    // Propagate
    let result: u64 = propagate_collatz_sequence(x, n);
    println!("Propagation result: {result}")

}

fn get_positive_integer_from_user()-> u64 {

    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    let num: u64 = match buffer.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    return num;

}

fn propagate_collatz_sequence(a: u64, n: u64) -> u64 {

    let mut i: u64 = 0;
    let mut x: u64 = a;

    while i < n {

        if x % 2 == 0 {
            x = x / 2;
        }
        else {
            x = 3*x + 1;
        }

        i += 1;

    }

    return x;

}