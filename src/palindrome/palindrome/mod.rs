use crate::stdio::{input, output};
mod check_palindrome;
mod longest_palindrome;

pub fn run_longest_palindrome() {
    output::write_to_console("Enter a string to find the longest palindrome: ");
    let s = input::read_from_console();

    let longest_palindrome = longest_palindrome::find_longest_palindrome(&s);

    println!("---------");
    println!("Input string: {}", s);
    println!("Input size: {}", s.len());
    println!("Longest palindrome string: {}", longest_palindrome);
    println!("Longest palindrome size: {}", longest_palindrome.len());
    println!("---------");
}
