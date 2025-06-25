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
    let more_paths = paths.len() > 1;
    for path in paths {
        list_directory(path, show_hidden, long_format, f_type, more_paths);
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
    let dir = fs::read_dir(p);
    let p = PathBuf::from(path);
    let dir = fs::read_dir(p);

    let mut items: Vec<_> = match dir {
        Ok(d) => match d.collect::<Result<Vec<_>, _>>() {
            Ok(vec) => vec,
            Err(e) => {
                eprintln!("ls: error reading directory entries: {}", e);
                return;
            }
        },
        Err(_) => {
            // if Path::new(path).exists() {
            //     println!("{}", path);
            // } else {
            eprintln!("ls: cannot access '{}': No such file or directory", path);
            // }
            return;
        }
    };
    let mut res = String::new();
    if more_paths {
        res.push_str(&format!("{}:\n", path))
    }

    items.sort_by_key(|entry| entry.file_name());
    for item in items {
        let file = item.file_name();
        let file_str = file.to_string_lossy();
        if !show_hidden && file_str.starts_with('.') {
            continue;
        }
        let metadata = match item.metadata() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("ls: cannot get metadata for {:?}: {}", item.path(), e);
                continue;
            }
        };
        /***********************************************************/
        // let file_type = if metadata.is_dir() { "d" } else { "-" };
        let mut indicator = "";
        let mut file_type = "";
        let item_type = metadata.file_type();
        
            if item_type.is_symlink() {
                indicator = "@";
                file_type = "l";
            } else if item_type.is_dir() {
                indicator = "/";
                file_type = "d";
            } else if item_type.is_file() && (metadata.permissions().mode() & 0o111 != 0) {
                indicator = "*";
                file_type = "-";
            } else if item_type.is_fifo() {
                indicator = "|";
                file_type = "p";
            } else if item_type.is_socket() {
                indicator = "=";
                file_type = "s";
            } else if item_type.is_block_device(){
                file_type = "b";
            } else if item_type.is_char_device() {
                file_type = "c";
            } else {
                file_type = "-";
            }
        //////////////////////*************************************/
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
                    eprintln!("{}", err);
                    return;
                }
            };
            if f_type {
                res.push_str(&format!(
                    "{}{} {} {} {} {:>4} {} \x1b[34m{}{}\x1b[0m\n",
                    file_type, permissions, links, user, group, size, date, file_str,indicator
                ));
            } else if metadata.is_dir() {
                res.push_str(&format!(
                    "{}{} {} {} {} {:>4} {} \x1b[34m{}\x1b[0m\n",
                    file_type, permissions, links, user, group, size, date, file_str
                ));
            } else {
                res.push_str(&format!(
                    "{}{} {} {} {} {:>4} {} {}\n",
                    file_type, permissions, links, user, group, size, date, file_str
                ));
            }
        } else {
            if f_type && metadata.is_dir() {
                res.push_str(&format!("\x1b[34m{}{}\x1b[0m ", file_str,indicator))
            } else {
                if metadata.is_dir() {
                    res.push_str(&format!("\x1b[34m{}\x1b[0m ", file_str))
                } else {
                    res.push_str(&format!("{} ", file_str))
                }
            }
        }
    }
    println!("{}", res.trim())
}

fn format_permissions(p: String) -> String {
    let permissions = &p[(p.len() - 3)..];
    let mut res = String::new();
    for c in permissions.chars() {
        match c {
            '7' => res.push_str("rwx"),
            '6' => res.push_str("rw-"),
            '5' => res.push_str("r-x"),
            '4' => res.push_str("r--"),
            '3' => res.push_str("-wx"),
            '2' => res.push_str("-w-"),
            '1' => res.push_str("--x"),
            '0' => res.push_str("---"),
            _ => {
                println!("----{:?} >>{}", c, permissions);
            }
        }
    }
    res
}
