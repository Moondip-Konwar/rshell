use std::collections::HashMap;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, exit};
use std::{env, fs};

fn main() {
    let path = env::var("PATH").expect("Failed to read path.");
    let path_dirs = path.split(':');
    let mut executables: HashMap<String, PathBuf> = HashMap::new();
    for dir in path_dirs {
        let dir_path = Path::new(dir);
        let files = get_files_in(dir_path);
        executables.extend(files);
    }
    loop {
        let input = get_input();
        process_input(&input, &executables);
    }
}

fn get_input() -> String {
    let mut input: String = String::new();
    print!("$: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input = input.trim().to_string();

    input
}

fn get_files_in(path: &Path) -> HashMap<String, PathBuf> {
    let mut files: HashMap<String, PathBuf> = HashMap::new();
    let Ok(entries) = fs::read_dir(path) else {
        return files;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            let name = entry.file_name();
            files.insert(name.into_string().unwrap(), path);
        }
    }

    files
}

fn process_input(input: &str, executables: &HashMap<String, PathBuf>) {
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
            if let Some(path) = executables.get(command) {
                if let Err(e) = Command::new(path).args(args).status() {
                    eprintln!("{}: {}", command, e);
                }
            } else {
                println!("{command}: command not found.")
            }
        }
    }
}
