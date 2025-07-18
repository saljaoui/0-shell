use crate::commands::*;
use std::io::{self, Write};
use crate::signal_handler;

use fork::{fork, Fork};
use shell::*;
// handle 3la 7sap chno dkhl user lina fe input
pub fn dispatch(input: &str) {
    let parts_string: Vec<String> = parse_command(input);
    let parts: Vec<&str> = parts_string.iter().map(|s| s.as_str()).collect();
    if parts.len() == 0 {
        return;
    }
    let cmd = parts[0];
    let args = &parts[1..];
    if cmd == "cd" {
        cd::builtin_cd(args);
        return;
    }
    if check_flag(cmd, args) {
        return;
    }


    match fork() {
        Ok(Fork::Parent(child)) => {
            unsafe {
                signal(2, signal_handler_exit);
            }
            match fork::waitpid(child) {
                Ok(_) => {}
                Err(_) => eprintln!("Failted to wait on child"),
            }
            unsafe {
                signal(2, signal_handler);
            }
        }
        Ok(Fork::Child) => {
            match cmd {
                "echo" => echo::builtin_echo(args),
                "ls" => ls::builtin_ls(args),
                "cat" => cat::builtin_cat(args),
                "mkdir" => mkdir::builtin_mkdir(args),
                "rm" => rm::builtin_rm(args),
                "cp" => cp::builtin_cp(args),
                "pwd" => pwd::builtin_pwd(args),
                "mv" => mv::builtin_mv(args),
                "clear"=>clear_screen(),
                _ => println!("Command '{}' not found", cmd),
            }
            // println!("+++++++++++++");
            std::process::exit(0);
        }
        Err(_) => println!("Fork failed"),
    }
}

fn check_flag(cmd: &str, args: &[&str]) -> bool {
    if matches!(cmd, "echo" | "mkdir" | "cp" | "pwd" | "mv") {
        for arg in args {
            if arg.starts_with('-') {
                println!(
                    "{}: error — you are using an unsupported flag '{}' (don't use flag for this command)",
                    cmd, arg
                );
                return true;
            }
        }
    }
    false
}

extern "C" fn signal_handler_exit(_signal: i32) {
    println!();
}
unsafe extern "C" {
    fn signal(signal: i32, handler: extern "C" fn(i32));
}


fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
     match io::stdout().flush(){
            Ok(_)=>{},
            Err(e)=>{
                eprintln!("{e}");
            }
        };
}