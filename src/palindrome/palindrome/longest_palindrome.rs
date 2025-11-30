use crate::palindrome::check_palindrome::expand_check_palindrome;

pub fn find_longest_palindrome(s: &String) -> String {
    if s.is_empty() {
        return "".to_string();
    }

    let mut start = 0;
    let mut max_pal_size = 1;

    let len = s.len();
    let chars = s.chars().collect::<Vec<char>>();
    for i in 0..len {
        // odd length palindromes (center is single character)
        expand_check_palindrome(&chars, i, i, &mut start, &mut max_pal_size);

        // even length palindromes (center is two characters)
        if i + 1 < len {
            expand_check_palindrome(&chars, i, i + 1, &mut start, &mut max_pal_size);
        }
    }

    return s[start..start + max_pal_size].to_string();
}
