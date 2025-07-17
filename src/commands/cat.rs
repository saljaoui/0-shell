use std::fs::File;
use std::io::{self};

pub fn builtin_cat(args: &[&str]) {
    unsafe {
        signal(2, signal_handler);
    }
    if args.len() == 0 {
        loop {
            let mut input: String = String::new();
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
            if *arg == "-" || *arg == "--" {
                builtin_cat(&[])
            } else {
                match File::open(arg) {
                    Ok(mut file) => {
                        let stdout: io::Stdout = io::stdout();
                        let mut out_handle: io::StdoutLock<'static> = stdout.lock();
                        if let Err(e) = io::copy(&mut file, &mut out_handle) {
                            let error = e.to_string();
                            let formatarg = format_arg(arg);
                            let error_clean = error.split(" (os error").next().unwrap_or(&error);
                            eprintln!("cat: {}: {}", formatarg, error_clean);
                        }
                    }
                    Err(e) => {
                        let error = e.to_string();
                        let error_clean = error.split(" (os error").next().unwrap_or(&error);
                        let formatarg = format_arg(arg);
                        eprintln!("cat: {}: {}", formatarg, error_clean);
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

fn format_arg(arg: &str) -> String {
    if arg.contains("\n"){
        let parts: Vec<&str> = arg.trim().split('\n').collect();
        let quoted_parts: Vec<String> = parts.iter().map(|part| format!("'{}'", part)).collect();
        let mut joined= quoted_parts.join("$'\\n'");
        joined.push_str("$'\\n'");
        return joined
    }else {
        return arg.to_string();
    }
}
