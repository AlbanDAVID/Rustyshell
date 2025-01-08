use std::env;
use std::fs;

// retrieves the contents of an environment variable
pub fn get_env_var(env_var: &str) -> String {

    match env::var(env_var) {
        Ok(val) =>  return val,
        Err(e) => return e.to_string(),
    } 
}

// Browse directories and get path (if input is an available program/script in PATH)
pub fn find_path(env_var: &str, input: &str) -> Option<String> {

    // retrieve PATH env variable 
    let path = get_env_var(env_var);

    // Browse directories and get path (if input is an available program in PATH)
    for dir in path.split(':') {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    // Here, `entry` is a `DirEntry`.
                    if dir.to_owned() + "/" + input == entry.path().display().to_string() {
                        let path_found = entry.path().display().to_string();
                        return Some(path_found);
                    }         
                } 
            } 
        } 
    } None // if 'input' is not an available program in PATH

}
