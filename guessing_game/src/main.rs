extern crate rand;  // uses the "rand" crate

use std::io;            // allows input and output
use std::cmp::Ordering; // allows use of Ordering
use rand::Rng;          // allows random number generation

fn main() {
    // display an introduction
    println!("Guess the number!");

    // create a non-mutable random number 1 <= x < 101
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // display the random number (for debugging)
    // println!("The secret number is: {}", secret_number);

    // start an infinite loop
    loop {
        // tell the user what to do
        println!("Please input your guess:");

        // create a mutable variable "guess" and
            // assign it an instance of an empty string 
        let mut guess = String::new();

        // read input from the user and assign it to "guess"
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line"); // crashes the program on invalid input, supresses a compiler warning
        
        // convert the guess into an integer
            // restart the loop if the input is invalid
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => {
                println!("\"{}\" is not a valid number", guess.trim());
                continue;
            },
        };
        
        // print the user's input
        println!("You guessed: {}", guess);

        // match expression to check guess against secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small..."),
            Ordering::Greater => println!("Too big..."),
            Ordering::Equal   => {
                println!("You got it!");
                break;  // break out of the loop
            }
        }
    }
}
