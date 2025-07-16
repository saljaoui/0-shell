use std::fs;
use std::io::Write;
use std::io::{self};

pub fn builtin_cat(args: &[&str]) {
    unsafe {
        signal(2, signal_handler);
    }
    if args.len() == 0 {
        loop {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(output) => print!("{}", output),
                Err(e) => print!("{}", e),
            }
            if input == "" {
                break;
            }
        }
    } else if args.len() >= 1 {
        for arg in args {
            match fs::read_to_string(arg) {
                Ok(text) => {
                    print!("{}", text);
                }
                Err(_) => {
                    match fs::read(arg) {
                        Ok(bytes) => {
                            io::stdout().write_all(&bytes).unwrap();
                        }
                        Err(_) => {
                            eprintln!("cat: {}: No such file or directory", arg);
                        }
                    }
                }
            }
        }
    }
}

unsafe extern "C" {
    fn signal(signal: i32, handler: extern "C" fn(i32));
}
extern "C" fn signal_handler(_signal: i32) {
    std::process::exit(0);
}
