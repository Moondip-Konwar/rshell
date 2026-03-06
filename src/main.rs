use crossterm::terminal::enable_raw_mode;
use parser::parse_input;
use std::env;
use std::io::Write;
use std::io::{self, Result, Stdout};
use std::process::{Command, exit};

mod input;
mod parser;
mod tests;

struct Shell {
    stdout: Stdout,
}

impl Shell {
    // Sets up the shell
    fn new() -> Self {
        enable_raw_mode().expect("Failed to enable raw mode."); // FIXME: Disable raw mode
        Self {
            stdout: io::stdout(),
        }
    }

    // FIXME: Remove the extra white-line that magically appears
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
                self.print_stdout("\n");
                if let Err(e) = Command::new(command).args(args).status() {
                    let msg = format!("\nERROR: {}", e);
                    self.print_stdout(&msg);
                }
            }
        }
    }

    // Main Fetch-Decode-Execute loop
    fn run(&mut self) {
        loop {
            let input = self.fetch_input();
            let (cmd, args) = parse_input(&input);
            if cmd.is_empty() {
                continue;
            }
            self.process_input(&cmd, args);
        }
    }
}

fn main() {
    let mut shell = Shell::new();
    shell.run();
}
