use std::io::{self, Write};

pub fn write_to_console(s: &str) {
    print!("{}", s);
    if let Err(e) = io::stdout().flush() {
        eprintln!("Error flushing stdout: {}", e);
    }
}
