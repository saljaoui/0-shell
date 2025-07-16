use std::{fs, io, path::Path};

// (Keep all your existing helper functions exactly as they are)
pub fn copy_file(path_sources: &Path, destination: &str) {
    let valid = fs::copy(path_sources, destination);
    match valid {
        Ok(_) => {
            // println!("Copied  {:?} to {}", path_sources, destination)
        }
        Err(r) => {
            println!("Error to copy file {}", r)
        }
    }
}
pub fn check_file_size(path: &Path) -> io::Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len()) // Returns size in bytes
}
pub fn check_use_copy_yes_or_no(question: &str) -> bool {
    println!("{} (y/n)", question);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes"
}

pub fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)?{
        let entry = entry?;
        let ty=entry.file_type()?;
        let dest=dst.join(entry.file_name());
        if ty.is_dir(){
            copy_dir_all(&entry.path(), &dest)?;
        }else{
            fs::copy(entry.path(), dest)?;
        }
    }   

    Ok(())
}