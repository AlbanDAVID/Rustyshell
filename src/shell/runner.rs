use crate::get_env_var;
use std::fs;
use std::process::Output;
use std::process::Command;
use std::fs::OpenOptions;
use std::io::Write;

// run a program/script include in PATH
pub fn run_program(env_var: &str, command: &str, args:&[String]) -> bool {

    // retrieve PATH env variable 
    let path = get_env_var(env_var);

    // run program
    for dir in path.split(':') {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    // Here, `entry` is a `DirEntry`.
                    if dir.to_owned() + "/" + &command == entry.path().display().to_string() {
                        let path_found = entry.file_name();
                        let mut redirected = false;
                        let mut i = 0;
                        for arg in args {
                            if arg.as_str() == ">" || arg.as_str() == "1>" {
                                let next_arg = &args[i + 1];
                                let args_stdout = &args[0..i];

                                let output: Output = Command::new(&path_found)
                                    .args(args_stdout) // Pass args as a slice of &str
                                    .output() // Execute the program and capture the output
                                    .expect("Failed to execute program");

                                let stdout = std::str::from_utf8(&output.stdout).expect("Invalid UTF-8 in stdout");
                                let stderr = std::str::from_utf8(&output.stderr).expect("Invalid UTF-8 in stderr");
                                let custom_stderr = stderr.replace("/usr/bin/", "");
                                print!("{}", custom_stderr);
                                //println!("{:?}", next_arg);
                                fs::write(next_arg, stdout).expect("Unable to write file");
                                redirected = true;
                                break;
                            } 
                            else if arg.as_str() == "2>" {

                                let next_arg = &args[i + 1];
                                let args_stdout = &args[0..i];

                                let output: Output = Command::new(&path_found)
                                    .args(args_stdout) // Pass args as a slice of &str
                                    .output() // Execute the program and capture the output
                                    .expect("Failed to execute program");

                                let stdout = std::str::from_utf8(&output.stdout).expect("Invalid UTF-8 in stdout");
                                let stderr = std::str::from_utf8(&output.stderr).expect("Invalid UTF-8 in stderr");
                                print!("{}", stdout);
                                //print!("{}", custom_stderr);
                                //println!("{:?}", next_arg);
                                fs::write(next_arg, stderr).expect("Unable to write file");
                                redirected = true;
                                break;

                            }
                            else if arg.as_str() == ">>" || arg.as_str() == "1>>" { 


                                let next_arg = &args[i + 1];
                                let args_stdout = &args[0..i];

                                let output: Output = Command::new(&path_found)
                                    .args(args_stdout) 
                                    .output() 
                                    .expect("Failed to execute program");

                                let mut file = OpenOptions::new()
                                    .append(true)  
                                    .create(true)  
                                    .open(&next_arg)  
                                    .expect("Unable to open file");
                                let stdout = std::str::from_utf8(&output.stdout).expect("Invalid UTF-8 in stdout");
                                let stderr = std::str::from_utf8(&output.stderr).expect("Invalid UTF-8 in stderr");
                                let custom_stderr = stderr.replace("/usr/bin/", "");
                                //print!("{}", stdout);
                                print!("{}", custom_stderr);
                                //println!("{:?}", next_arg);
                                file.write_all(stdout.as_bytes())
                                    .expect("Unable to write to file");
                                redirected = true;
                                break;
                            }

                            else if arg.as_str() == "2>>" {

                                let next_arg = &args[i + 1];
                                let args_stdout = &args[0..i];

                                let output: Output = Command::new(&path_found)
                                    .args(args_stdout) 
                                    .output() 
                                    .expect("Failed to execute program");

                                let mut file = OpenOptions::new()
                                    .append(true)  
                                    .create(true)  
                                    .open(&next_arg)  
                                    .expect("Unable to open file");

                                let stdout = std::str::from_utf8(&output.stdout).expect("Invalid UTF-8 in stdout");
                                let stderr = std::str::from_utf8(&output.stderr).expect("Invalid UTF-8 in stderr");
                                print!("{}", stdout);
                                //print!("{}", custom_stderr);
                                //println!("{:?}", next_arg);
                                file.write_all(stderr.as_bytes())
                                    .expect("Unable to write to file");
                                redirected = true;
                                break;

                            }

                            i += 1;
                        }

                        if !redirected {
                            let output = Command::new(&path_found)
                                .args(args)
                                .output()
                                .expect("Failed to execute program");

                            let stdout = String::from_utf8_lossy(&output.stdout);
                            print!("{}", stdout);
                            let stderr = std::str::from_utf8(&output.stderr).expect("Invalid UTF-8 in stderr");
                            print!("{}", stderr); 
                        }


                        return true
                    }         
                } 
            } 
        } 
    } return false

}

