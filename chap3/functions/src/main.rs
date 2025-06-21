fn main() {
    
    // Another function
    another_function(32);

    // Print labeled measurement
    print_labeled(6.9, 'm');

    // Five
    let x = five();

    // Plus One
    let x_1 = plus_one(x);
    println!("x = {x}");
    println!("x + 1 = {x_1}");

}

fn another_function(x: isize) {
    println!("x = {x}");
}

fn print_labeled(meas: f64, units: char) {
    println!("Measurement is {meas} {units}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}