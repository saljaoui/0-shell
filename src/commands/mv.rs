use std::{fs, io, path::Path};

use crate::commands::utls_file::{check_file_size, copy_dir_all, copy_file};

pub fn builtin_mv(args: &[&str]) {
    if args.is_empty() {
        return;
    }

    let destination = args[args.len() - 1];
    let sources = &args[..args.len() - 1];
    let path_destination = Path::new(destination);

    match sources.len() {
        1 => move_single_file(sources[0], path_destination).unwrap(),
        _ => move_multiple_files(sources, path_destination).unwrap(),
    }
}

fn move_single_file(sources: &str, path_destination: &Path) -> io::Result<()> {
    let path_source = Path::new(sources);
    if !path_source.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Source '{}' doesn't exist", sources),
        ));
    }

    if path_destination.is_dir() {
        let new_path = path_destination.join(path_source.file_name().unwrap());
        fs::rename(sources, new_path).unwrap();
    } else {
        if path_source.is_dir(){
            
        }
        fs::rename(sources, path_destination).unwrap();
    }
    if path_source.is_file() {
        println!("{}-------", sources);
        fs::copy(path_source, path_destination)?;
        fs::remove_file(path_source)?;
    } 
    Ok(())
}
fn move_multiple_files(sources: &[&str], path_destination: &Path) -> io::Result<()> {
    Ok(())
}
