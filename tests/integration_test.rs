use std::process::{Command, Stdio};
use std::fs;
use std::io::{Write};
use std::process::Child;


////////////////////////////////////////////////////////////////////////////////
// to run integration test in local : sudo docker-compose run --rm rustyshell //
////////////////////////////////////////////////////////////////////////////////

// interactive shell function for testing
fn interactive_shell(bin_path: &str, cmds: Vec<&str>) -> Result<Child, std::io::Error> {
    let mut child = Command::new(bin_path) 
        .stdin(Stdio::piped()) 
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn() 
        .expect("Failed to start interactive shell");

    // command line  
    let stdin = child.stdin.as_mut().expect("Failed to access stdin");
    for cmd in cmds {
    writeln!(stdin, "{}", cmd).expect("Failed to write to stdin");
    } 

    Ok(child)
}

#[test]
fn test_command() {

    let cmds: Vec<&str> = vec!["echo blue", "pwd"];
    if let Ok(child) =  interactive_shell("./target/debug/Rustyshell", cmds) {
    // wait end run end and print
    let output = child.wait_with_output().expect("Failed to wait for program to complete");
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);
    assert_eq!(stdout, "🇷 blue\n🇷 /app\n🇷 ");
    }
}

#[test]
fn test_redirection() {

    let cmds: Vec<&str> = vec!["echo green > test", "ls >> test", "cat test"];
    if let Ok(child) =  interactive_shell("./target/debug/Rustyshell", cmds) {
    // wait end run end and print
    let output = child.wait_with_output().expect("Failed to wait for program to complete");
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);
    assert!(fs::read_to_string("test").expect("REASON").contains("green"));
    // remove test file
    let _ = fs::remove_file("test");
    }
}
