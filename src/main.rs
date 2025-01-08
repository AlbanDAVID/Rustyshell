#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::fs;
use std::process::{Command, Output};
use std::str;
use std::env::set_current_dir;
use shlex::Shlex;
use std::fs::OpenOptions;

use crate::utils::file;

fn redirection(command: &str, args:&[String]) {

    let mut redirected = false;
    let mut i = 0;
    for arg in args {
        if arg.as_str() == ">" || arg.as_str() == "1>"{
            let next_arg = &args[i + 1];
            let prev_arg = &args[0..i];

            match command {
                "echo" => fs::write(next_arg, prev_arg.join(" ") +  "\n").expect("Unable to write file"),
                _ => println!("error"),
            }

            redirected = true;
            break;
        } 
        if arg.as_str() == "2>" {

            let next_arg = &args[i + 1];
            let prev_arg = &args[0..i];


            fs::write(next_arg,"").expect("Unable to write file");
            println!("{}", prev_arg.join(" "));
            redirected = true;
            break;
        }

        if arg.as_str() == ">>" || arg.as_str() == "1>>"{
            let next_arg = &args[i + 1];
            let prev_arg = &args[0..i];


            let mut file = OpenOptions::new()
                .append(true)  
                .create(true)  
                .open(&next_arg)  
                .expect("Unable to open file");


            let prev_arg = prev_arg.join(" ") + "\n";
            match command {
                "echo" => file.write_all(prev_arg.as_bytes()).expect("Unable to write to file"),
                _ => println!("error"),
            }

            redirected = true;
            break;
        } 


        if arg.as_str() == "2>>" {

            let next_arg = &args[i + 1];
            let prev_arg = &args[0..i];

            let mut file = OpenOptions::new()
                .append(true)  
                .create(true)  
                .open(&next_arg)  
                .expect("Unable to open file");

            file.write_all("".as_bytes()).expect("Unable to write to file");

            println!("{}", prev_arg.join(" "));
            redirected = true;
            break;
        }

        i += 1;
    }

    if !redirected {
        println!("{}", &args.join(" "));
    }
}


fn run_program(env_var: &str, command: &str, args:&[String]) -> bool {

    // retrieve PATH env variable 
    let path = file::get_env_var(env_var);

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

                                let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8 in stdout");
                                let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8 in stderr");
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

                                let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8 in stdout");
                                let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8 in stderr");
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
                                let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8 in stdout");
                                let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8 in stderr");
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

                                let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8 in stdout");
                                let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8 in stderr");
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
                            let custom_stdout = stdout.replace("///", "");
                            print!("{}", custom_stdout);
                            let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8 in stderr");
                            print!("{}", stderr); 
                        }


                        return true
                    }         
                } 
            } 
        } 
    } return false

}

// PWD command
fn get_pwd()-> std::io::Result<()> {
    let pwd = env::current_dir()?;
    println!("{}", pwd.display());
    Ok(())
}

// CD command
pub fn change_dir(input: &str) -> Result<(), Box<dyn std::error::Error>>  {

    //this std function handle absolute and relative path
    //`let _ = ...` to ignore the resulting value
    if input == "~" {
        let _ = set_current_dir(get_env_var("HOME"))?; 
    } else {
        let _ = set_current_dir(input)?; 
    }

    Ok(())
}


fn handle_command(input: &str) -> bool {

    // separate command and args (if any).
    // Shlex will split whitespaces and consider  everything betwenn single or doubles quotes as a
    // single command
    let split_input: Vec<String> = Shlex::new(input).collect(); // type &[String]
    let command = &split_input[0]; // type &String
    let args = &split_input[1..]; // type &[String] 

    // Match command ands args
    match command.as_str() {
        // exit
        "exit" => return true,
        // pwd
        "pwd" =>  {
            if args.is_empty() {
                match get_pwd() {
                    Ok(()) => (),
                    Err(e)=> println!("{}", e), // std error from propagation error
                }                   
            } else {
                eprintln!("pwd: expected 0 arguments; got {}", &args.len());
            }}, 
        // echo
        "echo"  => redirection(&command, &args),
        // cd
        "cd" =>  match change_dir(&args.join(" ")) {
            Ok(()) => (),
            // Err(e) => println!("{}", e), // std error from error propagation 
            Err(_) => println!("cd: {}: No such file or directory", &args.join(" ")),
        },
        // type shell builtin
        "type" => match args.join(" ").as_str() {
            "echo" => println!("echo is a shell builtin"),
            "exit" => println!("exit is a shell builtin"),
            "pwd" => println!("pwd is a shell builtin"),
            "cd" => println!("cd is a shell builtin"),
            "type" => println!("type is a shell builtin"),
            // type for program includ in PATH
            _ => match find_path("PATH", &args.join(" ")){
                Some(path_found) => println!("{} is {}", &args.join(" "), path_found),
                None => println!("{}: not found", &args.join(" ")),
            } 
        },
        // run a program
        _ => if run_program("PATH", command, args) == false {
            println!("{}: not found", &command);
        },
    }
    false
}



fn main() {

    loop { 
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        // clean input (without whitespace)
        let input = input.trim();

        // exit the loop (program) if exit cmd is called
        if handle_command(input) {
            break;
        }  
    }
}

