use crate::commands::{ls, echo};

// handle 3la 7sap chno dkhl user lina fe input 
pub fn dispatch(input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let cmd = parts[0];
    let args = &parts[1..];

    match cmd {
        "echo" => echo::builtin_echo(args),
        "ls" => ls::builtin_ls(args),
        // "cd"   => builtin_cd(args),
        // add dakchi li ba9i hena ...

        _      => println!("Command '{}' not found", cmd),
    }

}
