pub fn expand_check_palindrome(
    chars: &Vec<char>,
    mut left: usize,
    mut right: usize,
    max_pal_start: &mut usize,
    max_pal_size: &mut usize,
) {
    let len = chars.len();
    if left >= len || right >= len {
        return;
    }

    while chars[left] == chars[right] {
        let current_pal_size = right - left + 1;
        if current_pal_size > *max_pal_size {
            *max_pal_size = current_pal_size;
            *max_pal_start = left;
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
