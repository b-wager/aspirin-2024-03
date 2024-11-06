//! Implement the FizzBuzz algorithm.

/// Print numbers from 1 to `max_num` with FizzBuzz rules.
///
/// This function will print numbers from 1 to `max_num` with the following rules:
/// - For multiples of 3, print "Fizz" instead of the number.
/// - For multiples of 5, print "Buzz" instead of the number.
/// - For multiples of both 3 and 5, print "FizzBuzz" instead of the number.
///
/// # Examples
///
/// ```
/// print_fizz_buzz(15);
/// ```
pub fn print_fizz_buzz(max_num: u32) {
    for i in 1..=max_num {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}
