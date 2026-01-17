mod check_palindrome;
mod dto;
mod longest_palindrome;

use crate::stdio::{input, output};

pub fn run_longest_palindrome() {
    output::write_to_console("Enter a string to find the longest palindrome: ");
    let s = input::read_from_console();
    let input = dto::LongestPalindromeInput { string: s };

    let lpo = longest_palindrome::find_longest_palindrome(input);

    println!("{lpo}");
}
