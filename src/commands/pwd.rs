use crate::commands::utls_file::get_current_dir;
use std::path::PathBuf;


pub fn builtin_pwd(args: &[&str]){
    if args.len() > 0 {
        println!("pwd: too many arguments");
        return
    }
    let current_dir:PathBuf  = match get_current_dir() {
        Some(dir) => dir,
        None => return,
    };
    println!("{}",current_dir.display());
}