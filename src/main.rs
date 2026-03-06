use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use parser::parse_input;
use std::env;
use std::io::Write;
use std::io::{self, Result, Stdout};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio, exit};

mod input;
mod parser;
mod tests;

struct Shell {
    stdout: Stdout,
    home_path: PathBuf,
}

impl Shell {
    // Sets up the shell
    fn new() -> Self {
        enable_raw_mode().expect("Failed to enable raw mode.");
        Self {
            stdout: io::stdout(),
            home_path: env::home_dir().expect("Failed to retrieve home path."),
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

    // Executes the respective command
    fn process_input(&mut self, command: &str, args: Vec<String>) {
        match command {
            // Builtins
            "exit" => Shell::exit(),
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
                let child = Command::new(command)
                    .args(args)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn();

                match child {
                    Ok(mut child) => {
                        // Take the stdout/stderr handles
                        let stdout = child.stdout.take().unwrap();
                        let reader = BufReader::new(stdout);

                        // Process line by line
                        for line_content in reader.lines().map_while(Result::ok) {
                            self.print_stdout(&("\n".to_owned() + &line_content));
                        }

                        // Wait for the process to actually exit
                        let _ = child.wait();
                    }
                    Err(e) => {
                        self.print_stdout(&format!("\r\nERROR {command}: {e}\r\n"));
                    }
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

    // Disables raw mode and terminates the program
    fn exit() {
        disable_raw_mode().expect("Failed to disable raw mode.");
        exit(0);
    }
}

fn main() {
    let mut shell = Shell::new();
    shell.run();
}
