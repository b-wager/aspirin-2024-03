//! This module provides a simple calculator that can perform bitwise operations on two numbers.
//!
//! The user will be prompted to input two numbers and an operation (AND, OR, or XOR). The program
//! will then perform the operation on the two numbers and display the result.

use std::io;

/// Get the user's input as an integer.
///
/// This function will prompt the user to input a number in either binary, decimal, or hexadecimal
/// format. A hexadecimal number should start with "0x" and a binary number should start with "0b".
/// The input will be read from the command line and converted to an integer. If the input is not a
/// valid number, the program will panic.
fn get_number_input() -> i32 {
    let mut input = String::new();
    // Read the input from the user.
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if &input.trim()[..2] == "0x" {
        // Convert hexadecimal input to an integer.
        match i32::from_str_radix(&input.trim()[2..], 16) {
            Ok(num) => num,
            Err(_) => panic!("Invalid hexadecimal entry."),
        }
    } else if &input.trim()[..2] == "0b" {
        // Convert binary input to an integer.
        match i32::from_str_radix(&input.trim()[2..], 2) {
            Ok(num) => num,
            Err(_) => panic!("Invalid binary entry."),
        }
    } else {
        // Convert decimal input to an integer.
        match input.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Invalid entry."),
        }
    }
}

/// Get the user's input for the desired operation.
///
/// This function will prompt the user to input the desired operation (AND, OR, or XOR). The input
/// will be read from the command line and converted to a character. If the input is not a valid
/// operation, the program will panic.
fn get_operation_input() -> char {
    println!("Please enter the desired operation:");

    let mut input = String::new();
    // Read the input from the user.
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Convert the input to a character.
    match input.trim().to_uppercase().as_str() {
        "AND" | "&" => '&',
        "OR" | "|" => '|',
        "XOR" | "^" => '^',
        _ => panic!("Invalid operation."),
    }
}

/// Perform the desired operation on the two numbers.
///
/// This function will perform the desired bitwise operation (AND, OR, or XOR) on the two input
/// numbers and return the result.
fn do_operation(first_number: i32, second_number: i32, operation: char) -> i32 {
    match operation {
        '&' => first_number & second_number,
        '|' => first_number | second_number,
        '^' => first_number ^ second_number,
        _ => panic!("Invalid operation."),
    }
}

/// Main function for the calculator.
///
/// This function will prompt the user to input two numbers and a bitwise operation (AND, OR, or
/// XOR). The program will then perform the operation on the two numbers and display the result.
pub fn calculator() {
    println!("Please enter the first number:");
    let first_number = get_number_input();

    println!("Please enter the second number:");
    let second_number = get_number_input();

    let operation = get_operation_input();

    let result = do_operation(first_number, second_number, operation);

    println!(
        "The result of {} {} {} is {}",
        first_number, operation, second_number, result
    );
}

#[cfg(test)]
mod tests {
    use crate::calculator::do_operation;
    #[test]
    fn test_bitwise_and() {
        assert_eq!(do_operation(2, 27, '&'), 2);
        assert_eq!(do_operation(2, 3, '&'), 2);
        assert_eq!(do_operation(10, 5, '&'), 0);
    }
    #[test]
    fn test_bitwise_or() {
        assert_eq!(do_operation(2, 27, '|'), 27);
        assert_eq!(do_operation(2, 3, '|'), 3);
        assert_eq!(do_operation(10, 5, '|'), 15);
    }
    #[test]
    fn test_bitwise_xor() {
        assert_eq!(do_operation(2, 27, '^'), 25);
        assert_eq!(do_operation(2, 3, '^'), 1);
        assert_eq!(do_operation(10, 5, '^'), 15);
    }
}
