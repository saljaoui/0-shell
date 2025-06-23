// use std::fs::remove_file;
use std::fs;


pub fn builtin_rm(args: &[&str]) {
    println!("{:?}", args);
    if args.contains(&"-r") {
    let r = fs::remove_dir(args[0]);
    println!("{:?}", r);
    println!("dir");
    } else {
    let r = fs::remove_file(args[0]);
    println!("{:?}", r);
    println!("file");
    }
    // let r = fs::remove_file(args[0]);
    // println!("{:?}", r);
        

}