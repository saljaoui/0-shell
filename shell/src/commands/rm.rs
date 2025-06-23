// use std::fs::remove_file;
use std::fs;


pub fn builtin_rm(args: &[&str]) {
    println!("{:?}", args);
    let r = fs::remove_file(args[0]);
    println!("{:?}", r);
}