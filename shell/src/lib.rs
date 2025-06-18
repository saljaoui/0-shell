
// handle 3la 7sap chno dkhl user lina fe input 
pub fn dispatch(input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let cmd = parts[0];
    let args = &parts[1..];

    match cmd {
        "echo" => builtin_echo(args),
        // "cd"   => builtin_cd(args),
        // add dakchi li ba9i hena ...

        _      => println!("Command '{}' not found", cmd),
    }

}

/// Builtin: echo
fn builtin_echo(args: &[&str]) {
    let output = args.join(" ");
    println!("{}", output);
}

/// Builtin: cd
// fn builtin_cd(args: &[&str]) {
//     println!("{:?}", args);
// }
