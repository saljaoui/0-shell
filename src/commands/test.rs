let (max_links, max_user, max_group, max_size) = calculate_max_widths(&entries);



/****************** */
use chrono::{DateTime, Duration, Local};
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use users::{get_group_by_gid, get_user_by_uid};

// ============================================================================
// CONFIGURATION & OPTIONS
// ============================================================================

#[derive(Default)]
struct LsOptions {
    show_hidden: bool,
    long_format: bool,
    f_type: bool,
}

// ============================================================================
// MAIN ENTRY POINT
// ============================================================================

pub fn builtin_ls(args: &[&str]) {
    let (options, paths) = match parse_arguments(args) {
        Ok(result) => result,
        Err(()) => return, // Error already printed in parse_arguments
    };

    let (files, directories) = categorize_paths(paths);
    let multiple_sections = directories.len() > 1 || (!files.is_empty() && !directories.is_empty());

    // Print individual files first
    print_files(&files, &options);
    
    // Then print directories
    print_directories(&directories, &options, multiple_sections, !files.is_empty());
}

// ============================================================================
// ARGUMENT PARSING
// ============================================================================

fn parse_arguments(args: &[&str]) -> Result<(LsOptions, Vec<&str>), ()> {
    let mut options = LsOptions::default();
    let mut paths = Vec::new();
    
    for arg in args {
        match *arg {
            "-l" => options.long_format = true,
            "-a" => options.show_hidden = true,
            "-F" => options.f_type = true,
            "--help" => {
                print_help();
                return Err(());
            }
            "-" => {
                eprintln!("ls: cannot access '-': No such file or directory");
                continue; 
            }
            _ => {
                if arg.starts_with('-') {
                    if !parse_combined_options(arg, &mut options)? {
                        return Err(());
                    }
                } else {
                    paths.push(*arg);
                }
            }
        }
    }
    
    if paths.is_empty() {
        paths.push(".");
    }
    
    Ok((options, paths))
}

fn parse_combined_options(arg: &str, options: &mut LsOptions) -> Result<bool, ()> {
    for a in arg.chars() {
        match a {
            'l' => options.long_format = true,
            'a' => options.show_hidden = true,
            'F' => options.f_type = true,
            '-' => {} // Skip leading dashes
            _ => {
                eprintln!("ls: invalid option -- '{}'\nTry 'ls --help' for more information.", a);
                return Err(());
            }
        }
    }
    Ok(true)
}

fn print_help() {
    println!("Usage: ls [OPTION]... [FILE]...");
    println!("List information about the FILEs (the current directory by default).");
    println!();
    println!("Options:");
    println!("  -l      use a long listing format");
    println!("  -a      do not ignore entries starting with .");
    println!("  -F      append indicator (one of */=>@|) to entries");
    println!("  --help  display this help and exit");
}

// ============================================================================
// PATH CATEGORIZATION
// ============================================================================

fn categorize_paths(paths: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
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
            directories.push(path);
        } else {
            files.push(path);
        }
    }
    
    (files, directories)
}

// ============================================================================
// FILE & DIRECTORY PRINTING
// ============================================================================

fn print_files(files: &[&str], options: &LsOptions) {
    for path in files {
        print_single_file(path, options);
    }
}

fn print_directories(directories: &[&str], options: &LsOptions, multiple_sections: bool, has_files: bool) {
    for (i, path) in directories.iter().enumerate() {
        if i > 0 || has_files {
            println!(); 
        }
        print_directory(path, options, multiple_sections);
    }
}

fn print_single_file(path: &str, options: &LsOptions) {
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
    
    match format_entry(&file_str, &metadata, options, Some(&p)) {
        Ok(output) => print!("{}", output),
        Err(e) => eprintln!("ls: {}", e),
    }
}

fn print_directory(path: &str, options: &LsOptions, show_header: bool) {
    let entries = match read_directory_entries(path, options) {
        Ok(entries) => entries,
        Err(()) => return,
    };
    
    if show_header {
        println!("{}:", path);
    }

    let (total_blocks, formatted_entries) = process_directory_entries(entries, options);
    
    if options.long_format {
        println!("total {}", total_blocks / 2);
    }

    print_directory_entries(formatted_entries);
}

