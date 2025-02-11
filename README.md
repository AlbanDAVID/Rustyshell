[![Rustyshell](https://github.com/AlbanDAVID/Rustyshell/actions/workflows/rust.yml/badge.svg)](https://github.com/AlbanDAVID/Rustyshell/actions/workflows/rust.yml)
# Rustyshell

Rustyshell is a project for learning Rust by implementing a (very) basic shell.  
I try to replicate some functionalities of a shell like Bash.  

For guidance, I refer to the [Bash Reference Manual](https://www.gnu.org/software/bash/manual/bash.html).

---

## Features

Rustyshell currently supports:

### 1. Built-in Commands
The following built-in commands are implemented:
- **`exit`**: Exit the shell.
- **`pwd`**: Print the current working directory.
- **`echo`**: Print text to standard output.
- **`cd`**: Change the current directory.
- **`type`**: Display information about command types.

### 2. Program Execution
Rustyshell can execute external programs or scripts present in the system's `PATH` and print their `stdout`/`stderr`.

### 3. Redirection and Quoting
The shell supports:

- Redirection operators:  `>, 2>, >>, 2>>`
- Quoting to handle spaces and special characters.

---

