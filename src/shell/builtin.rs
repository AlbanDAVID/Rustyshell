use std::fs;
use std::fs::OpenOptions;
use std::env;
use std::env::set_current_dir;
use crate::get_env_var;
use std::io::Write;

// ECHO redirection
pub fn redirection(command: &str, args:&[String]) {

    let mut redirected = false;
    let mut i = 0;
    for arg in args {
        if arg.as_str() == "3>" || arg.as_str() == "1>"{
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

// PWD command
pub fn get_pwd()-> std::io::Result<()> {
    let pwd = env::current_dir()?;
    println!("{}", pwd.display());
    Ok(())
}

// CD command
pub fn change_dir(input: &str) -> Result<(), Box<dyn std::error::Error>>  {

    //this std function handle absolute and relative path
    //`let _ = ...` to ignore the resulting value
    // `?` is the propagation error. If the dir is wrong during cd,
    // std error of std function `set_current_dir` will appear. 
    // it's also possible to custom the message error (see, in handle_command when `change_dir` is
    // called)
    if input == "~" {
        let _ = set_current_dir(get_env_var("HOME"))?; 
    } else {
        let _ = set_current_dir(input)?; 
    }

    Ok(())
}

