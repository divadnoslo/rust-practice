// Chapter 3 Section 5 -- Control Flow (if statements, for loops)

fn main() {
    
    // Example 1
    let num = 7;

    if num < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    // Example 2
    let new_num = 7;

    if new_num % 2 == 0 {
        println!("number is even");
    } else if new_num % 3 == 0 {
        println!("number is dividable by 3");
    } else if new_num % 7 == 0 {
        println!("you picked seven");
    } else {
        println!("bad num");
    }

    // Example 3 - an infinite loop
    // loop {
    //     println!("here is an infinite loop -- no more while(true)!");
    // }

    // Example 4 - While Loop
    let mut x: u8 = 0;
    while x < 9 {
        x = x + 1;
        println!("x = {}", x);
    }

    // Example 5 - For Loop
    let a: [u8; 7] = [1, 1, 2, 4, 8, 16, 24];

    for element in a.iter() {
        println!("fibb sequence: {}", element);
    }

}
