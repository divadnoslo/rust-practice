// Chapter 3 Section 4

fn main() {
    
    // Example 1
    let x: f64 = 5.123456;
    print_value(x);

    // Example 2
    let a: u8 = 100;
    let b: u16 = 129;
    print_two_values(a, b);

    // Example 3
    let y: i32 = 1;
    println!("value of y: {}", y);
    println!("value of y + 1: {}", add_one_i32(y));

}

fn print_value(x: f64) {
    println!("The value is: {}", x);
}

fn print_two_values(x: u8, y: u16) {
    println!("The value is: {}", x);
    println!("The value is: {}", y);
}

fn add_one_i32(x: i32) -> i32 {
    x + 1
}
