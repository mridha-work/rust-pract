mod core;
mod stdio;

fn main() {
    loop {
        stdio::output::write_to_console("press ctrl+C to exit.\n");
        core::run_longest_palindrome();
    }
}
