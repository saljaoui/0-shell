use crate::commands::utls_file::{get_current_dir, set_current_dir};
use std::env;
use std::path::Path;
use std::path::PathBuf;

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
        if !check_path(args[0]) {
            return;
        }

        match env::set_current_dir(&args[0]) {
            Ok(_) => {
                if let Ok(new_path) = env::current_dir() {
                    set_current_dir(new_path);
                }
            }
            Err(_) => {
                println!("cd: can't cd to {}", args[0]);
            }
        }
    }
}

fn check_path(args: &str) -> bool {
    let splitedargs = args.split("/").into_iter().collect::<Vec<&str>>();

    let mut current_dir: PathBuf = match get_current_dir() {
        Some(dir) => dir,
        None => return false,
    };

    for arg in splitedargs {
        if arg == ".." {
            current_dir.pop();
        } else {
            current_dir.push(arg);
        }
    }
    if current_dir.exists() && current_dir.is_dir() {
        return true;
    } else {
        println!("cd: can't cd to {}", args);
        return false;
    }
}
