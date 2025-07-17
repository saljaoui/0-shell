use std::{fs, io, path::Path};

use crate::commands::utls_file::copy_dir_all;

pub fn builtin_mv(args: &[&str]) {
    if args.is_empty() {
        return;
    }

    let destination = args[args.len() - 1];
    let sources = &args[..args.len() - 1];
    let path_destination = Path::new(destination);
    match move_single_file(sources[0], path_destination) {
        Ok(_) => {
            // Ok(())
        }
        Err(err) => println!("{}", err),
    }
}

fn move_single_file(sources: &str, path_destination: &Path) -> io::Result<()> {
    let path_source = Path::new(sources);
    if path_destination == Path::new(".") || path_destination == Path::new("./") {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                " mv: cannot overwrite non-directory '{}' doesn't exist {}",
                path_destination.display(),
                sources
            ),
        ));
    }
    if !path_source.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("mv: cannot stat '{}': No such file or directory", sources),
        ));
    }
    if path_destination.is_dir() {
        let path = path_source.file_name();
        match path {
            Some(o) => {
                let new_path = path_destination.join(o);
                match fs::rename(sources, new_path) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{e}");
                    }
                };
            }
            None => {
                println!(
                    "mv: cannot move '{}' to '{}': Device or resource busy",
                    sources,
                    path_destination.display()
                )
            }
        }
    } else {
        match fs::rename(sources, path_destination) {
            Ok(()) => {
                // println!("File moved successfully!");
            }
            Err(error) => {
                println!("Error moving file: {}", error);
                return Err(error);
            }
        }
    }
    if path_source.is_file() {
        copy_dir_all(path_source, path_destination)?;
        fs::remove_file(path_source)?;
    }
    Ok(())
}
