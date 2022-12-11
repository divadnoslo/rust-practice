// Chapter 3 Section 2 - DataTypes
//use std::io;

fn main() {
    
    // Integer DataTypes
    let a: u8 = 127;
    let b: i64 = -654321; 
    println!("a: {}   b: {}", a, b);
    println!("Default Signed Integer Max Value: {}", isize::MAX);
    println!("Default Signed Integer Min Value: {}", isize::MIN);
    println!("Default Unsigned Integer Max Value: {}", usize::MAX);
    println!("Default Unsigned Integer Min Value: {}", usize::MIN);

    // Floating Point Data Types
    let c: f32 = 3.145159;
    let d: f64 = 3.145159;
    println!("c: {}   d: {}", c, d);

    // Mathematical Operators
    let x: f64 = 3.145159;
    let y: f64 = 2.879159;

    // Addition
    let z1 = x + y;

    // Subtraction
    let z2 = x - y;

    // Multiplication
    let z3 = x * y;

    // Division
    let z4 = x / y;

    // Remainder (Modulus)
    let z5 = x % y;

    // Print Results
    println!("x: {}   y: {}", x, y);
    println!("add: {}   subtract: {}", z1, z2);
    println!("mult: {}   div: {}", z3, z4);
    println!("remainder: {}", z5);

    // Boolean Data Types
    let flag: bool = true;
    println!("flag: {}", flag);

    // Character Data Types
    let letter = 'z';
    println!("letter: {}", letter);

}

