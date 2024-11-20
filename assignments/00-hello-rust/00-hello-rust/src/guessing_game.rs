//! A simple guessing game.
//!
//! The user has to guess a randomly generated number between 1 and 100. They will be prompted to
//! input their guess and the program will tell them if their guess is too high, too low, or
//! correct.
#![warn(missing_docs)]

use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// Get the user's input as an integer.
///
/// This function will prompt the user to input their guess which is then read from the command line
/// and converted to an integer. If the input is not a valid integer, the program will panic.
fn get_input() -> i32 {
    println!("Please input your guess");

    // Read the input from the user.
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Convert the input to an integer.
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid entry."),
    }
}

/// Main function for the guessing game.
///
/// This function generates a random number between 1 and 100 and prompts the user to guess the
/// number. The user will be told if their guess is too high, too low, or correct. The game will
/// continue until the user guesses the correct number.
fn main() {
    println!("Guess the number!");

    // Generate a random number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        // Get the user's guess.
        let guess = get_input();
        print!("You guessed: {}. ", guess);

        // Compare the user's guess to the secret number.
        match secret_number.cmp(&guess) {
            Ordering::Equal => {
                println!("That is correct!");
                break;
            }
            Ordering::Greater => println!("You're guess is too low."),
            Ordering::Less => println!("You're guess is too high."),
        }
    }
}
