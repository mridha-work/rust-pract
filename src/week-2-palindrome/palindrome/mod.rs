use crate::read;
pub mod longest_palindrome;

pub fn run_longest_palindrome() {
    print!("Enter a string to find the longest palindrome: ");
    let s = read::from_stdin();

    let longest_palindrome = longest_palindrome::find_longest_palindrome(&s);

    println!("---------");   
    println!("Input string: {}", s);
    println!("Input size: {}", s.len());
    println!("Longest palindrome: {}", longest_palindrome);
    println!("Longest palindrome size: {}", longest_palindrome.len());
}
