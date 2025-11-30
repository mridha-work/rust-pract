pub fn find_longest_palindrome(s: &String) -> String {
    if s.is_empty() {
        return "".to_string();
    }

    let mut start = 0;
    let mut max_len = 1;

    let chars: Vec<char> = s.chars().collect();
    let len: usize = chars.len();
    for i in 0..len {
        // odd length palindromes (center is single character)
        expand_check_palindrome(&chars, i, i, &mut start, &mut max_len);
        
        // even length palindromes (center is two characters)
        if i + 1 < len {
            expand_check_palindrome(&chars, i, i+1, &mut start, &mut max_len);
        }
    }
    
    return s[start..start + max_len].to_string();
}

fn expand_check_palindrome(chars: &Vec<char>, left: usize, right: usize, start: &mut usize, max_len: &mut usize) {
    let len: usize = chars.len();
    if left >= len || right >= len {
        return;
    }
    
    let mut left = left;
    let mut right = right;
    while chars[left] == chars[right] {
        let current_len = right - left + 1;
        if current_len > *max_len {
            *max_len = current_len;
            *start = left;
        }

        if left == 0 {
            break;
        }
        left -= 1;

        if right >= len - 1 {
            break;
        }
        right += 1;
    }
}
