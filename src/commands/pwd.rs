 use std::env;

pub fn builtin_pwd(args: &[&str]){
    if args.len() > 0 {
        println!("pwd: too many arguments");
        return
    }
   match env::current_dir(){
        Ok(path)=> println!("{}", path.to_string_lossy()),
        Err(e)=> println!("{e}")
    };
   

}