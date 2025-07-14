use chrono::{DateTime, Duration, Local};
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use users::{get_group_by_gid, get_user_by_uid};

#[derive(Default)]
struct LsOptions {
    show_hidden: bool,
    long_format: bool,
    f_type: bool,
}

pub fn builtin_ls(args: &[&str]) {
    let mut options = LsOptions::default();
    let mut paths = Vec::new();
    
    for arg in args {
        match *arg {
            "-l" => options.long_format = true,
            "-a" => options.show_hidden = true,
            "-F" => options.f_type = true,
            "--help" => {
                println!("Usage: ls [OPTION]... [FILE]...\nList information about the FILEs (the current directory by default).\n\nOptions:\n  -l      use a long listing format\n  -a      do not ignore entries starting with .\n  -F      append indicator (one of */=>@|) to entries\n  --help  display this help and exit");
                return;
            }
            "-" => {
                eprintln!("ls: cannot access '-': No such file or directory");
                continue; 
            }
            _ => {
                if arg.starts_with('-') {
                    for a in arg.chars() {
                        match a {
                            'l' => options.long_format = true,
                            'a' => options.show_hidden = true,
                            'F' => options.f_type = true,
                            '-' => {}
                            _ => {
                                eprintln!("ls: invalid option -- '{}'\nTry 'ls --help' for more information.", a);
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
    
    let mut directories = Vec::new();
    let mut files = Vec::new();
   
    for path in paths {
        let metadata = match fs::metadata(path) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("ls: cannot access '{}': {}", path, e);
                continue; 
            }
        };
        
        if metadata.is_dir() {
            directories.push(path)
        } else {
            files.push(path)
        }
    }
    
    let more_paths = directories.len() > 1 || (!files.is_empty() && !directories.is_empty());
    
    for path in files.clone() {
        list_file(path, &options);
    }
    if !more_paths && !files.is_empty() && !directories.is_empty() {
        println!();
    }
    for (i, path) in directories.iter().enumerate() {
        if i > 0 || !files.is_empty() {
            println!(); 
        }
       
        list_directory(path, &options, more_paths);
    }
}

#[derive(Debug)]
struct MaxWidths {
    max_links: usize,
    max_user: usize,
    max_group: usize,
    max_size: usize,
}

fn list_file(path: &str, options: &LsOptions) {
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
        None => path.to_string(), 
    };
    let items = Vec::new();
    let max_widths = calculate_max_widths(&items);

    match print_entry(&file_str, &metadata, options, Some(&p), &max_widths) {
        Ok(r) => print!("{}", &r),
        Err(e) => eprintln!("ls: {}", e),
    }
}

fn list_directory(
    path: &str,
    options: &LsOptions,
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

 

items.sort_by(|a, b| {
    let a_name = a.file_name().to_string_lossy().to_lowercase();
    let b_name = b.file_name().to_string_lossy().to_lowercase();
    
    let a_clean = a_name.strip_prefix('.').unwrap_or(&a_name);
    let b_clean = b_name.strip_prefix('.').unwrap_or(&b_name);
    
    a_clean.cmp(&b_clean)
});
    
    let mut res = String::new();
    let mut total_blocks = 0;
    let mut all_entries = Vec::new();

    if options.show_hidden {
        for special in [".", ".."] {
            let special_path = p.join(special);
            if let Ok(meta) = fs::metadata(&special_path) {
                total_blocks += meta.blocks();
                all_entries.push((special.to_string(), meta, special_path));
            }
        }
    }

    for item in items {
        let file_str = item.file_name().to_string_lossy().to_string();
        if !options.show_hidden && file_str.starts_with('.') {
            continue;
        }

        let metadata = match item.metadata() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("ls: error reading metadata for '{}': {}", file_str, e);
                continue;
            }
        };
        
        total_blocks += metadata.blocks();
        all_entries.push((file_str, metadata, item.path()));
    }

    if options.long_format {
        println!("total {}", total_blocks / 2);
    }

    let max_widths = calculate_max_widths(&all_entries);
    // println!("---{:?}",max_widths);
    for (file_str, metadata, path) in all_entries {
        match print_entry(&file_str, &metadata, options, Some(&path), &max_widths) {
            Ok(r) => res.push_str(&r),
            Err(e) => eprintln!("ls: {}", e),
        }
    }

    if !res.is_empty() {
        println!("{}", res.trim_end());
    }
}

fn format_permissions(mode: u32) -> String {
    let permissions = format!("{:o}", mode);
    let perm_str = &permissions[(permissions.len() - 3)..];
    let special = if permissions.len() >= 4 {
        permissions.chars().nth(permissions.len() - 4).unwrap()
    } else {
        '0'
    };
    
    let mut res = String::new();
    for (i, c) in perm_str.chars().enumerate() {
        let per = match c {
            '7' => "rwx",
            '6' => "rw-",
            '5' => "r-x",
            '4' => "r--",
            '3' => "-wx",
            '2' => "-w-",
            '1' => "--x",
            _ => "---",
        };
        
        let mut chars: Vec<char> = per.chars().collect();
        
        match i {
            0 => { 
                if ['4', '5', '6', '7'].contains(&special) {
                    chars[2] = if chars[2] == 'x' { 's' } else { 'S' };
                }
            }
            1 => { 
                if ['2', '3', '6', '7'].contains(&special) {
                    chars[2] = if chars[2] == 'x' { 's' } else { 'S' };
                }
            }
            2 => { 
                if ['1', '3', '5', '7'].contains(&special) {
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
    options: &LsOptions,
    path: Option<&Path>,
    max_widths: &MaxWidths
) -> Result<String, String> {
    let mut indicator = "";
    let mut file_type = "-";
    let mut color = "\x1b[0m";
    let reset = "\x1b[0m";
    let item_type = metadata.file_type();

    if item_type.is_symlink() {
        indicator = "@";
        file_type = "l";
        color = "\x1b[1;36m"; 
    } else if item_type.is_dir() {
        indicator = "/";
        file_type = "d";
        color = "\x1b[1;34m"; 
    } else if item_type.is_file() && metadata.permissions().mode() & 0o111 != 0 {
        indicator = "*";
        file_type = "-";
        color = "\x1b[1;32m"; 
    } else if item_type.is_fifo() {
        indicator = "|";
        file_type = "p";
        color = "\x1b[1;33m"; 
    } else if item_type.is_socket() {
        indicator = "=";
        file_type = "s";
        color = "\x1b[1;35m";
    } else if item_type.is_block_device() {
        file_type = "b";
        color = "\x1b[1;33;40m"; 
    } else if item_type.is_char_device() {
        file_type = "c";
        color = "\x1b[1;33;40m"; 
    }

    if options.long_format {
        let permissions = format_permissions(metadata.permissions().mode());
        let user = get_user_by_uid(metadata.uid())
            .map(|u| u.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| metadata.uid().to_string());
        let group = get_group_by_gid(metadata.gid())
            .map(|g| g.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| metadata.gid().to_string());
        let size = metadata.len();
        let links = metadata.nlink();
        
        let date = match metadata.modified() {
            Ok(modified) => {
                let mut date_time: DateTime<Local> = modified.into() ;
                date_time = date_time + Duration::hours(1);
                let now = Local::now();

                if now.signed_duration_since(date_time) > Duration::days(180) 
                    || date_time > now {
                    date_time.format("%b %e  %Y").to_string()
                } else {
                    date_time.format("%b %e %H:%M").to_string()
                }
            }
            Err(err) => return Err(format!("error reading date: {}", err)),
        };

        let mut filename_display = format!("{}{}{}", color, file_str, reset);
        
        if item_type.is_symlink() {
            if let Some(p) = path {
                match fs::read_link(p) {
                    Ok(target) => {
                        filename_display = format!("{}{}{} -> {}", 
                            color, file_str, reset, target.display());
                    }
                    Err(_) => {} 
                }
            }
        }

        if options.f_type {
            Ok(format!(
                "{}{} {:>width_links$} {:>width_user$} {:>width_group$} {:>width_size$} {} {}{}\n",
                file_type, 
                permissions, 
                links, 
                user, 
                group, 
                size, 
                date, 
                filename_display, 
                indicator,
                width_links = max_widths.max_links,
                width_user = max_widths.max_user,
                width_group = max_widths.max_group,
                width_size = max_widths.max_size
            ))
        } else {
            Ok(format!(
                "{}{} {:>width_links$} {:>width_user$} {:>width_group$} {:>width_size$} {} {}\n",
                file_type, 
                permissions, 
                links, 
                user, 
                group, 
                size, 
                date, 
                filename_display,
                width_links = max_widths.max_links,
                width_user = max_widths.max_user,
                width_group = max_widths.max_group,
                width_size = max_widths.max_size
            ))
        }
    } else {
        if options.f_type {
            Ok(format!("{}{}{}{} ", color, file_str, reset, indicator))
        } else {
            Ok(format!("{}{}{} ", color, file_str, reset))
        }
    }
}
 use std::fs::Metadata;
// use std::fs::DirEntry;
fn calculate_max_widths(items: &Vec<(String, Metadata, PathBuf)>) -> MaxWidths {
    let mut max_widths = MaxWidths {
        max_links: 0,
        max_user: 0,
        max_group: 0,
        max_size: 0,
    };
    
    for (file_str, metadata, path) in items {
        // let metadata = match item.metadata() {
        //     Ok(m) => m,
        //     Err(e) => {
        //         eprintln!("ls: error reading metadata for : {}", e);
        //         continue;
        //     }
        // };
        max_widths.max_links = max_widths.max_links.max(metadata.nlink().to_string().len());
        max_widths.max_size = max_widths.max_size.max(metadata.len().to_string().len());
        
        let user = get_user_by_uid(metadata.uid())
            .map(|u| u.name().to_string_lossy().len())
            .unwrap_or_else(|| metadata.uid().to_string().len());
        max_widths.max_user = max_widths.max_user.max(user);
        
        let group = get_group_by_gid(metadata.gid())
            .map(|g| g.name().to_string_lossy().len())
            .unwrap_or_else(|| metadata.gid().to_string().len());
        max_widths.max_group = max_widths.max_group.max(group);
    }
    
    max_widths
}