use colored_text::Colorize;
use crossterm::terminal::enable_raw_mode;
use parser::parse_input;
use std::env;
use std::io::Write;
use std::io::{self, Result, Stdout};
use std::process::{Command, exit};

mod parser;
mod tests;

struct Shell {
    stdout: Stdout,
}

impl Shell {
    // Sets up the shell
    fn new() -> Self {
        enable_raw_mode().expect("Failed to enable raw mode."); // FIXME: Disblae raw mode
        Self {
            stdout: io::stdout(),
        }
    }

    fn print_stdout(&mut self, msg: &str) {
        let msg = msg.replace("\n", "\r\n");
        write!(self.stdout, "{msg}").expect("Failed to write to stdout.");
        self.stdout.flush().expect("Failed to flush to stdout.")
    }

    // Fetches and returns the currently active git branch as String
    fn get_current_git_branch() -> Result<String> {
        Command::new("git")
            .args(["branch", "--show-current"])
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    // Displays an input prompt `current/path [active branch]❯ `
    // and then fecthes and returns the user input
    fn get_input(&mut self) -> String {
        let mut input: String = String::new();
        let mut display_str: String = "$: ".to_string(); // Fallback

        // Current dir
        if let Ok(current_dir) = env::current_dir()
            && let Some(path) = current_dir.to_str()
        {
            display_str = path.to_owned().blue();
        }

        // Git branch
        if let Ok(current_branch) = Shell::get_current_git_branch() {
            display_str += " ";
            display_str += &current_branch.green();
        }

        // Display str
        display_str += "❯ ";
        self.print_stdout(&display_str);

        // Get input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        input = input.trim().to_string();

        input
    }

    // Executes the respective command
    fn process_input(&mut self, command: &str, args: Vec<String>) {
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
                    let msg = format!("{}", current_dir.display());
                    self.print_stdout(&msg);
                } else {
                    self.print_stdout("Failed to get current dir.")
                }
            }

            // Executables
            _ => {
                if let Err(e) = Command::new(command).args(args).status() {
                    let msg = format!("ERROR: {}", e);
                    self.print_stdout(&msg);
                }
            }
        }
    }

    // Main Fetch-Decode-Execute loop
    // TODO: Add tokenizer and decoder
    fn run(&mut self) {
        loop {
            let input = self.get_input();
            let (cmd, args) = parse_input(&input);
            self.process_input(&cmd, args);
        }
    }
}

fn main() {
    let mut shell = Shell::new();
    shell.run();
}
