// Testing Concepts from Chapter 3 Section 1: Variables and Mutability

fn main() {

    // Example 1 - Should Not Work, "x" is immutable
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // Example 2 - Should Work, "x" is now mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Example 3 - Constants
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum number of points is: {}", MAX_POINTS);

    // Example 4 - Shadowing Variables
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    // Example 5 - Shadowing while changing Data Type
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces: {}", spaces);

    // Example 6 - Should Not Work, Mutable variables cannot change datatype
    // let mut a: u32 = 123;
    // a = -123;
    // println!("a = {}", a);


}
