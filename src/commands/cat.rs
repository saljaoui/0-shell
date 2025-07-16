use std::io::{self};
use std::fs;

pub fn builtin_cat(args: &[&str]){
    unsafe {
        signal(2,signal_handler);
    }
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
                    print!("{}",text)
                } 
                Err(_) =>{
                    println!("cat: {}: No such file or directory",arg);
                } 
            }    
        }
    }
}



unsafe extern "C" {
    fn signal(signal: i32, handler: extern "C" fn(i32));
}
extern "C" fn signal_handler(_signal: i32) {
    // println!("tttttt");
    std::process::exit(0);
}