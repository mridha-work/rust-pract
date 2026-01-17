use std::io;

pub fn read_from_console() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");

    s.trim().to_string()
}
