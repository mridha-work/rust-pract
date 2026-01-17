// -- input DTO --
pub struct LongestPalindromeInput {
    pub string: String,
}

impl LongestPalindromeInput {
    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }
}

// -- output DTO --
pub struct LongestPalindromeOutput {
    input: String,
    input_size: usize,
    longest_palindrome: String,
    longest_palindrome_size: usize,
}

impl LongestPalindromeOutput {
    pub fn new(input: String, longest_palindrome: String) -> Self {
        let input_size = input.len();
        let longest_palindrome_size = longest_palindrome.len();

        LongestPalindromeOutput {
            input,
            input_size,
            longest_palindrome,
            longest_palindrome_size,
        }
    }
}

impl Default for LongestPalindromeOutput {
    fn default() -> Self {
        LongestPalindromeOutput::new(String::new(), String::new())
    }
}

impl std::fmt::Display for LongestPalindromeOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "---------\n\
             Input string: '{}'\n\
             Input size: {}\n\
             Longest palindrome string: '{}'\n\
             Longest palindrome size: {}\n\
             ---------\n",
            self.input, self.input_size, self.longest_palindrome, self.longest_palindrome_size
        )
    }
}
