fn main() {
    
    // If Statements
    let x = 7;
    if x < 10 {
        println!("x = {x} is less than 10");
    } else {
        println!("x = {x} is greater than 10");
    }

    // Else-If Statements
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Interesting Use-Case
    let condition = true;
    let y = if condition {3} else {4};
    println!("y = {y}");

    // "loop"
    // loop {
    //    println!("again!");
    //}

    // Return a value from a "loop"
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("counter = {result}");

    // Loop Labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // While Loop
    let mut a = 3;
    while a > 0 {
        println!("{a}...");
        a -= 1;
    }
    println!("liftoff");

    // For Loop
    let b = [1, 2, 3, 4, 5];
    for element in b {
        println!("element = {element}");
    }

}
