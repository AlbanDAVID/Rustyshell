use shlex::Shlex;
use crate::builtin::get_pwd;
use crate::builtin::redirection;
use crate::builtin::change_dir;
use crate::find_path;
use crate::shell::runner::run_program;

// handle command
pub fn handle_command(input: &str) -> bool {

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



