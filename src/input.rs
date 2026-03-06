use std::env;

use crate::Shell;
use colored_text::Colorize;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};

impl Shell {
    // Fetches Raw Mode input
    // Keeps fectching characters until <Enter> or <Ctrl + C> is pressed
    // Everytime a character is fetched, it is also printed on screen due to
    // the typed text being invsible due to raw mode
    // TODO: Fix this mess of a comment
    fn _get_input_string(&mut self) -> String {
        let mut input = String::new();
        loop {
            // Get key event
            let Event::Key(key) = event::read().unwrap() else {
                continue;
            };

            // Handle <Ctrl + C>
            if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                // TODO: Currently simply quits the shell
                // Do something else
                Shell::exit();
            }

            // Handle keys
            // TODO: Handle Left, Right, Up, Down, etc
            match key.code {
                KeyCode::Enter => {
                    input += "\n";
                    break;
                }
                KeyCode::Backspace => {
                    input.pop();
                    // TODO: Implement stdout backspace
                }

                KeyCode::Char(c) => {
                    let char = c.to_string();
                    self.print_stdout(&char);
                    input += &char;
                }
                _ => todo!("Keycode {} not handled", key.code),
            }
        }
        input
    }

    // Displays an input prompt `current/path [active branch]❯ `
    // and then fecthes and returns the user input
    pub fn fetch_input(&mut self) -> String {
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
        display_str = "\n".to_string() + &display_str + "❯ ";
        self.print_stdout(&display_str);

        // Get input
        let mut input = self._get_input_string();
        input = input.trim().to_string();

        input
    }
}
