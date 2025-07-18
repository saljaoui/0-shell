use crate::commands::utls_file::{get_current_dir, set_current_dir};
use std::env;
use std::path::Path;

pub fn builtin_cd(args: &[&str]) {
    if args.len() == 0 || args[0] == "~" {
        let user = whoami::username();
        let path = format!("/home/{}", user);
        let root = Path::new(&path);
        if env::set_current_dir(&root).is_ok() {
            if let Ok(current) = env::current_dir() {
                set_current_dir(current);
            }
        } else {
            println!("cd: failed to change to home directory");
        }
    } else {
        match env::set_current_dir(&args[0]) {
            Ok(_) => {
                if let Ok(new_path) = env::current_dir() {
                    set_current_dir(new_path);
                }
            }
            Err(_) => {
                println!("cd: no such file or directory: {}", args[0]);
            }
        }
    }
}
