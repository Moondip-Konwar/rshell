# rshell

A lightweight command-line shell written in Rust.

## Features

* **Command Execution**: Run system binaries and external programs.
* **Built-in Commands**: Includes native support for `cd`, `pwd`, and `exit`.
* **Dynamic Prompt**: Displays the current working directory.
* **Git Integration**: Automatically detects and displays the current Git branch in the prompt with color coding.
* **Input Parsing**: Handles basic tokenization of commands and arguments.

## Project Structure

The project is organized into modular components:

* `main.rs`: The core REPL (Read-Eval-Print Loop) and execution logic.
* `parser.rs`: Logic for splitting input strings into executable commands and arguments.
* `logging.rs`: Internal utility for formatted output and error handling.
* `tests.rs`: Suite for verifying shell functionality.

## Installation

Ensure you have the Rust toolchain installed, then clone the repository and build:

```bash
cargo build --release

```

## Usage

Run the shell directly using cargo:

```bash
cargo run

```

Once inside, you will see a prompt showing your path and Git branch:

```text
/home/user/project master ❯ ls -l

```


