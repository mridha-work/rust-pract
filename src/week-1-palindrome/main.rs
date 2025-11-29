use std::io::{self, Write};

fn main() {
    print!("Enter a string for longest palindrome check: ");
    if let Err(e) = io::stdout().flush() {
        eprintln!("Error flushing stdout: {}", e);
        return;
    }
    
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");
    
    let s = s.trim();

    let longest = find_longest_palindrome(s);

    println!("---------");   
    println!("Input string: {}", s);
    println!("Longest palindrome: {}", longest);
}

fn find_longest_palindrome(s: &str) -> &str {
    if s.is_empty() {
        return "";
    }

    let bytes = s.as_bytes();
    let len  = bytes.len();
    let mut start = 0;
    let mut max_len = 1;
    
    for i in 0..len {
        // odd length palindromes (center is single character)
        let mut left = i as i32;
        let mut right = i;
        while left >= 0 && right < len && bytes[left as usize] == bytes[right] {
            let current_len = right - left as usize + 1;
            if current_len > max_len {
                max_len = current_len;
                start = left as usize;
            }
            left -= 1;
            right += 1;
        }
        
        // even length palindromes (center is two characters)
        if i + 1 < len {
            let mut left = i as i32;
            let mut right = i + 1;
            while left >= 0 && right < len && bytes[left as usize] == bytes[right] {
                let current_len = right - left as usize + 1;
                if current_len > max_len {
                    max_len = current_len;
                    start = left as usize;
                }
                left -= 1;
                right += 1;
            }
        }
    }
    
    return &s[start..start + max_len]
}
