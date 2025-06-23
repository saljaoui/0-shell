use std::path::Path;
use std::fs;
pub fn builtin_mkdir(args: &[&str]){
    for arg in args{
        match Path::new(arg).is_dir(){
            true => {
                println!("mkdir: cannot create directory {}: File exists",arg)
                
            },
            false => fs::create_dir(arg).expect("REASON"),
        }    
    }
}