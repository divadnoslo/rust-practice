use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number: {}", secret_number);

    println!("Guess the number!");

    loop {

        println!("Please input your guess below...");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Large!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }

    }

}
