use std::ops::Deref;
use std::path::Path;
use std::{fs, io};
pub fn builtin_cd(args: &[&str]) {
    if args.len() == 0 {}
    let destination = args[args.len() - 1];
    let sources = &args[..args.len() - 1];
    let path_destination = Path::new(destination);

    if path_destination.is_file() && sources.len() == 1 {
        let path_sources = Path::new(sources[0]);
        if !path_sources.exists() {
            println!("Path does not exist!");
            return;
        }
        if !path_sources.is_file() {
            println!("this is not  a file {:?}", path_sources);
            return;
        }
        match check_file_size(path_destination) {
            Ok(siz) => {
                if siz > 0 {
                    let question = format!("File '{}' already exists. Overwrite?", destination);
                    if !check_use_copy_yes_or_no(&question) {
                        return;
                    } else {
                        copy_file(path_sources, destination)
                    }
                } else {
                    copy_file(path_sources, destination)
                }
            }
            Err(e) => eprintln!("Error checking file size: {}", e),
        }
    }
    if sources.len()>1{
        if !path_destination.is_dir(){
            println!("{:?} is Not directory",path_destination);
        }
        for file in sources{
            let path =Path::new(file);
            if !path.is_file(){
                println!("{:?} is Not File",path);
            }else{
                
            }
        }
    }
}

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

// fn copy_file(src: &str, dst: &str) -> Result<(), String> {
//     let src_path = Path::new(src);
//     let dst_path = Path::new(dst);

//     if !src_path.exists() {
//         return Err(format!(
//             "cp: cannot stat '{}': No such file or directory",
//             src
//         ));
//     }

//     // Handle directory destinations
//     let final_dst = if dst_path.is_dir() {
//         let filename = src_path
//             .file_name()
//             .ok_or_else(|| format!("cp: cannot get filename from '{}'", src))?
//             .to_str()
//             .ok_or("cp: invalid filename encoding")?;
//         dst_path.join(filename)
//     } else {
//         dst_path.to_path_buf()
//     };

//     fs::copy(src_path, final_dst).map_err(|e| format!("cp: cannot copy: {}", e))?;

//     Ok(())
// }
