use std::process::{Command, Stdio};
use std::fs;
use std::io::{Write};

#[test]
fn test_ls_redirection_to_file() {
    let file_name = "test_output.txt";
    if fs::metadata(file_name).is_ok() {
        fs::remove_file(file_name).expect("Échec de la suppression du fichier de test existant");
    }

    // run interactive shell
    let mut child = Command::new("./target/debug/Rustyshell") 
        .stdin(Stdio::piped()) 
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn() 
        .expect("Échec du démarrage du shell interactif");

    // simule command line  
    let stdin = child.stdin.as_mut().expect("Échec d'accès à stdin");
    writeln!(stdin, "echo test").expect("Échec de l'écriture dans stdin");


    // wait run end and print
    let output = child.wait_with_output().expect("Échec de l'attente de la fin du programme");
    let stdout = String::from_utf8_lossy(&output.stdout);

    println!("{}", stdout);

    assert_eq!(stdout, "🇷 test\n🇷 ");
}

