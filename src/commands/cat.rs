use std::fs::File;
use std::io::{self};

fn check(arg: &str) -> bool {
    if arg == "-" || arg == "--" {
        return true;
    }
    false
}

pub fn builtin_cat(args: &[&str]) {
    unsafe {
        signal(2, signal_handler);
    }
    if args.len() == 0 {
        loop {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => print!("{}", input),
                Err(e) => print!("{}", e),
            }
            if input == "" {
                break;
            }
        }
    } else if args.len() >= 1 {
        for arg in args {
            let check = check(arg);
            if !check {
                match File::open(arg) {
                    Ok(mut file) => {
                        let stdout = io::stdout();
                        let mut out_handle = stdout.lock();
                        if let Err(e) = io::copy(&mut file, &mut out_handle) {
                            eprintln!("--cat: {}: {}", arg, e);
                        }
                    }
                    Err(_) => {
                        eprintln!("cat: {}: No such file or directory", arg);
                    }
                }
            } else {
                builtin_cat(&[])
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
