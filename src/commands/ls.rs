use chrono::{DateTime, Local};
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use users::{get_group_by_gid, get_user_by_uid};
use std::os::unix::fs::FileTypeExt;

pub fn builtin_ls(args: &[&str]) {
    let mut show_hidden = false;
    let mut long_format = false;
    let mut f_type = false;
    let mut paths = Vec::new();
    for arg in args {
        match *arg {
            "-l" => long_format = true,
            "-a" => show_hidden = true,
            "-F" => f_type = true,
            "--help" => {
                println!("Usage: ls [OPTION]...\nList information about the FILEs (the current directory by default).\n\nOptions:\n  -l      use a long listing format\n  -a      do not ignore entries starting with .\n  -F      append indicator (one of */=>@|) to entries\n  --help  display this help and exit");
                return;
            }
            "-" => {
                eprintln!("ls: cannot access '-': No such file or directory");
                return;
            }
            _ => {
                if arg.starts_with('-') {
                    for a in arg.chars() {
                        match a {
                            'l' => long_format = true,
                            'a' => show_hidden = true,
                            'F' => f_type = true,
                            '-' => {}
                            _ => {
                                eprintln!("ls: invalid option -- '{}'\nTry 'ls --help' for more information.",a);
                                return;
                            }
                        }
                    }
                } else {
                    paths.push(*arg)
                }
            }
        }
    }
    if paths.is_empty() {
        paths.push(".")
    }
   let mut directorys = Vec::new();
   let mut files = Vec::new();
    for path in paths {
        let metadata = match fs::metadata(path) {
            Ok(m) => m,
            Err(e) => {
                        eprintln!("ls: cannot access '{}': {}", path, e);
                        return;
                        }
        };
        if metadata.is_dir() {
            directorys.push(path)
        } else {
            files.push(path)
        }
    }
    let more_paths = directorys.len() > 1 || !files.clone().is_empty();
    for path in files{
        list_file(path, long_format, f_type);
    }
    for path in directorys{
        list_directory(path, show_hidden, long_format, f_type, more_paths);
    }
}
fn list_file(
    path: &str,
    long_format: bool,
    f_type: bool,
) {
    let p = PathBuf::from(path);
    let metadata = match fs::symlink_metadata(&p) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("ls: cannot access '{}': {}", path, e);
            return;
        }
    };
    let file_str = match p.file_name() {
        Some(f) => f.to_string_lossy().to_string(),
        None => {
            eprintln!("ls: invalid file name '{}'", path);
            return;
        }
    };
        match print_entry(&file_str, &metadata, long_format, f_type){
            Ok(r)=> print!("{}",&r),
            Err(e)=>eprint!("{:?}",e),
        }
}
fn list_directory(
    path: &str,
    show_hidden: bool,
    long_format: bool,
    f_type: bool,
    more_paths: bool,
) {
    let p = PathBuf::from(path);
    let dir = match fs::read_dir(&p) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("ls: cannot access '{}': {}", path, e);
            return;
        }
    };

    let mut items: Vec<_> = match dir.collect::<Result<Vec<_>, _>>() {
        Ok(vec) => vec,
        Err(e) => {
            eprintln!("ls: error reading directory '{}': {}", path, e);
            return;
        }
    };

    if more_paths {
        println!("{}:", path);
    }

    items.sort_by_key(|e| e.file_name());
    let mut res = String::new();
    for item in items {
        let file_str = item.file_name().to_string_lossy().to_string();
        if !show_hidden && file_str.starts_with('.') {
            continue;
        }

        let metadata = match item.metadata() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("ls: error reading metadata: {}", e);
                continue;
            }
        };
        
        match print_entry(&file_str, &metadata, long_format, f_type){
            Ok(r)=>res.push_str(&r),
            Err(e)=>eprintln!("{:?}",e),
        }
        
    }
    println!("{}", res.trim());

}


