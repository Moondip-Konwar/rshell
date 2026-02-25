use std::env;
use std::io::{self, Write};
use std::process::{Command, exit};

mod logging;

fn main() {
    loop {
        let input = get_input();
        process_input(&input);
    }
}

fn get_input() -> String {
    let mut input: String = String::new();
    if let Ok(current_dir) = env::current_dir()
        && let Some(path) = current_dir.to_str()
    {
        let display_str = path.to_owned() + "❯ ";
        logging::input(&display_str);
    } else {
        logging::input("$: ");
    }
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input = input.trim().to_string();

    input
}

fn process_input(input: &str) {
    if input.is_empty() {
        return;
    }

    let mut args = input.split_whitespace();
    let Some(command) = args.next() else { return };

    match command {
        // Builtins
        "exit" => exit(0),
        "cd" => {
            if let Some(dir) = args.next() {
                let _ = env::set_current_dir(dir);
            }
        }
        "pwd" => {
            if let Ok(current_dir) = env::current_dir() {
                println!("{}", current_dir.display());
            } else {
                println!("Failed to get current dir.")
            }
        }

        // Executables
        _ => {
            if let Err(e) = Command::new(command).args(args).status() {
                logging::error(format!("{command}: {e}").as_str());
            }
        }
    }
}
