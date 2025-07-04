pub fn builtin_echo(args: &[&str]) {
    let output = args.join(" ");
    println!("{}", output);
}