fn format_permissions(p: String) -> String {
    let permissions = &p[(p.len() - 3)..];
     let special = if p.len() >= 4 {
       p.chars().nth(p.len() - 4).unwrap()
    } else {
        '0'
    };
    // println!("{:?}",special);
    let mut res = String::new();
    for  (i, c) in permissions.chars().enumerate() {
        let per = match c {
            '7' =>"rwx",
            '6' =>"rw-",
            '5' =>"r-x",
            '4' =>"r--",
            '3' =>"-wx",
            '2' =>"-w-",
            '1' =>"--x",
             _=>"---",
        };
        let mut chars: Vec<char> = per.chars().collect();
        match i {
            0 => { // user
                if special == '4' || special == '5' || special == '6' ||special == '7' {
                    chars[2] = if chars[2] == 'x' { 's' } else { 'S' };
                }
            }
            1 => { // group
                if special == '2' || special == '3' || special == '6' ||special == '7'  {
                    chars[2] = if chars[2] == 'x' { 's' } else { 'S' };
                }
            }
            2 => { // others
                if special == '1' || special == '3' || special == '5' ||special == '7'  {
                    chars[2] = if chars[2] == 'x' { 't' } else { 'T' };
                }
            }
            _ => {}
        }

        res.push_str(&chars.iter().collect::<String>());
    }
    res
}
fn print_entry(
    file_str: &str,
    metadata: &fs::Metadata,
    long_format: bool,
    f_type: bool,
)-> Result<String,String> {
    let mut indicator = "";
    let mut file_type = "-";
    let mut color = "\x1b[0m";
    let reset = "\x1b[0m";
    let item_type = metadata.file_type();

    if item_type.is_symlink() {
        indicator = "@";
        file_type = "l";
        color = "\x1b[1m\x1b[36m";
    } else if item_type.is_dir() {
        indicator = "/";
        file_type = "d";
        color = "\x1b[1m\x1b[34m";
    } else if item_type.is_file() && metadata.permissions().mode() & 0o111 != 0 {
        indicator = "*";
        file_type = "-";
        color = "\x1b[1m\x1b[32m";
    } else if item_type.is_fifo() {
        indicator = "|";
        file_type = "p";
        color = "\x1b[1m\x1b[33m";
    } else if item_type.is_socket() {
        indicator = "=";
        file_type = "s";
        color = "\x1b[1m\x1b[35m";
    } else if item_type.is_block_device() {
        file_type = "b";
        color = "\x1b[1m\x1b[1;33;40m";
    } else if item_type.is_char_device() {
        file_type = "c";
        color = "\x1b[1m\x1b[1;33;40m";
    }

   if long_format {
            let permissions = format_permissions(format!("{:o}", metadata.permissions().mode()));
            let user = match get_user_by_uid(metadata.uid()) {
                Some(u) => u.name().to_string_lossy().into_owned(),
                None => "user".to_string(),
            };
            let group = match get_group_by_gid(metadata.gid()) {
                Some(g) => g.name().to_string_lossy().into_owned(),
                None => "user".to_string(),
            };
            let size = metadata.len();
            let links = metadata.nlink();
            let date = match metadata.modified() {
                Ok(modified) => {
                    let date_time: DateTime<Local> = modified.into();
                    let format_time = date_time.format("%b %e %H:%M").to_string();
                    format_time
                }
                Err(err) => {
                   return  Err(format!("{}", err));
                }
            };
            if f_type {
                return Ok(format!(
                    "{}{} {} {} {} {:>4} {} {}{}{}{}\n",
                    file_type, permissions, links, user, group, size, date,color, file_str,reset,indicator
                ));
            } else {
                return Ok(format!(
                    "{}{} {} {} {} {:>4} {} {}{}{}\n",
                    file_type, permissions, links, user, group, size, date,color, file_str,reset
                ));
            }
        } else {
            if f_type {
                return Ok(format!("{}{}{}{} ", color,file_str,reset,indicator))
            } else {
                return Ok(format!("{}{}{} ", color, file_str,reset))
            }
        }
}
