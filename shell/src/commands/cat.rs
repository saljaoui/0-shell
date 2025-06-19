use std::io::{self};
use std::fs;

pub fn builtin_cat(args: &[&str]){
    //reading lines and rewriting them
    if args.len()== 0{
        loop{
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            print!("{}",input);
            if input == "" { break; }
        }
    }else if args.len() >= 1 {
        for arg in args{
            match fs::read_to_string(arg){
                Ok(text)=>{
                    println!("{:?}",text)
                } 
                Err(_) =>{
                    println!("cat: {}: No such file or directory",arg);
                } 
            }    
        }
    }
}