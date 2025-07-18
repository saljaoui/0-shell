use crate::commands::*;
use crate::signal_handler;
// use fork::waitpid;

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
                _ => println!("Command '{}' not found", cmd),
            }
            // println!("+++++++++++++");
            std::process::exit(0);
        }
        Err(_) => println!("Fork failed"),
    }
}
unsafe extern "C" {
    fn signal(signal: i32, handler: extern "C" fn(i32));
}
extern "C" fn signal_handler_exit(_signal: i32) {
    println!();
}

