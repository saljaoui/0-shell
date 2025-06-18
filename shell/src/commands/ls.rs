use std::fs;
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::MetadataExt;
use users::{get_user_by_uid, get_group_by_gid};

pub fn builtin_ls(args: &[&str]){
    let mut show_hidden = false;
    let mut long_format = false;
    let mut is_dir = false;
    let mut paths = Vec::new();
    for arg in args{
        match *arg{
            "-l"=> long_format = true,
            "-a"=> show_hidden = true,
            "-F"=> is_dir = true,
            "--help"=> println!("Usage: ls [OPTION]...\nList information about the FILEs (the current directory by default).\n\nOptions:\n  -l      use a long listing format\n  -a      do not ignore entries starting with .\n  -F      append indicator (one of */=>@|) to entries\n  --help  display this help and exit"),
            _=> {
                if arg.starts_with('-'){
                    for a in arg.chars(){
                        match a {
                            'l'=> long_format  = true,
                            'a'=>  show_hidden = true,
                            'F'=> is_dir = true,
                            '-'=>{},
                            _=>{
                                println!("ls: invalid option -- '{}'\nTry 'ls --help' for more information.",a)
                            }
                        }
                    }
                }else {
                    paths.push(*arg)
                }
            }
        }
    }
    if paths.is_empty(){
        paths.push(".")
    }
    let more_paths = if paths.len() > 1 { true } else { false };
    for path in paths {
        list_directory(path, show_hidden, long_format, is_dir, more_paths);
    }

}

fn list_directory(path: &str, show_hidden: bool, long_format: bool, is_dir: bool, more_paths:bool){
    let p = PathBuf::from(path);
    let dir = fs::read_dir(p);
    let mut items: Vec<_>=  match dir{
        Ok(d)=> d.collect::<Result<Vec<_>, _>>().unwrap(),
        Err(_)=> {
            println!("ls: cannot access '{}': No such file or directory",path);
            return;
        }
    };
    let mut res = String::new();
    if more_paths{
        res.push_str(&format!("{}:\n",path))
    }
    for item in items{
        let file = item.file_name();
        let file_str = file.to_string_lossy();
        if !show_hidden && file_str.starts_with('.'){
            continue;
        }
        
        let metadata = item.metadata().unwrap();
        if long_format {
            let file_type = if metadata.is_dir() { "d" } else { "-" };
            let permissions = format!("{:o}", metadata.permissions().mode() & 0o777);
        println!("----{:?} {:?} ",get_user_by_uid(metadata.uid()),get_group_by_gid(metadata.gid()));
            let size = metadata.len();
        } else {
            if is_dir && metadata.is_dir(){
                res.push_str(&format!("\x1b[34m{}/\x1b[0m ", file_str))
            }else{
                let mut file = String::new();
                if metadata.is_dir(){
                    file = format!("\x1b[34m{}\x1b[0m ", file_str)
                }else {
                    file = format!("{} ", file_str)
                }
                res.push_str(&file)
            }
        }
    }
    println!("{}",res)
    
}