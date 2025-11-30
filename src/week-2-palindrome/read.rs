use std::io::{self, Write};

pub fn from_stdin() -> String {
    if let Err(e) = io::stdout().flush() {
        eprintln!("Error flushing stdout: {}", e);
        return String::new();
    }

    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");
    
    return s.trim().to_string();
}
