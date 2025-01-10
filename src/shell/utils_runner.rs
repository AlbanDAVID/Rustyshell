use std::fs;
use crate::shell::utils_runner::fs::OpenOptions;
use std::io::Write;
use std::process::Output;
use std::process::Command;
use std::ffi::OsString;


// handle redirection
pub fn handle_redirection(operator :&str, next_arg: &String, stdout: &str, stderr: &str) {
    match operator {
        ">" | "1>" => {  
            let custom_stderr = stderr.replace("/usr/bin/", "");
            print!("{}", custom_stderr);
            //println!("{:?}", next_arg);
            fs::write(next_arg, stdout).expect("Unable to write file");
        }, 

        "2>" => { print!("{}", stdout);
            //print!("{}", custom_stderr);
            //println!("{:?}", next_arg);
            fs::write(next_arg, stderr).expect("Unable to write file");

        },

        ">>" | "1>>" => { 
            let mut file = OpenOptions::new()
                .append(true)  
                .create(true)  
                .open(&next_arg)  
                .expect("Unable to open file");

            let custom_stderr = stderr.replace("/usr/bin/", "");
            print!("{}", custom_stderr);
            file.write_all(stdout.as_bytes())
                .expect("Unable to write to file");
            },

        "2>>" => { 
            let mut file = OpenOptions::new()
                .append(true)  
                .create(true)  
                .open(&next_arg)  
                .expect("Unable to open file");

            print!("{}", stdout);
            file.write_all(stderr.as_bytes())
                .expect("Unable to write to file");

            },

        _ => {} 
    }
} 

// execute command without redirection
pub fn execute_command(path_found: &OsString, args: &[String]) -> Result<Output, std::io::Error>{
    let output: Output = Command::new(&path_found)
        .args(args)
        .output()
        .expect("Failed to execute program");

    Ok(output)
}

