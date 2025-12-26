use super::check_palindrome::expand_check_palindrome;
use super::dto::{LongestPalindromeInput, LongestPalindromeOutput};

pub fn find_longest_palindrome(input: LongestPalindromeInput) -> LongestPalindromeOutput {
    if input.is_empty() {
        return LongestPalindromeOutput::default();
    }

    let mut max_pal_start = 0;
    let mut max_pal_size = 1;

    let s = input.string;
    let len = s.len();
    let chars = s.chars().collect::<Vec<char>>();
    for i in 0..len {
        // odd length palindromes (center is single character)
        expand_check_palindrome(&chars, i, i, &mut max_pal_start, &mut max_pal_size);

        // even length palindromes (center is two characters)
        expand_check_palindrome(&chars, i, i + 1, &mut max_pal_start, &mut max_pal_size);
    }

    let longest_palindrome = s[max_pal_start..max_pal_start + max_pal_size].to_string();

    LongestPalindromeOutput::new(s.to_string(), longest_palindrome)
}
