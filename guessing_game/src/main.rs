use std::io;    // allows input and output

fn main() {
    // print two lines to give the user the instructions
    println!("Guess the number!");
    println!("Please input your guess:");

    // creates a mutable variable "guess" and assigns it a value of an empty string 
    let mut guess = String::new();

    // read input from the user and assign it to "guess"
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    
    // print the user's input
    println!("You guessed: {}", guess);
}
