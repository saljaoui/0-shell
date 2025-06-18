use std::io::{self, Write};

fn main() {

    // wlc msg :) 
    println!("===============================");
    println!("🦀  Welcome to 0-shell (Rust)  ");
    println!("===============================");
    println!("Type 'exit' to quit.\n");

    // infinity loop bach n9ra chno dkhol user
    loop {
        // hadi daroriya tlbinha fe project ndiroha
        print!("$ ");
        io::stdout().flush().unwrap(); // bach n9dro ndiro print dyal "$ " f terminal

        // read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input == "" { break; } // Ctrl+D 

        let input = input.trim();

        if input == "exit" { break; } // exit bach ykhrj 

        // temporary behavior
        println!("You typed: {:?}", input);
    }

    // goodbye msg :)
    println!("\n\n===============================");
    println!("👋  Goodbye! Thanks for using 0-shell");
    println!("===============================\n");

}
