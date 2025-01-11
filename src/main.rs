#[allow(unused_imports)]
use std::io::{self, Write};
mod utils;
use crate::utils::file::{*};
mod shell;
use crate::shell::{*};
use crate::handle_command::{*};

fn main() {

    loop { 
        print!("🇷 ");
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

