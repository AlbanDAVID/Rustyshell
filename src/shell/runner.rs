use std::fs;
use crate::get_env_var;
use crate::shell::utils_runner::{*};

// run a program/script include in PATH. Handle redirection.
pub fn run_program(env_var: &str, command: &str, args:&[String]) -> bool {

    // retrieve PATH env variable 
    let path = get_env_var(env_var);
    // operators's definition
    let operators = [">", "1>", "2>", ">>", "1>>", "2>>"];

    // search if command is inclued in PATH. Then, decide if there is a redirection.
    for dir in path.split(':') {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if dir.to_owned() + "/" + &command == entry.path().display().to_string() {
                        let path_found = entry.file_name();
                        
                        if let Some(pos) = args.iter().position(|arg| operators.contains(&arg.as_str())) {
                            let (prev_args, rest) = args.split_at(pos);

                            if let Some(next_arg) = rest.get(1) {
                                let operator = &args[pos];

                                if let Ok(output) = execute_command(&path_found, prev_args) {
                                    let stdout = std::str::from_utf8(&output.stdout).expect("Invalid UTF-8 in stdout");
                                    let stderr = std::str::from_utf8(&output.stderr).expect("Invalid UTF-8 in stderr");
                                    handle_redirection(operator, next_arg, stdout, stderr);
                                } else {
                                    eprintln!("Error during cmd line execution");
                                }
                            }

                        } 

                        else // if no redirection 
                        {
                            if let Ok(output) = execute_command(&path_found, args) {
                                let stdout = String::from_utf8_lossy(&output.stdout);
                                print!("{}", stdout);
                                let stderr = std::str::from_utf8(&output.stderr).expect("Invalid UTF-8 in stderr");
                                print!("{}", stderr); 
                            } else {
                                eprintln!("Error during cmd line execution");
                            }
                        } 

                        return true // a program has been executed
                    }         
                } 
            } 
        } 
    } return false // program has not been executed
}


