use std::fs;
use std::path::Path;

pub fn builtin_mkdir(args: &[&str]) {
    for arg in args {
        let path = Path::new(arg);
        if path.exists() {
            println!("mkdir: cannot create directory '{}': File exists", arg);
        } else {
            match fs::create_dir(path) {
                Ok(_) => {}
                Err(e) => {
                    let error = e.to_string();
                    let error_clean = error.split(" (os error").next().unwrap_or(&error);
                    println!("mkdir: cannot create directory '{}': {}", arg, error_clean);
                }
            }
        }
    }
}
