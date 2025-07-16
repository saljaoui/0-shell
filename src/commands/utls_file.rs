use std::{ fs, io, path::Path };

// (Keep all your existing helper functions exactly as they are)
pub fn copy_file(path_sources: &Path, destination: &Path) {
    let valid = fs::copy(path_sources, destination);
    match valid {
        Ok(_) => { println!("Copied  {:?} to {}", path_sources, destination.display()) }
        Err(r) => { println!("Error to copy file {}", r) }
    }
}
pub fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dest = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dest)?;
        } else {
            fs::copy(entry.path(), dest)?;
        }
    }

    Ok(())
}
