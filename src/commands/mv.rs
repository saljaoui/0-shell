use std::{fs, io, path::Path};

use crate::commands::utls_file::copy_dir_all;

pub fn builtin_mv(args: &[&str]) {
    if args.len() < 2 {
        println!("mv: missing file operand");
        return;
    }

    let destination = args[args.len() - 1];
    let sources = &args[..args.len() - 1];
    let dest_path = Path::new(destination);

    // Handle multiple sources
    if sources.len() > 1 {
        if !dest_path.exists() {
            println!("mv: target '{}' is not a directory", destination);
            return;
        }
        if !dest_path.is_dir() {
            println!(
                "mv: cannot overwrite non-directory '{}' with multiple sources",
                destination
            );
            return;
        }
    }

    let mut has_errors = false;
    for source in sources {
        match move_single_file(source, dest_path) {
            Ok(_) => {}
            Err(e) => {
                has_errors = true;
                match e.kind() {
                    io::ErrorKind::NotFound => {
                        println!("mv: cannot stat '{}': No such file or directory", source);
                        return;
                    }
                    io::ErrorKind::InvalidInput => {
                        if dest_path.exists() && !dest_path.is_dir() {
                            println!(
                                "mv: cannot overwrite non-directory '{}' with directory '{}'",
                                destination, source
                            );
                        } else {
                            println!("mv: {}", e);
                        }
                    }
                    _ => println!("mv: {}", e),
                }
            }
        }
    }

    if has_errors {
        std::process::exit(1);
    }
}

fn move_single_file(sources: &str, path_destination: &Path) -> io::Result<()> {
    let path_source = Path::new(sources);
    // println!("{} {}", path_destination.display(), path_source.display());
    if !path_source.exists() {
        let meta = match fs::symlink_metadata(&path_source) {
            Ok(m) => m,
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("mv: cannot stat '{}': No such file or directory", sources),
                ))
            }
        };
        if meta.is_symlink() {
            if path_source.is_file() {
                copy_dir_all(path_source, path_destination)?;
                fs::remove_file(path_source)?;
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("mv: cannot stat '{}': No such file or directory", sources),
            ));
        }
    }
    if path_destination.is_dir() {
        let path = path_source.file_name();
        match path {
            Some(o) => {
                let new_path = path_destination.join(o);
                let rename = fs::rename(sources, new_path);
                match rename {
                    Err(err) => {
                        println!("'{}'", err);
                    }
                    _ => {}
                }
            }
            None => {
                println!(
                    "mv: cannot move '{}' to '{}': Device or resource busy",
                    sources,
                    path_destination.display()
                );
            }
        }
    } else {
        match fs::rename(sources, path_destination) {
            Ok(()) => {
                // println!("File moved successfully!");
            }
            Err(error) => {
                // println!("Error moving file: {}", error);
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
