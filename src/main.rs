use std::io::{self, Write};
use std::process::exit;

fn main() {
    loop {
        let input = get_input();
        process_input(input);
    }
}

fn get_input() -> String {
    let mut input: String = String::new();
    print!("$ ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input = input.trim().to_string();

    input
}

fn process_input(input: String) {
    if input.is_empty() {
        return;
    }

    match input.as_str() {
        "exit" => exit(0),
        "cd" => println!("Not implemented yet."),
        _ => {
            println!("{}: Command not found.", input)
        }
    }
}