// ============================================================================
// DIRECTORY PROCESSING
// ============================================================================

fn read_directory_entries(path: &str, options: &LsOptions) -> Result<Vec<DirectoryEntry>, ()> {
    let p = PathBuf::from(path);
    let dir = match fs::read_dir(&p) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("ls: cannot access '{}': {}", path, e);
            return Err(());
        }
    };

    let mut items: Vec<_> = match dir.collect::<Result<Vec<_>, _>>() {
        Ok(vec) => vec,
        Err(e) => {
            eprintln!("ls: error reading directory '{}': {}", path, e);
            return Err(());
        }
    };

    // Sort items alphabetically (case-insensitive)
    items.sort_by(|a, b| {
        let a_name = a.file_name().to_string_lossy().to_lowercase();
        let b_name = b.file_name().to_string_lossy().to_lowercase();
        a_name.cmp(&b_name)
    });

    let mut entries = Vec::new();

    // Add special entries (. and ..) if showing hidden files
    if options.show_hidden {
        for special in [".", ".."] {
            let special_path = p.join(special);
            if let Ok(metadata) = fs::metadata(&special_path) {
                entries.push(DirectoryEntry {
                    name: special.to_string(),
                    metadata,
                    path: special_path,
                });
            }
        }
    }

    // Add regular entries
    for item in items {
        let file_str = item.file_name().to_string_lossy().to_string();
        
        // Skip hidden files unless explicitly requested
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
        
        entries.push(DirectoryEntry {
            name: file_str,
            metadata,
            path: item.path(),
        });
    }

    Ok(entries)
}

fn process_directory_entries(entries: Vec<DirectoryEntry>, options: &LsOptions) -> (u64, Vec<String>) {
    let mut total_blocks = 0;
    let mut formatted_entries = Vec::new();

    for entry in entries {
        total_blocks += entry.metadata.blocks();
        
        match format_entry(&entry.name, &entry.metadata, options, Some(&entry.path)) {
            Ok(formatted) => formatted_entries.push(formatted),
            Err(e) => eprintln!("ls: {}", e),
        }
    }

    (total_blocks, formatted_entries)
}

fn print_directory_entries(formatted_entries: Vec<String>) {
    let combined_output: String = formatted_entries.join("");
    if !combined_output.is_empty() {
        println!("{}", combined_output.trim_end());
    }
}

// ============================================================================
// ENTRY FORMATTING
// ============================================================================

struct DirectoryEntry {
    name: String,
    metadata: fs::Metadata,
    path: PathBuf,
}

struct FileTypeInfo {
    indicator: &'static str,
    file_type: &'static str,
    color: &'static str,
}

fn get_file_type_info(metadata: &fs::Metadata) -> FileTypeInfo {
    let item_type = metadata.file_type();
    
    if item_type.is_symlink() {
        FileTypeInfo { indicator: "@", file_type: "l", color: "\x1b[1;36m" }
    } else if item_type.is_dir() {
        FileTypeInfo { indicator: "/", file_type: "d", color: "\x1b[1;34m" }
    } else if item_type.is_file() && metadata.permissions().mode() & 0o111 != 0 {
        FileTypeInfo { indicator: "*", file_type: "-", color: "\x1b[1;32m" }
    } else if item_type.is_fifo() {
        FileTypeInfo { indicator: "|", file_type: "p", color: "\x1b[1;33m" }
    } else if item_type.is_socket() {
        FileTypeInfo { indicator: "=", file_type: "s", color: "\x1b[1;35m" }
    } else if item_type.is_block_device() {
        FileTypeInfo { indicator: "", file_type: "b", color: "\x1b[1;33;40m" }
    } else if item_type.is_char_device() {
        FileTypeInfo { indicator: "", file_type: "c", color: "\x1b[1;33;40m" }
    } else {
        FileTypeInfo { indicator: "", file_type: "-", color: "\x1b[0m" }
    }
}

fn format_entry(
    file_str: &str,
    metadata: &fs::Metadata,
    options: &LsOptions,
    path: Option<&Path>,
) -> Result<String, String> {
    let file_info = get_file_type_info(metadata);
    let reset = "\x1b[0m";

    if options.long_format {
        format_long_entry(file_str, metadata, options, path, &file_info, reset)
    } else {
        format_short_entry(file_str, options, &file_info, reset)
    }
}

