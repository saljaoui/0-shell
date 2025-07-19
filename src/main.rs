use atty::Stream;
use std::env;
use std::io::{self, Write};
mod commands;
mod dispatch;
use commands::utls_file::{set_current_dir};
// mod signal_handler;

unsafe extern "C" {
    fn signal(signal: i32, handler: extern "C" fn(i32));
}
pub extern "C" fn signal_handler(_signal: i32) {
    println!();
    print!("\x1b[32m0-shell\x1b[0m:$ ");
    io::stdout().flush().unwrap(); // bach n9dro ndiro print dyal "$ " f terminal
}

fn main() {
    if !atty::is(Stream::Stdout) {
        eprintln!("error: broken pipe\nstdout is NOT connected to a terminal.");
        return;
    }
    // wlc msg :)
    println!("===============================");
    println!("🦀  Welcome to our 0-shell (Rust)  ");
    println!("===============================");
    println!("Type 'exit' to quit.\n");
    let result = env::current_dir();

    match result {
        Ok(path) => {
            set_current_dir(path.clone());
        },
        Err(e) => {
            println!("Failed to get current directory: {}", e);
        }
    }

    // signal_handler::setup_signal_handler();
    unsafe {
        signal(2, signal_handler);
    }

    // infinity loop bach n9ra chno dkhol user
    loop {
        // hadi daroriya tlbinha fe project ndiroha
        print!("\x1b[32m0-shell\x1b[0m:$ ");
        match io::stdout().flush() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{e}");
            }
        }; // bach n9dro ndiro print dyal "$ " f terminal

        // read input
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF (Ctrl+D)
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                continue;
            }
        }

        let input = input.trim();

        if input == "exit" {
            break;
        } // exit bach ykhrj

        // nsfdo input bach nhandliw 3la 7sab chno dkhl lina user
        if !input.is_empty() {
            dispatch::dispatch(input);
        }
    }

    // goodbye msg :)
    println!("\n\n===============================");
    println!("👋  Goodbye! Thanks for using our 0-shell");
    println!("===============================\n");
}
