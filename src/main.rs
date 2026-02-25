use std::collections::HashMap;
use std::io::Result;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::{env, fs};

fn main() {
    let path = env::var("PATH").expect("Failed to read path.");
    loop {
        let input = get_input();
        process_input(&input, &path);
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

fn get_files_in(path: &Path) -> Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        }
    }

    Ok(files)
}

fn process_input(input: &String, path: &String) {
    if input.is_empty() {
        return;
    }

    let path_dirs = path.split(':');
    let mut executables: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
    for dir in path_dirs {
        let dir_path = Path::new(dir);
        let files = get_files_in(dir_path).unwrap();
        executables.insert(PathBuf::from(dir_path), files);
    }

    match input.as_str() {
        // Builtins
        "exit" => exit(0),
        "cd" => println!("Not implemented yet."),
        "path" => println!("{}", path),

        // Invalid
        _ => {
            println!("{}: Command not found.", input)
        }
    }
}
