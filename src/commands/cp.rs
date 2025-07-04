use std::fs;
use crate::commands::cp::fs::OpenOptions;
use std::path::Path;
 use std::io::Write;
pub fn builtin_cd(args: &[&str])-> Result<(), Box<dyn std::error::Error>> {
    if args.len() == 0 {}
    let destination = args[args.len()-1];
    let sources = &args[..args.len()-1];

     let mut dest_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)  // Clear existing content first
        .open(destination)?;

    for src in sources {
        let content = std::fs::read_to_string(src)?;
        dest_file.write_all(content.as_bytes())?;
    }
    Ok(())
    // println!("{:?} {:?} ",destination,sources);
    // for src in sources {
    //     copy_file(src, destination)?;
    // }
    // Ok(())
}


fn copy_file(src: &str, dst: &str) -> Result<(), String> {
    let src_path = Path::new(src);
    let dst_path = Path::new(dst);

    if !src_path.exists() {
        return Err(format!("cp: cannot stat '{}': No such file or directory", src));
    }

    // Handle directory destinations
    let final_dst = if dst_path.is_dir() {
        let filename = src_path.file_name()
            .ok_or_else(|| format!("cp: cannot get filename from '{}'", src))?
            .to_str()
            .ok_or("cp: invalid filename encoding")?;
        dst_path.join(filename)
    } else {
        dst_path.to_path_buf()
    };

    fs::copy(src_path, final_dst)
        .map_err(|e| format!("cp: cannot copy: {}", e))?;

    Ok(())
}