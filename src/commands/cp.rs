use std::fs;
use std::path::Path;
use std::{ fs };

use crate::commands::utls_file::{ copy_file };

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
    if !path_source.exists() {
        println!("cp: cannot stat '{}': No such file or directory",destination.display());
        return;
    }
    if !destination.exists() {
        println!("Pcp: cannot stat '{}': No a directory",destination.display());
        return;
    }
    if !path_source.is_file() {
        println!("{:?} is not a file", path_source);
        return;
    }
    let final_destination = if destination.is_dir() {
        let filename = path_source.file_name();
        match filename {
            Some(file) => { destination.join(file) }
            None => {
                println!("Error: Could not extract filename from {:?}", path_source);
                return;
            }
        }
    } else {
        destination.to_path_buf()
    };
    copy_file(path_source, &final_destination);
}

fn handle_multiple_files(sources: &[&str], destination: &Path) {
    if !destination.is_dir() {
        println!("'{}' is not a directory", destination.display());
        return;
    }
    if !destination.exists() {
        if let Err(e) = fs::create_dir_all(destination) {
            println!("Failed to create directory: {}", e);
            return;
        }
    }

    // Copy each file
    for file in sources {
        let path = Path::new(file);
        if !path.exists() {
            println!("Error: '{}' does not exist", file);
            break;
        }

        let dest_path = destination.join(path.file_name().unwrap());
        if let Err(e) = fs::copy(path, dest_path) {
            println!("Failed to copy '{}': {}", file, e);
        }
    }
}
