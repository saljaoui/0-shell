use std::fs;
use std::path::Path;
pub fn builtin_cd(args: &[&str])-> Result<(), Box<dyn std::error::Error>> {
    if args.len() == 0 {}
    let destination = args[args.len()-1];
    let sources = &args[..args.len()-1];
    println!("{:?} {:?} ",destination,sources);
    for src in sources {
        copy_file(src, destination)?;
    }
    Ok(())
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