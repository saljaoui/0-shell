use crate::commands::utls_file::get_current_dir;


pub fn builtin_pwd(args: &[&str]){
    if args.len() > 0 {
        println!("pwd: too many arguments");
        return
    }
    println!("{:?}",get_current_dir());
}