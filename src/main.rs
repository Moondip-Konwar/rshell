use std::env;
use std::io::{self};
use std::process::{Command, exit};

mod logging;
mod tests;

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

fn push_till(chars: &[char], from: usize, till: char) -> (String, usize) {
    let mut i: usize = from;
    let mut substr = String::new();
    while i < chars.len() && chars[i] != till {
        substr += chars[i].to_string().as_str();
        i += 1;
    }

    (substr, i)
}

fn parse_input(input: &str) -> (String, Vec<String>) {
    let mut args: Vec<String> = Vec::new();
    let chars: Vec<char> = input.trim_start().chars().collect();

    // Parse command
    let (command, mut i) = push_till(&chars, 0, ' ');

    // Parse args
    while i < chars.len() {
        // Skip whitespaces
        if chars[i] == ' ' {
            i += 1;
            continue;
        }
        let arg: String;

        // Parse double quotes string
        if chars[i] == '"' {
            i += 1; // Skip the starting "
            (arg, i) = push_till(&chars, i, '"');

            i += 1; // Skip the ending "
            args.push(arg);
            continue; // Go back to loop start
        }

        // Parse double quotes string
        if chars[i] == '\'' {
            i += 1; // Skip the starting '
            (arg, i) = push_till(&chars, i, '\'');

            i += 1; // Skip the ending '
            args.push(arg);
            continue; // Go back to loop start
        }

        // Parse flag
        (arg, i) = push_till(&chars, i, ' ');
        args.push(arg);
        continue;
    }

    (command, args)
}
fn process_input(input: &str) {
    if input.is_empty() {
        return;
    }

    let (cmd, args) = parse_input(input);
    let command = cmd.as_str();

    match command {
        // Builtins
        "exit" => exit(0),
        "cd" => {
            if let Some(dir) = args.first() {
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
