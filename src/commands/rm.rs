use std::fs;
use std::path::Path;
use std::io::{self, Write};


pub fn builtin_rm(args: &[&str]) {
    if args.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }

    let mut flage = false;
    let mut paths = Vec::new();
    for arg in args {
        if arg.starts_with('-') && arg.len() > 1 {
            for ch in arg.chars().skip(1) {
                if ch == 'r' {
                    flage = true
                } else {
                    println!("rm: invalid option -- '{}'", arg);
                    return;
                }
            }
        } else {
            paths.push(arg);
        }
    }

    if paths.is_empty() {
        println!("rm: missing operand");
        return;
    }

    for path_str in paths {
        let path = Path::new(path_str);
        let metadata = if !path.exists() {
            match fs::symlink_metadata(&path) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("rm: cannot access '{}': {}", path_str, e);
                    continue;
                }
            }
        } else {
            match path.metadata() {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("rm: cannot access '{}': {}", path_str, e);
                    continue;
                }
            }
        };

        if metadata.is_dir() {
            if flage {
                if let Err(e) = std::fs::remove_dir_all(path) {
                    eprintln!("rm: cannot remove '{}': {}", path_str, e);
                }
            } else {
                eprintln!("rm: cannot remove '{}': Is a directory", path_str);
            }
        } else {
            if metadata.permissions().readonly() {
                print!("rm: remove write-protected regular file '{}'? ", path_str);
                io::stdout().flush().unwrap();

                let mut answer = String::new();
                io::stdin().read_line(&mut answer).unwrap();

                if !answer.trim().to_lowercase().starts_with("y") {
                    continue;
                }
            }
            if let Err(e) = std::fs::remove_file(path) {
                eprintln!("rm: cannot remove '{}': {}", path_str, e);
            }
        }
    }
}

