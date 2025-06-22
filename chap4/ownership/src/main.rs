fn main() {
    
    // 'String' datatype vs. a string literal
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    // Ownership example, the declaration of s2 invalidates s1
    // let mut s1 = String::from("hello");
    // let s2 = s1;
    // println!("{s1}, world!");

    // Scope and assignment
    let mut s = String::from("hello");
    s = String::from("ahoy");
    s.push_str(", world!");
    println!("{s}");

    // Using clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    // Data on the stack is copied, not cloned
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    // Ownership with functions
    // just go check the docs


}
