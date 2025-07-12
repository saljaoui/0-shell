use std::path::Path;
use std::{fs};

use crate::commands::utls_file::{check_file_size, check_use_copy_yes_or_no, copy_file};

pub fn builtin_cp(args: &[&str]) {
    if args.is_empty() {
        return;
    }

    let destination = args[args.len() - 1];
    let sources = &args[..args.len() - 1];
    let path_destination = Path::new(destination);

    match sources.len() {
        1 => handle_single_file(sources[0], path_destination),
        _ => handle_multiple_files(sources, path_destination),
    }
}

fn handle_single_file(source: &str, destination: &Path){
    let path_source = Path::new(source);
    if !path_source.exists() {
        println!("Path does not exist!");
        return ;
    }
    if !path_source.is_file() {
        println!("{:?} is not a file", path_source);
        return ;
    }

    if destination.exists() {
        match check_file_size(destination) {
            Ok(size) if size > 0 => {
                let question = format!("File '{}' already exists. Overwrite?", destination.display());
                if !check_use_copy_yes_or_no(&question) {
                    return ;
                }
            }
            Err(e) => {
                eprintln!("Error checking file size: {}", e);
                return ;
            }
            _ => {} 
        }
    }
    copy_file(path_source, &destination.to_string_lossy());
}

fn handle_multiple_files(sources: &[&str], destination: &Path) {
    if !destination.is_dir() {
        println!("'{}' is not a directory", destination.display());
        return;
    }
    if !destination.exists() {
        if let Err(e) = fs::create_dir_all(destination) {
            eprintln!("Failed to create directory: {}", e);
            return;
        }
    }

    // Copy each file
    for file in sources {
        let path = Path::new(file);
        if !path.is_file() {
            println!("Skipping '{}': Not a regular file", file);
            continue;
        }
        
        let dest_path = destination.join(path.file_name().unwrap());
        if let Err(e) = fs::copy(path, dest_path) {
            eprintln!("Failed to copy '{}': {}", file, e);
        }
    }
}

