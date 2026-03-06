use colored_text::Colorize;
use parser::parse_input;
use std::env;
use std::io::{self};
use std::process::{Command, exit};

mod logging;
mod parser;
mod tests;

struct Shell {}

impl Shell {
    // Sets up the shell
    fn new() -> Self {
        Self {}
    }

    // Fetches and returns the currently active git branch as String
    // TODO: Change to Result<String>
    fn get_current_git_branch() -> String {
        Command::new("git")
            .args(["branch", "--show-current"])
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
            .unwrap_or_else(|_| "".to_string())
    }

    // Displays an input prompt `current/path [active branch]❯ `
    // and then fecthes and returns the user input
    fn get_input() -> String {
        let mut input: String = String::new();
        let mut display_str: String = "$: ".to_string(); // Fallback

        // Current dir
        // TODO: Detach into Shell::get_current_dir
        if let Ok(current_dir) = env::current_dir()
            && let Some(path) = current_dir.to_str()
        {
            display_str = path.to_owned();
        }

        // Git branch
        let current_branch = Shell::get_current_git_branch();
        if !current_branch.is_empty() {
            display_str += " ";
            display_str += &current_branch.green();
        }

        // Display str
        display_str += "❯ ";
        logging::input(&display_str);

        // Get input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        input = input.trim().to_string();

        input
    }

    // Executes the respective command
    fn process_input(command: &str, args: Vec<String>) {
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

    // Main Fetch-Decode-Execute loop
    // TODO: Add tokenizer and decoder
    fn run(&self) {
        loop {
            let input = Self::get_input();
            let (cmd, args) = parse_input(&input);
            Self::process_input(&cmd, args);
        }
    }
}

fn main() {
    let shell = Shell::new();
    shell.run();
}
