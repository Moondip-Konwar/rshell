use std::io::{self, Write};

use colored_text::Colorize;

pub fn error(msg: &str) {
    let output = format!("ERROR: {msg}").red();
    println!("{output}")
}

pub fn input(msg: &str) {
    let output = msg.blue();
    print!("{output}");
    io::stdout().flush().unwrap();
}
