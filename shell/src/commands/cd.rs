use std::env;
use std::path::Path;
use users::get_current_username;

pub fn builtin_cd(args: &[&str]) {
    if args.len() == 0 {
        let user = whoami::username();
        let path = format!("/home/{}",user);
        let root = Path::new(&path);
        env::set_current_dir(&root);
    }else{
        let _ = match env::set_current_dir(&args[0]){
            Err(e) => println!("cd: can't cd to {}",args[0]),
            Ok(f) => f,
        };
    }
}


