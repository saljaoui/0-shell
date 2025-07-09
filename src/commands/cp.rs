use std::path::Path;
use std::{fs, io};

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

fn handle_single_file(source: &str, destination: &Path) {
    let path_source = Path::new(source);
    
    // Validation
    if !path_source.exists() {
        println!("Path does not exist!");
        return;
    }
    if !path_source.is_file() {
        println!("{:?} is not a file", path_source);
        return;
    }

    // Handle existing destination
    if destination.exists() {
        match check_file_size(destination) {
            Ok(size) if size > 0 => {
                let question = format!("File '{}' already exists. Overwrite?", destination.display());
                if !check_use_copy_yes_or_no(&question) {
                    return;
                }
            }
            Err(e) => {
                eprintln!("Error checking file size: {}", e);
                return;
            }
            _ => {} // File exists but is empty
        }
    }

    // Perform copy
    copy_file(path_source, &destination.to_string_lossy());
}

fn handle_multiple_files(sources: &[&str], destination: &Path) {
    // Directory validation
    if !destination.is_dir() {
        println!("'{}' is not a directory", destination.display());
        return;
    }

    // Create directory if needed
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

// (Keep all your existing helper functions exactly as they are)
fn copy_file(path_sources: &Path, destination: &str) {
    let valid = fs::copy(path_sources, destination);
    match valid {
        Ok(_) => {
            println!("Copied  {:?} to {}", path_sources, destination)
        }
        Err(r) => {
            println!("Error to copy file {}", r)
        }
    }
}
fn check_file_size(path: &Path) -> io::Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len()) // Returns size in bytes
}
fn check_use_copy_yes_or_no(question: &str) -> bool {
    println!("{} (y/n)", question);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes"
}