fn format_long_entry(
    file_str: &str,
    metadata: &fs::Metadata,
    options: &LsOptions,
    path: Option<&Path>,
    file_info: &FileTypeInfo,
    reset: &str,
) -> Result<String, String> {
    let permissions = format_permissions(metadata.permissions().mode());
    let user = get_user_by_uid(metadata.uid())
        .map(|u| u.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| metadata.uid().to_string());
    let group = get_group_by_gid(metadata.gid())
        .map(|g| g.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| metadata.gid().to_string());
    let size = metadata.len();
    let links = metadata.nlink();
    
    let date = format_date(metadata)?;
    let filename_display = format_filename_with_symlink(file_str, metadata, path, file_info.color, reset);

    let format_str = if options.f_type {
        format!(
            "{}{} {:>2} {:>8} {:>8} {:>8} {} {}{}\n",
            file_info.file_type, permissions, links, user, group, size, date, filename_display, file_info.indicator
        )
    } else {
        format!(
            "{}{} {:>2} {:>8} {:>8} {:>8} {} {}\n",
            file_info.file_type, permissions, links, user, group, size, date, filename_display
        )
    };

    Ok(format_str)
}

fn format_short_entry(
    file_str: &str,
    options: &LsOptions,
    file_info: &FileTypeInfo,
    reset: &str,
) -> Result<String, String> {
    let format_str = if options.f_type {
        format!("{}{}{}{} ", file_info.color, file_str, reset, file_info.indicator)
    } else {
        format!("{}{}{} ", file_info.color, file_str, reset)
    };

    Ok(format_str)
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

fn format_date(metadata: &fs::Metadata) -> Result<String, String> {
    let modified = metadata.modified()
        .map_err(|err| format!("error reading date: {}", err))?;
    
    let date_time: DateTime<Local> = modified.into();
    let now = Local::now();

    let formatted = if now.signed_duration_since(date_time) > Duration::days(180) 
        || date_time > now {
        date_time.format("%b %e  %Y").to_string()
    } else {
        date_time.format("%b %e %H:%M").to_string()
    };

    Ok(formatted)
}

fn format_filename_with_symlink(
    file_str: &str,
    metadata: &fs::Metadata,
    path: Option<&Path>,
    color: &str,
    reset: &str,
) -> String {
    let mut filename_display = format!("{}{}{}", color, file_str, reset);
    
    if metadata.file_type().is_symlink() {
        if let Some(p) = path {
            if let Ok(target) = fs::read_link(p) {
                filename_display = format!("{}{}{} -> {}", 
                    color, file_str, reset, target.display());
            }
        }
    }
    
    filename_display
}

fn format_permissions(mode: u32) -> String {
    let permissions = format!("{:o}", mode);
    let perm_str = &permissions[(permissions.len() - 3)..];
    let special = if permissions.len() >= 4 {
        permissions.chars().nth(permissions.len() - 4).unwrap()
    } else {
        '0'
    };
    
    let mut result = String::new();
    
    for (i, c) in perm_str.chars().enumerate() {
        let base_perms = match c {
            '7' => "rwx", '6' => "rw-", '5' => "r-x", '4' => "r--",
            '3' => "-wx", '2' => "-w-", '1' => "--x", _ => "---",
        };
        
        let mut chars: Vec<char> = base_perms.chars().collect();
        
        // Apply special permissions (setuid, setgid, sticky bit)
        match i {
            0 if ['4', '5', '6', '7'].contains(&special) => {
                chars[2] = if chars[2] == 'x' { 's' } else { 'S' };
            }
            1 if ['2', '3', '6', '7'].contains(&special) => {
                chars[2] = if chars[2] == 'x' { 's' } else { 'S' };
            }
            2 if ['1', '3', '5', '7'].contains(&special) => {
                chars[2] = if chars[2] == 'x' { 't' } else { 'T' };
            }
            _ => {}
        }

        result.push_str(&chars.iter().collect::<String>());
    }
    
    result
}