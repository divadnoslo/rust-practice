use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // Determine Secret Number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Print Secret Number for Now
    //println!("Secret Number: {}", secret_number);

    // Begin Loop
    loop {

        // Intro
        println!("Guess the Number between 1 and 100");
        println!("Input your guess: ");

        // Initialize a mutable string variablie named "guess"
        let mut guess = String::new();

        // Read a line from the terminal, save in "guess"
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Convert 'guess' from string to "u32" datatype
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Repeat Guess
        println!("You guessed {}!", guess);

        // Compare Guess to Secret Number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small..."),
            Ordering::Greater => println!("Too Large..."),
            Ordering::Equal =>  {
                println!("You Win!");
                break;
            }
        }

    }

}
