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

fn parse_input(input: &str) -> (String, Vec<String>) {
    let mut command = String::new();
    let mut args: Vec<String> = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    // Skip leading space
    while i < chars.len() && chars[i] == ' ' {
        i += 1;
    }

    // Parse command
    while i < chars.len() && chars[i] != ' ' {
        command += chars[i].to_string().as_str();
        i += 1;
    }

    // Parse args
    while i < chars.len() {
        if chars[i] == ' ' {
            i += 1;
            continue;
        }
        let mut arg = String::new();

        // Parse double quotes string
        if chars[i] == '"' {
            i += 1; // Skip the starting "
            while i < chars.len() && chars[i] != '"' {
                arg += chars[i].to_string().as_str();
                i += 1;
            }

            i += 1; // Skip the ending "
            args.push(arg);
            continue; // Go back to loop start
        }

        // Parse double quotes string
        if chars[i] == '\'' {
            i += 1; // Skip the starting '
            while i < chars.len() && chars[i] != '\'' {
                arg += chars[i].to_string().as_str();
                i += 1;
            }

            i += 1; // Skip the ending '
            args.push(arg);
            continue; // Go back to loop start
        }

        // Parse flag
        while i < chars.len() && chars[i] != ' ' {
            arg += chars[i].to_string().as_str();
            i += 1;
        }
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
