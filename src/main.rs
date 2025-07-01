use std::io::{self, Write};
mod dispatch;
mod commands;
fn main() {

    // wlc msg :) 
    println!("===============================");
    println!("🦀  Welcome to our 0-shell (Rust)  ");
    println!("===============================");
    println!("Type 'exit' to quit.\n");

    // infinity loop bach n9ra chno dkhol user
    loop {
        // hadi daroriya tlbinha fe project ndiroha
        print!("\x1b[32m0-shell\x1b[0m:$ ");
        io::stdout().flush().unwrap(); // bach n9dro ndiro print dyal "$ " f terminal

        // read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input == "" { break; } // Ctrl+D 

        let input = input.trim();

        if input == "exit" { break; } // exit bach ykhrj 

        // nsfdo input bach nhandliw 3la 7sab chno dkhl lina user
         if input != "" {
            dispatch::dispatch(input);
         }

    }

    // goodbye msg :)
    println!("\n\n===============================");
    println!("👋  Goodbye! Thanks for using our 0-shell");
    println!("===============================\n");

}
