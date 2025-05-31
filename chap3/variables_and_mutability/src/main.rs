fn main() {

    // No Mutability
    // let x = 5;
    // println!("x = {x}");
    // x = 6;
    // println!("Now, x = {x}");

    // Allow Mutability
    let mut x = 5;
    println!("x = {x}");
    x = 6;
    println!("Now, x = {x}");

    // Constants
    const PI: f64 = 3.14159;
    println!("Here is a constant: pi = {PI}");

    // Shadowing
    let a = 10;
    let a = a * 2;
    {
        let a = a * 3;
        println!("In this scope, a = {a}");
    }
    println!("But now outside of the scope, it goes back to a = {a}");

    // When you don't want to use mutability -- beacuse we changed the type
    // let mut spaces = "   ";
    // spaces = spaces.len();

}
